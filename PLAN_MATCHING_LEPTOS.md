# Plan: Match Leptos Feature Parity

Goal: bring `dioxus-ui` to practical feature parity with the Leptos implementation under `/Users/user/dev/1-RUST/RUST-UI/app*`.

This file tracks confirmed gaps only. It is organized by execution priority rather than by folder so it can be used as a working backlog.

## Current Summary

- Core UI component parity is effectively complete.
- The largest visible product gap is documentation quality, especially install sections that still show placeholder content.
- The largest structural gap is Playwright coverage.
- The best implementation order is: install/docs rollout -> hooks exposure -> missing demos -> remaining docs pages -> test parity -> optional cleanup.

## Priority Order

### P0 — Highest Priority: user-visible gaps

These are the issues users will notice immediately when browsing the Dioxus docs site.

#### 1. Installation sections still show placeholder content on most docs pages

There are currently **92** `"Coming soon."` matches under `dioxus-ui/public/docs/`.

Leptos does not have this gap: component docs render a real install block with CLI and manual installation guidance.

Status:
- [x] Generic Dioxus install component exists: `src/components/install_command.rs`
- [x] Alert proof of concept is wired end-to-end
- [ ] Roll out the same pattern to the remaining component docs
- [ ] Verify that `ui-cli add <component>` is valid for Dioxus before documenting it broadly

Recommended execution:
1. Finish the install component rollout across all remaining component docs.
2. Validate the CLI command format in `ui-cli`.
3. Remove every remaining placeholder install section.

#### 2. Hooks exist but are not fully exposed in docs/navigation

Leptos has 9 hook docs in `public/docs/hooks/`.
Dioxus currently exposes only 3 in docs/registry/navigation:

- [x] `use_copy_clipboard`
- [x] `use_lock_body_scroll`
- [x] `use_random`

These 6 hooks exist on the Dioxus side but are still missing docs-page wiring and sidenav exposure:

- [ ] `use_history`
- [ ] `use_horizontal_scroll`
- [ ] `use_is_mobile`
- [ ] `use_locks`
- [ ] `use_media_query`
- [ ] `use_press_hold`

Required work per hook:
- [ ] Add the markdown doc in `public/docs/hooks/`
- [ ] Register it in `src/registry/hooks/`
- [ ] Add the sidenav entry in `src/__registry__/sidenav_hooks.rs`
- [ ] Ensure command bar / page navigation picks it up correctly

### P1 — High Priority: demo parity

These gaps directly affect the usefulness of the docs because the site depends heavily on live demos.

#### 3. Missing hook demos

The following hook demos exist on the Leptos side but are still missing in Dioxus:

- [ ] `demo_use_copy_to_clipboard.rs`
- [ ] `demo_use_horizontal_scroll.rs`
- [ ] `demo_use_is_mobile.rs`
- [ ] `demo_use_lock_body_scroll.rs`
- [ ] `demo_use_locks.rs`
- [ ] `demo_use_media_query.rs`
- [ ] `demo_use_press_hold.rs`
- [ ] `demo_use_random.rs`

Already present:
- [x] `demo_use_history.rs`

#### 4. Missing component demos

Registry demo count:
- Leptos: `322`
- Dioxus: `312`

Confirmed missing demos:
- [ ] `demo_item_file_upload.rs`
- [ ] `demo_item_group.rs`
- [ ] `demo_item_media_image.rs`
- [ ] `demo_item_rtl.rs`
- [ ] `demo_item_variants.rs`
- [ ] `demo_input_group_custom.rs`
- [ ] `demo_input_group_dropdown.rs`
- [ ] `demo_input_group_in_card.rs`
- [ ] `demo_input_group_kbd.rs`
- [ ] `demo_input_group_spinner.rs`
- [ ] `demo_input_group_tooltip.rs`
- [ ] `demo_kbd_input_group.rs`
- [ ] `demo_empty_input_group.rs`

Already completed:
- [x] Marker demos
- [x] Stepper demos

### P2 — Medium Priority: missing user-facing docs/pages

These are smaller than the install/docs systemic gap, but they are still visible holes in the site.

#### 5. Missing docs pages for surfaces that already exist

- [ ] `public/docs/workflow.md`
- [ ] `public/docs/cli.md`
- [ ] `public/docs/icons.md`

Notes:
- `workflow` exists as a Dioxus component surface but does not yet have a matching doc page.
- `cli.md` should be written for the Dioxus workflow. Do not blindly copy the Leptos version.
- `icons.md` is a docs-content gap only. Dioxus already has a dedicated `/icons` route via `src/routes/page_icons.rs`.

#### 6. Possibly missing download page

- [ ] `page_download.rs` equivalent, if Dioxus distribution needs the same download UX as Leptos

Decision needed:
- Confirm whether Dioxus ships downloadable desktop binaries through the same UX before porting this page.

### P3 — Medium/Low Priority: test coverage

This is a major gap, but it should follow the docs/demo stabilization work. Otherwise the tests will be ported against moving targets.

#### 7. Missing Playwright coverage

Current state:
- Leptos component specs: `60`
- Dioxus component specs: `3` (`card.spec.ts`, `node_canvas.spec.ts`, `workflow.spec.ts`)
- Leptos hook specs: `6`
- Dioxus hook specs: `0`

Missing hook test coverage:
- [ ] Add `e2e/tests/hooks/`
- [ ] Port `use-copy-clipboard.spec.ts`
- [ ] Port `use-history.spec.ts`
- [ ] Port `use-horizontal-scroll.spec.ts`
- [ ] Port `use-lock-body-scroll.spec.ts`
- [ ] Port `use-press-hold.spec.ts`
- [ ] Port `use-random.spec.ts`

Missing component test coverage:
- [ ] Port the missing component specs from Leptos (roughly 59 still missing on the Dioxus side)

Recommended execution:
1. Port tests first for hooks and components whose docs/demos are already finalized.
2. Reuse Leptos selectors and expectations only where the Dioxus DOM and behavior actually match.
3. Avoid bulk-copying tests before the remaining docs/demo work is stable.

### P4 — Low Priority: optional parity cleanup

These are real differences, but not meaningful product gaps.

#### 8. Shared constants module

Leptos centralizes pagination/grid constants in:
- `app_crates/registry/src/constants/mod.rs`
- `app_crates/registry/src/constants/pagination.rs`

Dioxus currently hardcodes equivalent values inline, such as `ROW_HEIGHT = 36`, in places like `use_virtual_scroll.rs`.

Status:
- [ ] Optional refactor to add `app_crates/registry/src/constants/mod.rs`
- [ ] Optional refactor to add `app_crates/registry/src/constants/pagination.rs`

This is organizational parity, not a missing feature.

#### 9. Domain-level items to confirm

- [ ] `bug_report` domain: confirm whether it is still relevant for Dioxus
- [ ] Internal `docs` domain/page structure: confirm whether it should exist in Dioxus in the same form

Notes:
- `icons` and `themes` are not actual gaps; they already exist under `app_crates/app_domain/src/`, just with different organization.
- `charts` routing is also not a real gap; it exists under `app_crates/app_routes/src/charts_routes.rs`.

## Already Completed

### UI component parity already handled

- [x] `marker.rs`
- [x] `stepper.rs`

Components that exist on Dioxus only and are not parity gaps:
- `dropzone`
- `radio_group`
- `toggle`
- `toolbar`
- `workflow`

### General pages already handled

- [x] Introduction
- [x] Installation
- [x] Changelog
- [x] Figma
- [x] RTL overview
- [x] 404 / catch-all not-found route

## Recommended Execution Plan

1. Finish the install-section rollout across all component docs.
2. Expose the missing hooks in docs, registry, sidenav, and navigation.
3. Port the missing hook demos and missing item/input-group demos.
4. Add the remaining user-facing docs pages: `workflow.md`, `cli.md`, `icons.md`.
5. Decide whether `page_download` is actually relevant for Dioxus.
6. Start Playwright parity only after the docs/demo surface above is stable.
7. Leave constants/domain cleanup for last.
