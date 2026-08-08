# Plan: Match Leptos Feature Parity

Goal: bring `dioxus-ui` to practical feature parity with the Leptos implementation under `/Users/user/dev/1-RUST/RUST-UI/app*`.

This file tracks confirmed gaps only. It is organized by execution priority rather than by folder so it can be used as a working backlog.

## Current Summary

- Core UI component parity is effectively complete.
- The largest visible product gap is now docs completeness for hooks and a few remaining docs pages, not the component install rollout itself.
- The largest structural gap is Playwright coverage.
- The best implementation order is: install/docs rollout -> hooks exposure -> missing demos -> remaining docs pages -> test parity -> optional cleanup.

## Priority Order

### P0 — Highest Priority: user-visible gaps

These are the issues users will notice immediately when browsing the Dioxus docs site.

#### 1. Installation rollout is mostly complete

The old large-scale install placeholder gap is no longer the main issue.

Current state checked in the repo:
- `96` markdown docs under `dioxus-ui/public/docs`
- `88` docs already use an `<Install... />` tag
- only `4` docs still contain a literal `"Coming soon."`

Those 4 remaining placeholder cases are:
- [ ] `public/docs/figma.md`
- [ ] `public/docs/hooks/use_copy_clipboard.md`
- [ ] `public/docs/hooks/use_lock_body_scroll.md`
- [ ] `public/docs/hooks/use_random.md`

Interpretation:
- Component installation coverage is largely done.
- The remaining install-related work is now concentrated in hook docs and a non-component static page (`figma.md`).

Status:
- [x] Generic Dioxus install component exists: `src/components/install_command.rs`
- [x] Component docs broadly use `<Install... />` wrappers now
- [ ] Verify that `ui-cli add <component>` is valid for Dioxus before documenting it broadly
- [ ] Replace the final remaining placeholder docs listed above

#### 1b. Installation page content is still less complete than Leptos

Leptos has an additional installation-specific interactive documentation block:
- `app/src/registry/md_docs/docs_installation_cli_tree_view.rs`

This renders a file tree + highlighted starter files inside the installation docs flow.
Dioxus currently has a simpler `public/docs/installation.md` page with prose and a single CLI command block, but no equivalent interactive installation example.

Status:
- [ ] Decide whether Dioxus should also have an installation-specific interactive example
- [ ] If yes, port or reimplement the tree/file-view example in Dioxus

This is now a secondary docs parity gap rather than a systemic install rollout blocker.

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

#### 6. Missing “all demos” overview page

Leptos has a dedicated overview page for browsing all component demos / hook demos:
- `app/src/domain/docs/routing/page_all_demos.rs`
- supported by `app/src/__registry__/demos_sidenav.rs`

I did not find an equivalent route/page on the Dioxus side.

Status:
- [ ] Decide whether Dioxus should expose an all-demos index page
- [ ] If yes, add the route, page, and demo index source

This is a real discoverability gap for users browsing the docs at a higher level than individual pages.

#### 7. Possibly missing download page

- [ ] `page_download.rs` equivalent, if Dioxus distribution needs the same download UX as Leptos

Decision needed:
- Confirm whether Dioxus ships downloadable desktop binaries through the same UX before porting this page.

#### 8. Command bar / top-level docs navigation taxonomy differs from Leptos

Leptos command-bar page entries point to dedicated page routes such as:
- `/docs/components/cli`
- `/docs/components/installation`
- `/icons`

Dioxus currently uses a different and partially collapsed scheme in `src/__registry__/command_bar.rs`:
- `Components`, `CLI`, and `Installation` page entries all exist under `CommandCategory::Pages`
- but `CLI` and `Installation` point to component-style routes
- and get-started docs like `Introduction`, `Installation`, `Changelog`, `Figma`, `RTL` also appear inside the component list

This may not always be a broken route, but it is a parity mismatch in navigation structure and information architecture.

Status:
- [ ] Review command-bar page entries for correct route targets
- [ ] Decide whether get-started docs should live only under `/docs/:name` in Dioxus
- [ ] Remove or justify duplicated docs entries under the component registry/search surface

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
