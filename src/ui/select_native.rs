use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn SelectNative(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(optional)] disabled: bool,
    #[props(optional)] onchange: Option<EventHandler<FormEvent>>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "peer inline-flex w-full cursor-pointer appearance-none items-center rounded-lg h-9 pe-8 ps-3 border border-input bg-background shadow-sm text-sm text-foreground transition-shadow focus-visible:border-ring focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/20 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "relative",
            select {
                class: "{merged}",
                id: id.as_deref().unwrap_or(""),
                disabled: disabled,
                onchange: move |e| {
                    if let Some(handler) = &onchange {
                        handler.call(e);
                    }
                },
                {children}
            }
            span { class: "pointer-events-none absolute inset-y-0 end-0 flex h-full w-9 items-center justify-center text-muted-foreground/80 peer-disabled:opacity-50",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "m6 9 6 6 6-6" }
                }
            }
        }
    }
}
