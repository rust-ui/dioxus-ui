# Dioxus UI — Roadmap

Porting components from [rust-ui.com](https://www.rust-ui.com) (Leptos) to Dioxus.

Each item requires: `src/ui/<component>.rs`, `src/registry/<component>.rs`, `public/docs/<component>.md`, demos in `src/demos/`.

---

## Inputs & Forms

- [x] Button (`button`, `button_group`)
- [x] Input
- [x] Textarea
- [ ] Select (custom dropdown)
- [x] Select Native
- [x] Checkbox
- [x] Switch
- [x] Slider
- [x] Radio Button (`radio-group`)
- [ ] Combobox
- [ ] Multi Select
- [ ] Date Picker
- [ ] Input Group
- [ ] Input OTP
- [ ] Input Phone
- [ ] Field
- [ ] Form
- [ ] Auto Form

## Display

- [x] Badge
- [x] Card
- [x] Separator
- [x] Avatar
- [x] Table
- [x] Alert
- [ ] Callout
- [x] Kbd
- [x] Label
- [ ] Chips
- [ ] Status
- [ ] Empty

## Feedback / Loading

- [x] Skeleton
- [x] Spinner
- [x] Progress
- [ ] Shimmer
- [ ] Animate
- [ ] Animate Group
- [ ] Toast
- [ ] Sonner

## Navigation

- [x] Breadcrumb
- [x] Tabs
- [ ] Pagination
- [ ] Bottom Nav
- [ ] Navigation Menu
- [ ] Menubar

## Overlay

- [ ] Dialog
- [x] Alert Dialog
- [ ] Drawer
- [ ] Sheet
- [ ] Popover
- [ ] Hover Card
- [x] Tooltip
- [ ] Dropdown Menu
- [ ] Context Menu
- [ ] Command

## Layout

- [x] Accordion
- [x] Collapsible
- [ ] Carousel
- [ ] Card Carousel
- [ ] Scroll Area
- [ ] Aspect Ratio
- [ ] Item

## Utilities

- [ ] Theme Toggle (component exists, no doc page yet)
- [x] Toggle
- [ ] Toggle Group
- [ ] Pressable
- [ ] Marquee
- [ ] Drag and Drop
- [ ] Dropzone
- [ ] Direction Provider

---

## Demos checklist

For each component, demos to port:

### Button
- [x] `DemoButton`
- [x] `DemoButtonVariants`
- [x] `DemoButtonSizes`
- [x] `DemoButtonDisabled`
- [x] `DemoButtonStateful`
- [x] `DemoButtonReactive`
- [x] `DemoButtonOverride`
- [x] `DemoButtonHref`
- [x] `DemoButtonGroup`
- [x] `DemoButtonGroupIcon`

### Badge
- [x] `DemoBadge`
- [x] `DemoBadgeVariants`
- [x] `DemoBadgeColors`
- [x] `DemoBadgeCustom`

### Card
- [x] `DemoCard`
- [x] `DemoCardAction`
- [x] `DemoCardGroup`
- [x] `DemoCardReverse`
- [x] `DemoCardSm`

### Input
- [x] `DemoInput`
- [x] `DemoInputCopy`

### Separator
- [x] `DemoSeparator`

### Skeleton
- [x] `DemoSkeleton`
- [x] `DemoSkeletonText`
- [x] `DemoSkeletonAvatar`
- [x] `DemoSkeletonForm`
- [x] `DemoSkeletonImage`
- [x] `DemoSkeletonTable`

### Spinner
- [x] `DemoSpinner`
- [x] `DemoSpinnerButton`

### Alert
- [x] `DemoAlert`
- [x] `DemoAlertDestructive`

### Avatar
- [x] `DemoAvatar`
- [x] `DemoAvatarFallback`
- [x] `DemoAvatarSizes`

### Checkbox
- [x] `DemoCheckbox`
- [x] `DemoCheckboxLabeled`
- [x] `DemoCheckboxDisabled`

### Kbd
- [x] `DemoKbd`
- [x] `DemoKbdCombination`

### Label
- [x] `DemoLabel`
- [x] `DemoLabelInput`

### Progress
- [x] `DemoProgress`
- [x] `DemoProgressAnimated`

### Switch
- [x] `DemoSwitch`
- [x] `DemoSwitchLabeled`

### Tabs
- [x] `DemoTabs`

### Textarea
- [x] `DemoTextarea`
- [x] `DemoTextareaDisabled`

### Accordion
- [x] `DemoAccordion`
- [x] `DemoAccordionBordered`

### Alert Dialog
- [x] `DemoAlertDialog`

### Breadcrumb
- [x] `DemoBreadcrumb`

### Collapsible
- [x] `DemoCollapsible`

### Radio Group
- [x] `DemoRadioGroup`

### Select Native
- [x] `DemoSelectNative`
- [x] `DemoSelectNativeGroup`

### Slider
- [x] `DemoSlider`
- [x] `DemoSliderDisabled`

### Table
- [x] `DemoTable`

### Toggle
- [x] `DemoToggle`

### Tooltip
- [x] `DemoTooltip`
