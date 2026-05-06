/// Returns highlighted HTML spans (inline styles, no `<pre>` wrapper).
/// Server: syntect. WASM/client: HTML-escaped plain text.
pub fn highlight_code(code: &str, language: Option<&str>) -> String {
    #[cfg(feature = "server")]
    {
        server::highlight(code, language)
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = language;
        plain_escape(code)
    }
}

#[cfg(not(feature = "server"))]
fn plain_escape(code: &str) -> String {
    code.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(feature = "server")]
mod server {
    use std::sync::OnceLock;

    use syntect::{
        easy::HighlightLines,
        highlighting::ThemeSet,
        html::{IncludeBackground, styled_line_to_highlighted_html},
        parsing::SyntaxSet,
        util::LinesWithEndings,
    };

    const HIGHLIGHT_THEME: &str = "base16-ocean.dark";

    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    fn syntax_set() -> &'static SyntaxSet {
        SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
    }

    fn theme_set() -> &'static ThemeSet {
        THEME_SET.get_or_init(ThemeSet::load_defaults)
    }

    pub fn highlight(code: &str, language: Option<&str>) -> String {
        let ss = syntax_set();
        let ts = theme_set();

        let theme = ts
            .themes
            .get(HIGHLIGHT_THEME)
            .or_else(|| ts.themes.values().next())
            .expect("no syntect theme available");

        let lang = language.unwrap_or("plain");

        let syntax = match lang {
            "rust" => ss.find_syntax_by_name("Rust"),
            "bash" | "sh" => ss
                .find_syntax_by_name("Bourne Again Shell (bash)")
                .or_else(|| ss.find_syntax_by_extension("sh")),
            "toml" => ss.find_syntax_by_name("TOML"),
            _ => ss.find_syntax_by_extension(lang),
        }
        .unwrap_or_else(|| ss.find_syntax_plain_text());

        let mut hl = HighlightLines::new(syntax, theme);
        let mut out = String::new();

        for line in LinesWithEndings::from(code) {
            let ranges = hl.highlight_line(line, ss).unwrap_or_default();
            let line_html = styled_line_to_highlighted_html(&ranges, IncludeBackground::No)
                .unwrap_or_else(|_| html_escape::encode_text(line).to_string());
            out.push_str(&line_html);
        }

        out
    }
}
