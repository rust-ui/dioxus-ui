# Plan: matching leptos (`app_crates/registry`, `app/src`)

Goal: dioxus-ui = feature parity with the leptos version (`/Users/user/dev/1-RUST/RUST-UI/app*`).

## 1. Missing UI components (`app_crates/registry/src/ui/`)

- [x] `marker.rs` — ported, wired (mod.rs, registry, sidenav, command bar), cargo check clean
- [x] `stepper.rs` — ported, wired, cargo check clean

Components dioxus has extra (not in leptos, leave as-is): `dropzone`, `radio_group`, `toggle`, `toolbar`, `workflow`.

## 2. Hooks — partially exposed

Leptos has 9 hook docs in `public/docs/hooks/`; dioxus currently exposes only 3 (`use_copy_clipboard`, `use_lock_body_scroll`, `use_random`) in `src/registry/hooks/` and `src/__registry__/sidenav_hooks.rs`.

These 6 hook files exist on the dioxus side, but still have no doc page and no sidenav entry:

- [ ] `use_history`
- [ ] `use_horizontal_scroll`
- [ ] `use_is_mobile`
- [ ] `use_locks`
- [ ] `use_media_query`
- [ ] `use_press_hold`

Actions per hook:
- doc `.md` in `public/docs/hooks/` (adapt from leptos `public/docs/hooks/*.md`)
- entry in `src/__registry__/sidenav_hooks.rs`

Hook demos are a separate gap. Missing on dioxus side versus leptos:

- [ ] `demo_use_copy_to_clipboard.rs`
- [ ] `demo_use_horizontal_scroll.rs`
- [ ] `demo_use_is_mobile.rs`
- [ ] `demo_use_lock_body_scroll.rs`
- [ ] `demo_use_locks.rs`
- [ ] `demo_use_media_query.rs`
- [ ] `demo_use_press_hold.rs`
- [ ] `demo_use_random.rs`

- [x] `use_stepper` — ported, wired into `hooks/mod.rs`
- [x] `demo_use_history.rs` — already present on dioxus side

## 3. Missing demos (component already exists on both sides)

Registry demo count: leptos `322`, dioxus `312`.

- [ ] `demo_item_*` (5): file_upload, group, media_image, rtl, variants
- [ ] `demo_input_group_*` (6): custom, dropdown, in_card, kbd, spinner, tooltip
- [ ] `demo_kbd_input_group.rs`
- [ ] `demo_empty_input_group.rs`
- [x] marker/stepper demos (12 files) — ported

## 4. `constants/` module missing (minor, cosmetic)

Leptos centralizes pagination/grid constants in `app_crates/registry/src/constants/{mod,pagination}.rs` (`PAGINATION::DEFAULT_PAGE_SIZE`, `PAGE_SIZE_OPTIONS`, `ROW_HEIGHT`), used by `use_virtual_scroll.rs`.
Dioxus hardcodes the same values (e.g. `ROW_HEIGHT = 36`) inline in `use_virtual_scroll.rs` instead of a shared module. Functionally equivalent — optional refactor for parity, not a real feature gap.

- [ ] `app_crates/registry/src/constants/mod.rs` + `pagination.rs` (optional)

## 5. Missing docs for components already ported on dioxus side

- [ ] `public/docs/workflow.md`

(`radio_group` doc already exists, just named `radio-group.md` — hyphen instead of underscore, inconsistent with rest but not missing.)

## 6. Domain — to verify / discuss

- [ ] `bug_report` — absent everywhere, no trace on dioxus side. Check if relevant.
- [ ] `docs` (internal domain/docs page) — absent. Check if relevant.

Note: `icons` and `themes` looked missing but already exist under `app_crates/app_domain/src/` (just organized differently from leptos `app/src/domain/`) — not an actual gap. Same for `charts` routing (`app_crates/app_routes/src/charts_routes.rs` exists, just not under `src/routes/`).

## 7. Missing general/static site pages

No doc, page, or route found anywhere in dioxus-ui for:

- [x] Introduction (leptos: `public/docs/introduction.md`) — ported, wired via new `/docs/:name` route
- [x] Installation (leptos: `public/docs/installation.md`) — ported, Framework Support section reframed for Dioxus
- [x] Changelog (leptos: `public/docs/changelog.md`) — created fresh (dioxus has no history to backfill)
- [x] Figma (leptos: `public/docs/figma.md`) — ported as-is
- [x] RTL overview page (leptos: `public/docs/rtl.md`) — ported, added a live `DemoButtonRtl` example
- [x] 404 / not-found route (leptos: `app/src/routes/page_not_found.rs`) — added `PageNotFound` + catch-all `#[route("/:..segments")]`

Lower priority / check relevance before porting:
- [ ] Download page (leptos: `app/src/routes/page_download.rs`, desktop release downloads) — confirm dioxus ships/distributes the same way before porting
- [ ] `cli.md` — present in leptos `public/docs/cli.md`, absent from dioxus `public/docs/`; dioxus has its own CLI (`ui-cli` crate), so this needs dioxus-specific content rather than a blind copy
- [ ] `icons.md` static doc page — present in leptos `public/docs/icons.md`, absent from dioxus `public/docs/`; note that dioxus already has a dedicated `/icons` route via `src/routes/page_icons.rs`, so this is a docs-content gap, not a feature gap

## 8. E2E test coverage (Playwright) — large gap

Leptos `e2e/tests/components/` has 60 component `.spec.ts` files + a `hooks/` test folder. Dioxus `e2e/tests/components/` currently has only 3 specs: `card.spec.ts`, `node_canvas.spec.ts`, `workflow.spec.ts`.

- [ ] Missing entire `hooks/` e2e test folder
- [ ] Missing 6 hook specs from leptos: `use-copy-clipboard`, `use-history`, `use-horizontal-scroll`, `use-lock-body-scroll`, `use-press-hold`, `use-random`
- [ ] Missing 59 component specs (accordion, alert, alert-dialog, auto-form, avatar, badge, bottom-nav, breadcrumb, button*, card-carousel, chart, checkbox, chips, combobox, command, context-menu, create-page, data-table, date-picker, dialog, drag-and-drop, drawer, dropdown-menu, dropzone, empty, form, input*, item, kbd, label, marquee, multi-select, pagination, popover, pressable, radio-button*, scroll-area, select, separator, sheet, sidenav, skeleton, slider, sonner, spinner, status, switch, table, tabs, textarea, theme-toggle, toast, tooltip)

This is likely the biggest real quality/coverage gap versus leptos — worth its own pass once components/demos above are ported (tests need matching selectors/behavior to port meaningfully).

## 9. `## Installation` section is a stub on every doc page — systemic gap

Found by grepping "Coming soon" across `public/docs/`: there are currently **92** matches on the dioxus side. The previous 91/91 count is stale now that additional pages exist and Alert has already been converted to `<InstallAlert />`. Leptos never has this stub — every component doc renders a real `<StaticInstallBadge />`-style component instead.

Leptos mechanism (read for reference):
- `app/src/domain/markdown_ui/components/static_install_wrapper.rs` — `StaticInstallWrapper` component: two tabs (CLI / Manual). CLI tab shows `cargo install ui-cli --force` + `ui add <demo_name>` + `ui add <install_name>`, syntax-highlighted (`SyntectHighlighterCode`). Manual tab shows the full raw component source with expand/collapse.
- `app/src/__registry__/static_md_registry.rs` — lookup table mapping each component name (`MarkdownType` enum) to `{ install_name, demo_name, raw_code }`.
- Per-component markdown files reference a per-component wrapper like `<StaticInstallBadge />`, resolved through the markdown-to-component pipeline (same one that resolves `<DemoBadge />` etc.).

Dioxus already has the building block needed (syntax highlighting exists: `dioxus-ui/src/markdown/highlight_code.rs`, syntect-based) — just missing the wrapper component + registry + wiring into all 91 doc pages.

- [x] Design the dioxus version: built a single generic component `src/components/install_command.rs` (`InstallCommand { name, demo_name, raw_code }`) instead of 91 generated per-component wrapper functions like leptos — simpler since each registry file just passes its own `include_str!` + names as props.
- [x] Build the CLI/Manual tabs component — reuses `registry::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent}` + `highlight_code.rs`; Manual tab has expand/collapse via `Button` (Secondary/Sm), matching leptos's UX.
- [x] Proof of concept done on **Alert**: `src/registry/alert.rs` registers `InstallAlert` tag calling `InstallCommand` with `include_str!("../../app_crates/registry/src/ui/alert.rs")`; `public/docs/alert.md` now uses `<InstallAlert />` instead of "Coming soon.". `cargo check --workspace` clean.
- [ ] Roll out the same pattern to the other ~90 `public/docs/*.md` + `src/registry/*.rs` files (register an `Install<Name>` tag per component, same 3-line pattern as alert.rs)
- [ ] Verify `ui-cli add <component>` command actually works for dioxus components before advertising it in the docs (check `ui-cli`'s dioxus support — `PLAN_HANDLE_DIOXUS.md` in that crate)

Biggest systemic/visible gap for actual site visitors — every single component doc page shows a broken-looking stub. Alert now fixed as reference implementation for the rollout.
