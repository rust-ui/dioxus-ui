+++
title = "Input OTP"
description = "A one-time password input with individual digit slots."
+++

<DemoInputOtp />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator};
```

```rust
rsx! {
    InputOTP { max_length: 6,
        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
        }
    }
}
```

## Examples

### With Separator

<DemoInputOtpSeparator />

## See Also

- [Input](/components/input)
- [Field](/components/field)
