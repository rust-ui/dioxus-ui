# Plan: Match Leptos Architecture Exactly

Goal: make `dioxus-ui` use the same docs/registry/workflow architecture as the Leptos app under `/Users/user/dev/1-RUST/RUST-UI/app*`.

Constraint: this is not "equivalent architecture". We want the same architecture and the same responsibility split as Leptos wherever Dioxus does not force a technical difference.

This file tracks the confirmed gaps and the implementation order needed to align Dioxus to that exact target.

## Current Conclusion

The original architecture mismatch has now been fixed in the Dioxus repo.

Completed:

- `dioxus-ui/src/__registry__/static_md_registry.rs` is now the central markdown assembly point
- docs/component/hook routes render through the central markdown path
- shared wrappers now exist for demos, install blocks, and md docs utility blocks
- `dioxus-ui/src/registry/` has been reduced to the Leptos-style minimal role
- the old generated per-component docs registry files under `src/registry/*.rs` have been removed
- markdown docs now use the Leptos-style `Static*` / `StaticInstall*` workflow
- `public/docs` now follows the Leptos-style split with:
  - root get-started docs
  - `public/docs/components/`
  - `public/docs/hooks/`
- Dioxus private registry naming is now closer to Leptos:
  - `demos_sidenav.rs`
  - `my_command_bar_constants.rs`

The remaining work is now mostly non-architectural:

- finishing content parity for any pages/hooks that still exist in Leptos but not yet in Dioxus
- cleaning missing frontmatter assets (`image`, `image_dark`) in many docs files
- removing leftover warnings unrelated to the docs architecture migration

## Exact Target Architecture

We want Dioxus to mirror the same overall structure as Leptos:

1. `src/__registry__/` is the central assembly layer
2. `src/registry/` stays minimal
3. markdown docs/demo/install registration is assembled centrally, not per component page
4. shared wrappers exist for:
   - demos
   - install blocks
   - md docs utility blocks
5. docs markdown files stay declarative, using shared tags
6. actual UI components, hooks, and demos live in `app_crates/registry/src/...`
7. Dioxus file/folder responsibilities should match the Leptos ones, not just the behavior

The concrete Leptos shape we are targeting is:

- `app/src/__registry__/`
  - `all_blocks.rs`
  - `demos_sidenav.rs`
  - `mod.rs`
  - `my_command_bar_constants.rs`
  - `static_md_registry.rs`
- `app/src/registry/`
  - `mod.rs`
  - `md_docs/mod.rs`
  - `md_docs/docs_installation_cli_tree_view.rs`

So the Dioxus end-state should converge toward that same split:

- `dioxus-ui/src/__registry__/` becomes the main assembly point
- `dioxus-ui/src/registry/` becomes tiny
- files like `dioxus-ui/src/registry/alert.rs` should disappear

## Reference Leptos Tree

This is the concrete Leptos structure we are using as the target reference.

```text
app/
├── Cargo.toml
├── build.rs
└── src/
    ├── MARKDOWN.md
    ├── __registry__/
    │   ├── all_blocks.rs
    │   ├── demos_sidenav.rs
    │   ├── mod.rs
    │   ├── my_command_bar_constants.rs
    │   └── static_md_registry.rs
    ├── app.rs
    ├── components/
    ├── domain/
    │   ├── blocks/
    │   ├── bug_report/
    │   ├── charts/
    │   ├── create/
    │   ├── docs/
    │   ├── icons/
    │   ├── markdown_ui/
    │   ├── tests/
    │   ├── themes/
    │   └── views/
    ├── lib.rs
    ├── registry/
    │   ├── mod.rs
    │   └── md_docs/
    │       ├── mod.rs
    │       └── docs_installation_cli_tree_view.rs
    ├── routes/
    │   ├── mod.rs
    │   ├── page_charts.rs
    │   ├── page_download.rs
    │   ├── page_home.rs
    │   ├── page_home_sparkles.rs
    │   └── page_not_found.rs
    ├── shell.rs
    ├── sidenav_shortfix_all_blocks.rs
    └── utils/

app_crates/
├── app_components/
│   └── src/
├── app_config/
│   └── src/
├── app_domain/
│   └── src/
│       ├── constants/
│       ├── icons/
│       ├── markdown_config/
│       ├── themes/
│       └── utils/
├── app_routes/
│   └── src/
│       ├── blocks_routes.rs
│       ├── charts_routes.rs
│       ├── docs_routes.rs
│       └── lib.rs
└── registry/
    └── src/
        ├── blocks/
        ├── charts/
        ├── constants/
        ├── demos/
        ├── hooks/
        ├── lib.rs
        ├── ui/
        └── utils/
```

## Dioxus Structure Target

Dioxus should be brought to the same split:

```text
dioxus-ui/
├── app_crates/
│   ├── app_components/            <- add if Leptos responsibilities require it
│   ├── app_config/
│   ├── app_domain/
│   ├── app_routes/
│   └── registry/
│       └── src/
│           ├── blocks/
│           ├── charts/
│           ├── constants/
│           ├── demos/
│           ├── hooks/
│           ├── lib.rs
│           ├── ui/
│           └── utils/
└── src/
    ├── __registry__/
    │   ├── all_blocks.rs
    │   ├── demos_sidenav.rs
    │   ├── mod.rs
    │   ├── my_command_bar_constants.rs
    │   └── static_md_registry.rs
    ├── components/
    ├── domain/
    ├── registry/
    │   ├── mod.rs
    │   └── md_docs/
    ├── routes/
    └── utils/
```

Anything outside this target split needs to be justified by a real Dioxus-specific technical constraint, not convenience.

## Priority Order

### P0 — Core docs architecture replacement

This is now done for the docs markdown workflow and registry split.

#### 1. Delete the current per-component registry pattern

Status:

- [x] old per-component docs registry pattern removed
- [x] old hook docs registry pattern removed

Target state:

- one central registry builder under `dioxus-ui/src/__registry__/static_md_registry.rs`
- all markdown tag registrations handled there
- no per-component docs registry files
- same flow as Leptos `MyMd` in `app/src/__registry__/static_md_registry.rs`

Required work:

- [x] Add `dioxus-ui/src/__registry__/static_md_registry.rs`
- [x] Centralize all `MdComponents` registrations there
- [x] Stop treating each docs page like its own registry module
- [x] Remove the dependency on `src/registry/<component>.rs` and `src/registry/hooks/<hook>.rs` for markdown component wiring
- [x] Delete obsolete per-component docs registry files once the central path is live
- [x] Keep page metadata and markdown loading aligned with the Leptos pattern
- [x] Move Dioxus docs into the same root/components/hooks split as Leptos
- [x] Switch markdown tags from `Demo*` / `Install*` to `Static*` / `StaticInstall*`

#### 2. Copy the Leptos wrapper flow into Dioxus

Leptos uses dedicated wrappers:

- `static_demo_wrapper.rs`
- `static_install_wrapper.rs`
- `static_md_docs_wrapper.rs`

Target state in Dioxus:

- demo markdown tags render through one shared demo wrapper
- install markdown tags render through one shared install wrapper
- docs utility blocks render through one shared docs wrapper

Required work:

- [x] Add `dioxus-ui/src/domain/markdown_ui/components/static_demo_wrapper.rs`
- [x] Add `dioxus-ui/src/domain/markdown_ui/components/static_install_wrapper.rs`
- [x] Add `dioxus-ui/src/domain/markdown_ui/components/static_md_docs_wrapper.rs`
- [x] Route markdown tag rendering through these wrappers instead of per-page ad hoc components

#### 3. Copy the Leptos unified markdown registry model

Leptos has a unified registry model:

- `MarkdownType`
- static registry entry getters
- shared mapping for demo/install/doc block metadata

Current note:

- Dioxus now has the central registry file and central metadata ownership
- it does not yet copy the Leptos `MarkdownType` / `get_static_registry_entry(...)` API shape literally

Required work:

- [ ] Decide whether to port the Leptos `MarkdownType` enum literally
- [ ] Decide whether to port `get_static_registry_entry(...)` literally
- [x] Store demo/install metadata centrally instead of scattering it across `src/registry/*.rs`
- [x] Ensure wrappers resolve code/install metadata from that central source

#### 4. Make `src/registry/` match the Leptos shape

Leptos `app/src/registry/` is minimal and mainly used for md docs helper content.

Required work:

- [x] Audit every file in `dioxus-ui/src/registry/`
- [x] Shrink it toward the Leptos shape
- [x] keep `mod.rs`
- [x] keep only `md_docs/...` style helper content if still needed
- [x] remove component registry files from `src/registry/`
- [x] remove hook registry files from `src/registry/hooks/`
- [x] move docs/demo/install assembly concerns into `src/__registry__/`

#### 5. Make `src/__registry__/mod.rs` match the Leptos role

Leptos currently exposes:

- `all_blocks`
- `demos_sidenav`
- `my_command_bar_constants`
- `static_md_registry`

Required work:

- [x] audit current `dioxus-ui/src/__registry__/mod.rs`
- [x] make `static_md_registry` a first-class module there
- [ ] rename remaining modules to match Leptos more literally where needed
- [ ] decide what to do with Dioxus-only modules like `all_workflows.rs`
- [ ] ensure command bar constants and demo sidenav data live in `__registry__`, not mixed into the old docs registry layer

### P1 — Finish the remaining structure mismatches

Once the architecture is corrected, we need the same ownership boundaries.

#### 6. Keep demos, hooks, and UI source in `app_crates/registry`

User requirement confirmed:

- hook/component/demo source should follow the same overall architecture as Leptos
- not a separate Dioxus-only docs registry structure

Required work:

- [x] Verify every demo lives under `dioxus-ui/app_crates/registry/src/demos/`
- [x] Verify every hook lives under `dioxus-ui/app_crates/registry/src/hooks/`
- [x] Verify every UI primitive/component lives under `dioxus-ui/app_crates/registry/src/ui/`
- [x] Remove the old docs-registry duplication between `src/registry/` and `app_crates/registry/`
- [ ] Add `app_crates/app_components/` if we want to mirror Leptos even more literally

#### 7. Make docs pages declarative instead of registry-driven per page

Target state:

- docs markdown should mostly reference shared tags
- the central registry should decide how those tags render

Required work:

- [ ] Review all docs markdown in `public/docs/`
- [x] Normalize demo/install tags to the central workflow
- [x] Remove page-specific rendering logic that existed only because of the old Dioxus registry design

### P2 — Finish missing content after architecture parity

After the workflow matches, fill remaining content gaps.

#### 8. Hook docs parity

Leptos hook docs set is larger than the currently exposed Dioxus set.

Missing or not fully wired on Dioxus:

- [ ] `use_history`
- [ ] `use_horizontal_scroll`
- [ ] `use_is_mobile`
- [ ] `use_locks`
- [ ] `use_media_query`
- [ ] `use_press_hold`

Required per hook:

- [ ] add or finish markdown doc
- [ ] wire it into the central markdown registry
- [ ] expose it in sidenav
- [ ] expose it in command/search navigation
- [ ] verify demo/install/code metadata resolution via the new central workflow

#### 9. Remaining install/docs placeholders

Still confirmed as placeholder-style gaps:

- [ ] `public/docs/figma.md`
- [ ] `public/docs/hooks/use_copy_clipboard.md`
- [ ] `public/docs/hooks/use_lock_body_scroll.md`
- [ ] `public/docs/hooks/use_random.md`

Also missing relative to Leptos install experience:

- [ ] installation docs tree/file-view block equivalent to Leptos `docs_installation_cli_tree_view.rs`

#### 10. Missing component demos

Confirmed missing versus Leptos:

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

#### 11. Missing hook demos

Confirmed missing versus Leptos:

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

### P3 — Navigation and docs IA parity

After the central markdown workflow exists, align navigation and route shape.

#### 12. Match Leptos docs taxonomy and route intent

Confirmed mismatch today:

- Dioxus command bar mixes get-started pages and component-style entries differently from Leptos
- some docs pages are exposed through component-style surfaces instead of dedicated docs taxonomy

Required work:

- [ ] align command bar route targets with the Leptos structure
- [ ] align docs/get-started/component grouping with the Leptos IA
- [ ] remove duplicated or misplaced entries created by the old Dioxus registry design

#### 13. Missing overview/supporting pages

- [ ] `public/docs/workflow.md`
- [ ] `public/docs/cli.md`
- [ ] `public/docs/icons.md`
- [ ] all-demos overview page equivalent
- [ ] download page equivalent if still present in Leptos UX

### P4 — Test parity after workflow stabilization

Do not port tests in bulk before the markdown/docs workflow is stable.

#### 14. Restore Playwright parity

Current state:

- Leptos component specs: much broader coverage
- Dioxus component specs: only a small subset
- Dioxus hook specs: missing

Required work:

- [ ] add Dioxus hook e2e coverage
- [ ] port Leptos component specs where DOM/behavior match
- [ ] port hook specs after docs/demo routes stabilize
- [ ] validate wrappers, markdown tags, and install blocks through e2e tests

### P5 — Optional cleanup

#### 15. Organizational parity cleanup

- [ ] centralize shared constants if still useful after the refactor
- [ ] port `bug_report` domain only if it is still a real Dioxus product requirement
- [ ] review any remaining domain structure differences after the docs workflow is aligned

## Already Confirmed As Done Or Not A Gap

- [x] central docs architecture moved to `src/__registry__/static_md_registry.rs`
- [x] per-component docs registry files removed
- [x] shared markdown wrappers added
- [x] `src/registry/md_docs/docs_installation_cli_tree_view.rs` added
- [x] component/docs/hook pages now render through the central markdown registry
- [x] Core UI component parity is broadly in place
- [x] `marker` parity handled
- [x] `stepper` parity handled
- [x] `workflow` exists on Dioxus as a Dioxus-specific surface
- [x] `dropzone`, `radio_group`, `toggle`, `toolbar` are not Leptos parity blockers by themselves
- [x] Introduction page exists
- [x] Installation page exists
- [x] Changelog page exists
- [x] Figma page exists
- [x] RTL overview exists

## Recommended Execution Order

1. Decide whether to copy Leptos `MarkdownType` / `get_static_registry_entry(...)` literally
2. Tighten remaining `__registry__` naming/module differences
3. Finish missing hooks, demos, and install/docs placeholders
4. Align navigation and route taxonomy
5. Port Playwright coverage
