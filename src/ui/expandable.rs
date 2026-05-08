use dioxus::prelude::*;
use icons::X;
use tw_merge::tw_merge;

#[derive(Clone, Copy)]
struct ExpandableState(Signal<bool>);

#[component]
pub fn ExpandableTrigger(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let mut expanded = use_signal(|| false);
    provide_context(ExpandableState(expanded));

    let base = tw_merge!(
        "overflow-hidden relative rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 h-[32px] w-[162px]",
        class.as_deref().unwrap_or("")
    );
    let class_str = use_memo(move || if expanded() { format!("{base} expand") } else { base.clone() });

    rsx! {
        div {
            class: "{class_str}",
            onclick: move |_| expanded.set(true),
            {children}
        }
    }
}

#[component]
pub fn ExpandableTransition(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "absolute transition-opacity duration-200 delay-100",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            class: "{c}",
            style: "transition-timing-function: cubic-bezier(0.0, 0.0, 0.2, 1);",
            div { class: "flex flex-row transition-transform duration-300 origin-top-left ease-[cubic-bezier(0.4,0.0,0.2,1)]",
                {children}
            }
        }
    }
}

#[component]
pub fn ExpandableContent(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let mut ctx = use_context::<ExpandableState>().0;
    let c = tw_merge!(
        "relative w-full bg-muted h-full scale-[0.55] origin-top-left transition-transform duration-300 ease-[cubic-bezier(0.4,0.0,0.2,1)]",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "absolute w-full h-full opacity-0 transition-opacity duration-100 ease-[cubic-bezier(0.4,0.0,1,1)]",
            div { class: "{c}",
                button {
                    r#type: "button",
                    class: "flex absolute top-1 right-1 justify-center items-center",
                    onclick: move |_| ctx.set(false),
                    X { class: "size-6" }
                }
                {children}
            }
        }
    }
}
