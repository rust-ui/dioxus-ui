+++
title = "Date Picker"
description = "Calendar and date selection components for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoDatePicker />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::date_picker::{DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerTitle};
```

```rust
rsx! {
    DemoDatePicker {}
}
```

## Examples

### Basic Date Picker

Single-date calendar picker.

<DemoDatePicker />

### Dropdown

Date picker presented inside a dropdown-like surface.

<DemoDatePickerDropdown />

### Dual Calendar

Two-month range picker layout for comparing and selecting broader date spans.

<DemoDatePickerDual />

### Full Dual Picker

Extended dual-month date picker state demo.

<DemoDatePickerDualFull />

### Presets

Date picker with shortcut buttons such as today, tomorrow, or a week ahead.

<DemoDatePickerPresets />

### Time

Date selection combined with time-oriented workflow.

<DemoDatePickerTime />

### Booked Days

Date picker showing unavailable or pre-booked dates.

<DemoDatePickerBooked />

### Week Numbers

Calendar layout including week number information.

<DemoDatePickerWeekNumbers />

## See Also

- [Popover](/components/popover)
- [Input](/components/input)
