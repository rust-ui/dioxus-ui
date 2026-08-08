+++
title = "Date Picker"
description = "Calendar and date selection components for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDatePicker />

## Installation

<StaticInstallDatePicker />

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

<StaticDatePicker />

### Dropdown

Date picker presented inside a dropdown-like surface.

<StaticDatePickerDropdown />

### Dual Calendar

Two-month range picker layout for comparing and selecting broader date spans.

<StaticDatePickerDual />

### Full Dual Picker

Extended dual-month date picker state demo.

<StaticDatePickerDualFull />

### Presets

Date picker with shortcut buttons such as today, tomorrow, or a week ahead.

<StaticDatePickerPresets />

### Time

Date selection combined with time-oriented workflow.

<StaticDatePickerTime />

### Booked Days

Date picker showing unavailable or pre-booked dates.

<StaticDatePickerBooked />

### Week Numbers

Calendar layout including week number information.

<StaticDatePickerWeekNumbers />

## See Also

- [Popover](/components/popover)
- [Input](/components/input)
