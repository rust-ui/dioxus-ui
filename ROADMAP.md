# Dioxus UI — Roadmap

Porting components from [rust-ui.com](https://www.rust-ui.com) (Leptos) to Dioxus.

Each item requires: `src/ui/<component>.rs`, `src/registry/<component>.rs`, `public/docs/<component>.md`, demos in `src/demos/`.

---

## Inputs & Forms

- [x] Button (`button`, `button_group`)
- [x] Input
- [ ] Textarea
- [ ] Select
- [ ] Checkbox
- [ ] Switch
- [ ] Slider
- [ ] Radio Button (`radio-button`, `radio-button-group`)
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
- [ ] Avatar
- [ ] Table
- [ ] Alert
- [ ] Callout
- [ ] Kbd
- [ ] Chips
- [ ] Status
- [ ] Empty

## Feedback / Loading

- [x] Skeleton
- [x] Spinner
- [ ] Progress
- [ ] Shimmer
- [ ] Animate
- [ ] Animate Group
- [ ] Toast
- [ ] Sonner

## Navigation

- [ ] Breadcrumb
- [ ] Tabs
- [ ] Pagination
- [ ] Bottom Nav
- [ ] Navigation Menu
- [ ] Menubar

## Overlay

- [ ] Dialog
- [ ] Alert Dialog
- [ ] Drawer
- [ ] Sheet
- [ ] Popover
- [ ] Hover Card
- [ ] Tooltip
- [ ] Dropdown Menu
- [ ] Context Menu
- [ ] Command

## Layout

- [ ] Accordion
- [ ] Collapsible
- [ ] Carousel
- [ ] Card Carousel
- [ ] Scroll Area
- [ ] Aspect Ratio
- [ ] Item

## Utilities

- [ ] Theme Toggle
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
