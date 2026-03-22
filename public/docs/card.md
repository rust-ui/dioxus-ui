---
title: Card
description: Displays a card with header, content, and footer sections.
---

## Demo

<DemoCard />

## Action

<DemoCardAction />

## Group

<DemoCardGroup />

## Small

<DemoCardSm />

## Reverse

<DemoCardReverse />

## Props

### Card

| Prop    | Type             | Default | Description             |
|---------|------------------|---------|-------------------------|
| `class` | `Option<String>` | `None`  | Extra Tailwind classes  |

### CardHeader / CardContent / CardFooter

| Prop    | Type             | Default | Description             |
|---------|------------------|---------|-------------------------|
| `class` | `Option<String>` | `None`  | Extra Tailwind classes  |

## Usage

```rust
rsx! {
    Card {
        CardHeader {
            CardTitle { "Title" }
        }
        CardContent {
            CardDescription { "Description" }
        }
        CardFooter {
            Button { "Action" }
        }
    }
}
```
