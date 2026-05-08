+++
title = "Card Carousel"
description = "A compact image carousel with overlay nav and indicators."
+++

<DemoCardCarousel />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::card_carousel::{CardCarousel, CardCarouselTrack, CardCarouselSlide, CardCarouselImage, CardCarouselOverlay, CardCarouselNav, CardCarouselNavButton, CardCarouselIndicators, CardCarouselIndicator};
```

```rust
rsx! {
    CardCarousel {
        CardCarouselOverlay {
            CardCarouselNav {
                CardCarouselNavButton { ChevronLeft {} }
                CardCarouselNavButton { ChevronRight {} }
            }
            CardCarouselIndicators {
                CardCarouselIndicator { aria_current: true }
                CardCarouselIndicator {}
            }
        }
        CardCarouselTrack {
            CardCarouselSlide {
                CardCarouselImage { src: "/img1.jpg", alt: "Slide 1" }
            }
        }
    }
}
```

## See Also

- [Carousel](/components/carousel)
- [Card](/components/card)
