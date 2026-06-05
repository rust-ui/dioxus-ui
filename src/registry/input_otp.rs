use dioxus::prelude::*;
use registry::demos::demo_input_otp::DemoInputOtp;
use registry::demos::demo_input_otp_separator::DemoInputOtpSeparator;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static INPUT_OTP: RegistryEntry = RegistryEntry {
    slug: "input-otp",
    raw: include_str!("../../public/docs/input_otp.md"),
    tags: &[],
    components: input_otp_components,
};

fn input_otp_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoInputOtp", |_| rsx! { DemoInputOtp {} });
    c.add("DemoInputOtpSeparator", |_| rsx! { DemoInputOtpSeparator {} });
    c
}
