---
title: Card
description: Displays a card with header, content, and footer sections.
---

## Demo

<demo-card></demo-card>

## Variants

<demo-card-action></demo-card-action>

<demo-card-group></demo-card-group>

<demo-card-sm></demo-card-sm>

<demo-card-reverse></demo-card-reverse>

## Props

### Card

| Prop    | Type             | Default | Description                  |
|---------|------------------|---------|------------------------------|
| `class` | `Option<String>` | `None`  | Extra Tailwind classes        |

### CardHeader / CardContent / CardFooter

| Prop    | Type             | Default | Description                  |
|---------|------------------|---------|------------------------------|
| `class` | `Option<String>` | `None`  | Extra Tailwind classes        |

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
