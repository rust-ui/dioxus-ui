# Dev Changelog

Internal changelog for the dioxus-ui site (not user-facing).

## 2026-09-09

### Improvements

- **Components index**: Wired preview thumbnails (light + dark) into the doc
  frontmatter for 20 components that already had screenshots on disk in
  `public/images/thumbnails/`: Accordion, Alert, Badge, Breadcrumb, Button,
  Checkbox, Date Picker, Dialog, Dropdown Menu, Dropzone, Input, Pagination,
  Popover, Select, Switch, Table, Tabs, Textarea, Toast, Tooltip. Cards on
  `/docs/components` previously showed the `_placeholder.webp` art or nothing
  because the `image` / `image_dark` frontmatter keys were missing.
  `public/docs/components/*.md`, regenerated `src/__registry__/demos_sidenav.rs`

### Bug Fixes

- **DemoWrapper**: Reset the Preview/Code tab to Preview when navigating between
  component docs. Client-side nav reuses the component instance, so the `tab`
  signal survived route changes and stayed stuck on Code. Fixed by comparing the
  previous `demo_name` during render (no `use_effect`).
  `src/components/demo_wrapper.rs`
