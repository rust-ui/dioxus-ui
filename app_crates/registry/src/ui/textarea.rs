use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Textarea(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] placeholder: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = 4)] rows: i64,
) -> Element {
    let class = tw_merge!(
        "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        textarea {
            class: "{class}",
            placeholder: placeholder.as_deref().unwrap_or(""),
            disabled,
            rows,
        }
    }
}
