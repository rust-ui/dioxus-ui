# Dev Changelog

Internal changelog for the dioxus-ui site (not user-facing).

## 2026-09-09

### Improvements

- **DemoWrapper**: Brought the demo block to parity with the leptos site. The
  Preview/Code tab row now carries a right-side action column: a `ui add <demo>`
  outline button that copies the CLI command, and a kebab menu with "Copy Demo"
  (copies the source) and "View as Markdown". Code panel markup now matches
  leptos `SyntectHighlighterCode` (same wrapper, classes, no floating copy
  button); source is path-rewritten (`use crate::registry::` -> `use
  crate::components::`) before display.
  `src/domain/markdown_ui/components/static_demo_wrapper.rs`
- **Syntax highlighting**: syntect now runs on the client (WASM) too, not just
  SSR. Code kept its colours after hydration before only on server-rendered
  pages; client-rendered code fell back to plain escaped text. `syntect` /
  `html-escape` are now unconditional deps and `highlight_code` mirrors the
  leptos `_markdown_crate` shape (shared impl + per-target `SyntaxSet` /
  `ThemeSet` caches). Grows the WASM bundle (embedded syntax + theme dumps).
  `src/markdown/highlight_code.rs`, `src/markdown/mod.rs`, `Cargo.toml`
- **DocHeader**: Aligned spacing/layout classes with the leptos header
  (`mt-2` gaps restored above description and tags, `min-w-0` + `truncate` on the
  title, `z-20` / `shrink-0` on the action cluster). `src/components/doc_header.rs`

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

- **Table of contents**: "On This Page" showed the headings of the first doc
  visited instead of the current one, and briefly listed every entry twice
  during navigation. `ComponentPage` / `HookPage` are reused across client-side
  nav and pushed the TOC into a context signal owned by `DocsLayout`; the
  `use_effect` doing it had no reactive dependency on `name` (ran once, never
  updated), and writing a parent signal during child render also corrupted the
  diff and duplicated the list. Removed the context signal entirely: `DocsLayout`
  now derives the TOC directly from `use_route()`, which is reactive, so it
  tracks the current page with no effect and no cross-component write.
  `src/routes/docs_layout.rs`, `src/routes/component_page.rs`,
  `src/routes/hook_page.rs`
