# Dioxus UI — Roadmap

Porting components from [rust-ui.com](https://www.rust-ui.com) (Leptos) to Dioxus.

Each item requires: `src/ui/<component>.rs`, `src/registry/<component>.rs`, `public/docs/<component>.md`, demos in `src/demos/`.

> **Source of truth: RUST-UI**. Demos and md files must match RUST-UI exactly.

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
- [x] Radio Button (`radio_group`)
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
- [x] Chips
- [x] Status
- [x] Empty

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
- [x] Pagination
- [ ] Bottom Nav
- [ ] Navigation Menu
- [ ] Menubar

## Overlay

- [x] Dialog
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

- [x] Theme Toggle
- [ ] Toggle (pending — no RUST-UI equivalent demo)
- [x] Toggle Group
- [ ] Pressable
- [ ] Marquee
- [ ] Drag and Drop
- [ ] Dropzone
- [ ] Direction Provider

---

## Demos checklist

Matches RUST-UI demos exactly. Skipped demos require components not yet ported (Field, InputGroup, InputOTP, InputPhone, Select, ButtonAction, Carousel, DropdownMenu).

### Accordion
- [x] `DemoAccordion`
- [x] `DemoAccordionBordered`
- [x] `DemoAccordionCard`
- [x] `DemoAccordionIcons`
- [x] `DemoAccordionRtl`

### Alert
- [x] `DemoAlert`
- [x] `DemoAlertColors`
- [x] `DemoAlertRtl`

### Alert Dialog
- [x] `DemoAlertDialog`
- [x] `DemoAlertDialogMedia`
- [x] `DemoAlertDialogRtl`
- [x] `DemoAlertDialogSmall`
- [x] `DemoAlertDialogSmallMedia`

### Avatar
- [x] `DemoAvatar`
- [x] `DemoAvatarBadge`
- [x] `DemoAvatarBadgeIcon`
- [x] `DemoAvatarGroup`
- [x] `DemoAvatarGroupCount`
- [x] `DemoAvatarGroupCountIcon`
- [x] `DemoAvatarRtl`
- [x] `DemoAvatarSize`

### Badge
- [x] `DemoBadge`
- [x] `DemoBadgeVariants`
- [x] `DemoBadgeColors`
- [x] `DemoBadgeCustom`
- [x] `DemoBadgeRtl`

### Breadcrumb
- [x] `DemoBreadcrumb`
- [x] `DemoBreadcrumbRtl`
- [ ] `DemoBreadcrumbDropdown` (needs DropdownMenu)

### Button
- [x] `DemoButton`
- [x] `DemoButtonVariants`
- [x] `DemoButtonSizes`
- [x] `DemoButtonDisabled`
- [x] `DemoButtonStateful`
- [x] `DemoButtonReactive`
- [x] `DemoButtonOverride`
- [x] `DemoButtonHref`
- [x] `DemoButtonRtl`
- [ ] `DemoButtonAction` (needs ButtonAction component)
- [ ] `DemoButtonWithClx` (needs leptos_ui::clx)

### Button Group
- [x] `DemoButtonGroup`
- [x] `DemoButtonGroupIcon`
- [x] `DemoButtonGroupInput`
- [x] `DemoButtonGroupSeparator`
- [x] `DemoButtonGroupSizes`
- [x] `DemoButtonGroupRtl`
- [ ] `DemoButtonGroupSelect` (needs Select component)

### Card
- [x] `DemoCard`
- [x] `DemoCardAction`
- [x] `DemoCardGroup`
- [x] `DemoCardReverse`
- [x] `DemoCardRtl`
- [x] `DemoCardSm`
- [ ] `DemoCardCarousel` (needs Carousel)

### Checkbox
- [x] `DemoCheckbox`
- [x] `DemoCheckboxRtl`

### Chips
- [x] `DemoChips`

### Collapsible
- [x] `DemoCollapsible`
- [x] `DemoCollapsibleBasic`
- [x] `DemoCollapsibleFileTree`
- [x] `DemoCollapsibleRtl`
- [x] `DemoCollapsibleSettings`

### Dialog
- [x] `DemoDialog`
- [x] `DemoDialogRtl`
- [x] `DemoDialogScrollable`
- [x] `DemoDialogStickyFooter`

### Empty
- [x] `DemoEmpty`
- [x] `DemoEmptyAvatarGroup`
- [x] `DemoEmptyBackground`
- [x] `DemoEmptyCard`
- [x] `DemoEmptyMuted`
- [x] `DemoEmptyOutline`
- [x] `DemoEmptyRtl`
- [ ] `DemoEmptyInputGroup` (needs InputGroup)

### Input
- [x] `DemoInput`
- [x] `DemoInputCopy`
- [x] `DemoInputRtl`
- [ ] `DemoInputGroup*` (needs InputGroup component)
- [ ] `DemoInputOtp*` (needs InputOTP component)
- [ ] `DemoInputPhone*` (needs InputPhone component)

### Kbd
- [x] `DemoKbd`
- [x] `DemoKbdButton`
- [x] `DemoKbdRtl`
- [ ] `DemoKbdInputGroup` (needs InputGroup)

### Label
- [x] `DemoLabel`
- [x] `DemoLabelRtl`

### Pagination
- [x] `DemoPagination`
- [x] `DemoPaginationRtl`

### Progress
- [x] `DemoProgress`
- [x] `DemoProgressControlled`
- [x] `DemoProgressRtl`
- [ ] `DemoProgressLabel` (needs Field component)

### Radio Button
- [x] `DemoRadioButton`
- [x] `DemoRadioButtonCustom`
- [ ] `DemoRadioButtonGroup` (needs RadioButtonGroup component)
- [ ] `DemoRadioButtonGroupRtl` (needs RadioButtonGroup component)
- [ ] `DemoRadioChoiceCard` (needs Field component)

### Select Native
- [x] `DemoSelectNativeAutoWidth`
- [x] `DemoSelectNativeError`
- [x] `DemoSelectNativeGroup`
- [x] `DemoSelectNativeOverlappingLabel`
- [x] `DemoSelectNativeTimezone`

### Separator
- [x] `DemoSeparator`
- [x] `DemoSeparatorRtl`

### Skeleton
- [x] `DemoSkeleton`
- [x] `DemoSkeletonAvatar`
- [x] `DemoSkeletonForm`
- [x] `DemoSkeletonImage`
- [x] `DemoSkeletonRtl`
- [x] `DemoSkeletonTable`
- [x] `DemoSkeletonText`

### Slider
- [x] `DemoSlider`
- [x] `DemoSliderControlled`
- [x] `DemoSliderFlat`
- [x] `DemoSliderMultiple`
- [x] `DemoSliderRtl`
- [x] `DemoSliderVertical`

### Spinner
- [x] `DemoSpinner`
- [x] `DemoSpinnerButton`
- [x] `DemoSpinnerRtl`

### Status
- [x] `DemoStatus`
- [x] `DemoStatusVariants`

### Switch
- [x] `DemoSwitch`
- [x] `DemoSwitchRtl`
- [ ] `DemoSwitchChoiceCard` (needs Field component)

### Table
- [x] `DemoTable`
- [x] `DemoTableRtl`

### Tabs
- [x] `DemoTabs`
- [x] `DemoTabsLine`
- [x] `DemoTabsRtl`
- [x] `DemoTabsVertical`

### Textarea
- [x] `DemoTextarea`
- [x] `DemoTextareaRtl`

### Theme Toggle
- [x] `DemoThemeToggle`

### Toggle Group
- [x] `DemoToggleGroup`
- [x] `DemoToggleGroupFontWeight`
- [x] `DemoToggleGroupOutline`
- [x] `DemoToggleGroupRtl`
- [x] `DemoToggleGroupSpacing`
- [x] `DemoToggleGroupVertical`

### Tooltip
- [x] `DemoTooltip`
- [x] `DemoTooltipRtl`
