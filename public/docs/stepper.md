+++
title = "Stepper"
description = "Rust/UI component for multi-step flows with a shared, navigable step index."
+++

<DemoStepper />

## Installation

<InstallStepper />

## Usage

```rust
use crate::ui::stepper::{
    Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger,
};
```

```rust
rsx! {
    Stepper { total_steps: 3, default_step: 0,
        StepperItem { step: 0,
            StepperTrigger {
                StepperIndicator {}
                StepperTitle { "Account" }
            }
            StepperSeparator {}
        }
    }
}
```

## Examples

### Vertical

Use the `orientation` prop to switch to a vertical layout.

<DemoStepperVertical />

### Controlled

Read the shared `StepperContext` from context to drive external navigation controls.

<DemoStepperControlled />

## See Also

- [Progress](/components/progress)
- [Tabs](/components/tabs)
