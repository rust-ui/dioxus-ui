# Plan: Match Leptos Architecture As Literally As Possible

Goal: make `dioxus-ui` match the Leptos app architecture and file responsibilities as closely as possible.

Constraint:

- this is not "rough parity"
- this is not "same behavior, different structure"
- Dioxus should copy the Leptos architecture, module split, naming, and ownership boundaries as literally as possible
- a difference is acceptable only when Dioxus creates a real technical constraint

This file intentionally keeps only the remaining mismatches. Completed work is removed to keep the plan readable.

## Target Rule

When there is a design choice:

- prefer the same file split as Leptos
- prefer the same module naming as Leptos
- prefer the same ownership boundaries as Leptos
- do not keep a Dioxus-specific structure just because it already works

## Current Remaining Gaps

### 1. Tighten `src/__registry__/` to the Leptos shape

Leptos exposes:

- `all_blocks`
- `demos_sidenav`
- `my_command_bar_constants`
- `static_md_registry`

Dioxus still has extra modules in `src/__registry__/`:

- `all_workflows.rs`
- `sidenav_get_started.rs`
- `sidenav_hooks.rs`
- `source_map.rs`

Required work:

- [ ] reduce `dioxus-ui/src/__registry__/mod.rs` toward the exact Leptos role
- [ ] rename or collapse helper modules when Leptos keeps that responsibility elsewhere
- [ ] isolate Dioxus-only registry extras from the Leptos-parity path

### 2. Finish shrinking `src/registry/` to the literal Leptos role

Leptos `app/src/registry/` contains only:

- `mod.rs`
- `md_docs/mod.rs`
- `md_docs/docs_installation_cli_tree_view.rs`

Dioxus still keeps one extra module:

- `src/registry/types.rs`

Required work:

- [ ] decide whether `src/registry/types.rs` should move into `src/__registry__/` or another Leptos-matching owner
- [ ] make `dioxus-ui/src/registry/` as close as possible to the Leptos tree

### 3. Match `app_crates/registry/src/` ownership more literally

Leptos `app_crates/registry/src/` contains:

- `blocks/`
- `charts/`
- `constants/`
- `demos/`
- `hooks/`
- `ui/`
- `utils/`

Dioxus currently differs:

- missing `constants/`
- missing `utils/`
- has Dioxus-only `workflows/`

Required work:

- [ ] add `dioxus-ui/app_crates/registry/src/constants/` if that ownership should mirror Leptos
- [ ] add `dioxus-ui/app_crates/registry/src/utils/` if that ownership should mirror Leptos
- [ ] decide whether `app_crates/registry/src/workflows/` is isolated as a Dioxus-only surface or split away from the Leptos-parity tree

### 4. Match `app_crates/app_domain/src/` ownership more literally

Leptos `app_crates/app_domain/src/` contains:

- `constants/`
- `icons/`
- `markdown_config/`
- `themes/`
- `utils/`

Dioxus currently differs:

- missing `markdown_config/`
- missing `utils/`

Required work:

- [ ] add `dioxus-ui/app_crates/app_domain/src/markdown_config/` or move equivalent logic there
- [ ] add `dioxus-ui/app_crates/app_domain/src/utils/` or move equivalent logic there
- [ ] audit whether logic currently under `src/markdown/` belongs in the Leptos-style crate boundary

### 5. Match top-level `src/` layout more literally

Leptos top-level `src/` includes:

- `app.rs`
- `lib.rs`
- `shell.rs`
- `domain/tests/`

Dioxus currently differs:

- has `main.rs` instead of the Leptos-style split
- has no `shell.rs`
- uses `domain/test/` instead of `domain/tests/`
- still exposes top-level `src/markdown/`

Required work:

- [ ] port the entry layout toward the closest Dioxus equivalent of Leptos `app.rs` / `lib.rs` / `shell.rs`
- [ ] rename `src/domain/test/` to `src/domain/tests/` if Dioxus does not block it
- [ ] decide whether `src/markdown/` should be folded into Leptos-style ownership boundaries instead of staying top-level
- [ ] add `dioxus-ui/app_crates/app_components/` if Leptos responsibilities require that literal split

### 6. Support-page and route parity still missing

Confirmed remaining gaps versus the Leptos app shape:

- `/download` exists as a Dioxus route constant and home-page CTA target, but no Dioxus route currently serves it
- Leptos has `page_download.rs`
- Dioxus still lacks the matching page/route

Other remaining support-page gaps:

- [ ] `public/docs/workflow.md` if we want the same support-page set as Leptos
- [ ] all-demos overview page equivalent
- [ ] download page and route equivalent

### 7. Keep the public docs IA literally Leptos-like

Dioxus still exposes docs pages that do not exist in the Leptos public docs surface:

- `components/bento_grid.md`
- `components/chat.md`
- `components/expandable.md`
- `components/faq_transition.md`
- `components/image.md`
- `components/mask.md`
- `components/radio-group.md`
- `components/select_native.md`
- `components/toggle.md`
- `components/toolbar.md`

Required work:

- [ ] decide whether these pages move out of the Leptos-parity docs surface
- [ ] or add an explicit separation so the Leptos-matching IA stays literal

### 8. Remaining docs cleanup

Required work:

- [ ] review `public/docs/**` frontmatter completeness for `image` / `image_dark`
- [ ] enrich `public/docs/figma.md` only if we want the same content depth as Leptos

### 9. Test parity after structure parity

Do not expand tests before the remaining architecture gaps above are settled.

Required work:

- [ ] add Dioxus hook e2e coverage
- [ ] port Leptos component specs where DOM and behavior truly match
- [ ] validate markdown wrappers, tags, and install blocks through e2e coverage

## Confirmed Non-Gaps

These should not stay in this file anymore as active plan items:

- central markdown registry exists in `src/__registry__/static_md_registry.rs`
- shared markdown wrappers already exist
- `MarkdownType` already exists in Dioxus
- `get_static_registry_entry(...)` already exists in Dioxus
- old per-component docs registry files are already gone
- docs/component/hook rendering already goes through the central markdown path
- `src/registry/md_docs/docs_installation_cli_tree_view.rs` already exists

## Recommended Execution Order

1. Tighten `src/__registry__/` and `src/registry/` until they match the Leptos responsibility split as literally as possible.
2. Close the crate-boundary mismatches in `app_crates/registry`, `app_crates/app_domain`, and `app_crates/app_components`.
3. Move the top-level `src/` layout closer to the Leptos split where Dioxus allows it.
4. Add the missing support pages and fix the current `/download` gap.
5. Separate Dioxus-only docs from the Leptos-parity docs surface.
6. Finish docs frontmatter cleanup.
7. Expand e2e parity only after the structure is stable.
