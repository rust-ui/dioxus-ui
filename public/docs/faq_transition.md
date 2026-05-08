+++
title = "FAQ Transition"
description = "Accordion-style FAQ sections using CSS checkbox peer transitions."
+++

<DemoFaqTransition />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::faq_transition::{Faq, FaqSection, FaqInput, FaqLabel, FaqContent, FaqDescription};
```

```rust
rsx! {
    Faq {
        FaqSection {
            FaqInput { id: "faq1" }
            FaqLabel { html_for: "faq1",
                span { "Question?" }
                ChevronDown { class: "size-4" }
            }
            FaqContent {
                FaqDescription { "Answer goes here." }
            }
        }
    }
}
```

## See Also

- [Collapsible](/components/collapsible)
- [Accordion](/components/accordion)
