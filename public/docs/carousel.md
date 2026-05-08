+++
title = "Carousel"
description = "A scrollable carousel with keyboard navigation, looping, and orientation support."
+++

<DemoCarousel />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext};
```

```rust
rsx! {
    Carousel { looping: true,
        CarouselContent {
            CarouselItem {
                div { class: "p-1",
                    Card { "Slide 1" }
                }
            }
            CarouselItem {
                div { class: "p-1",
                    Card { "Slide 2" }
                }
            }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
}
```

## See Also

- [Card Carousel](/components/card-carousel)
- [Scroll Area](/components/scroll-area)
