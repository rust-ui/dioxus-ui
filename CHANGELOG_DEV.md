# Dev Changelog

Internal changelog for the dioxus-ui site (not user-facing).

## 2026-09-09

### Bug Fixes

- **DemoWrapper**: Reset the Preview/Code tab to Preview when navigating between
  component docs. Client-side nav reuses the component instance, so the `tab`
  signal survived route changes and stayed stuck on Code. Fixed by comparing the
  previous `demo_name` during render (no `use_effect`).
  `src/components/demo_wrapper.rs`
