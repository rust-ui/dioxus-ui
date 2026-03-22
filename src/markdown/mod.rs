pub mod converter;

use pulldown_cmark::{html, Options, Parser};

pub struct Frontmatter {
    pub title: String,
    pub description: String,
}

/// Parse `---\nkey: value\n---\nbody` into (Frontmatter, body_markdown).
pub fn parse_md(raw: &str) -> (Frontmatter, &str) {
    let mut title = String::new();
    let mut description = String::new();

    if let Some(rest) = raw.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            let front = &rest[..end];
            let body = &rest[end + 5..]; // skip "\n---\n"

            for line in front.lines() {
                if let Some(v) = line.strip_prefix("title:") {
                    title = v.trim().to_string();
                } else if let Some(v) = line.strip_prefix("description:") {
                    description = v.trim().to_string();
                }
            }

            return (Frontmatter { title, description }, body);
        }
    }

    (Frontmatter { title, description }, raw)
}

pub fn markdown_to_html(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, opts);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
