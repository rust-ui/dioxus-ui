# Dev Changelog

Internal changelog for the dioxus-ui site (not user-facing).

## 2026-09-09

### Improvements

- **Component doc metadata**: Synced the TOML frontmatter of every
  `public/docs/components/*.md` to match the leptos site exactly (same keys,
  same values): `title`, `description`, `tags`, `is_new`, `image`,
  `image_dark`. 75 files updated. Many cards on `/docs/components` had an empty
  `description` or `tags`, ad-hoc dioxus-only wording, or a missing
  `image` / `image_dark` (falling back to nothing or `_placeholder.webp`);
  they now carry the same copy and thumbnails as leptos. Labels also
  normalise (`Auto Form` -> `AutoForm`, `Multi Select` -> `MultiSelect`,
  `Drag And Drop` -> `Drag and Drop`). 3 dioxus-only docs with no leptos
  counterpart (`image`, `mask`, `toolbar`) left untouched. Doc bodies are
  unchanged. Regenerated `src/__registry__/{demos_sidenav,static_md_registry,
  my_command_bar_constants}.rs` and `public/documentation_map.md`.
  `public/docs/components/*.md`

- **Page transitions**: Ported the leptos `page_transition` util.
  `src/utils/page_transition.rs` adds `retrigger_page_fade()` (replays the
  `page__fade` intro on `#page__outlet` on every route change) and a
  `ScrollToTop` component (resets `#data-scroll-target` scroll on nav, skips
  `/blocks/*`). Wired into `AppLayout` plus the docs / blocks / charts /
  workflows layouts; the `@keyframes page__fade_in` / `.page__fade` rule now
  ships from `main.rs`. Before, the fade played only on the first hard load and
  pages kept their previous scroll position after client-side nav.
  `src/utils/`, `src/main.rs`, `src/routes/app_layout.rs`,
  `src/routes/docs_layout.rs`,
  `src/domain/{blocks,charts,workflows}/routing/*_layout.rs`, `Cargo.toml`

- **Animate demo**: Brought `/docs/components/animate` to parity with the leptos
  site. `AnimateHoverVariant` gained the 8 variants leptos carries but dioxus was
  missing (`BounceCustom`, `FadeOutDownV2`, `FlashV0`, `JiggleV0`, `PulseCustom`,
  `RubberBandV0`, `ShakeV0`, `SwingV0`) plus a `strum::Display` derive, and the
  demo now renders the full `HOVER_ANIMATIONS` list (85 swatches) in the same
  `Grid3` of `Card` + `Animate` + `AnimatedChildren` + `CardDescription` layout
  as leptos, with a heading that shows the count. Known gap, shared with leptos:
  Tailwind v4 does not emit `hover:animate-<Name>` for names ending in `V0`/`V2`,
  so those 6 swatches are present but inert until the keyframes are renamed.
  `app_crates/registry/src/ui/animate.rs`,
  `app_crates/registry/src/demos/demo_animate.rs`
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
  title, `z-20` / `shrink-0` on the action cluster). The tags row is now always
  rendered, matching leptos: it doubles as the `mb-6` spacer below the header, so
  components with no tags (Avatar, etc.) keep the same gap before the demo block
  instead of having it collapse. `src/components/doc_header.rs`

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
