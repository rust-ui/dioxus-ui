use super::types::RegistryEntry;
use crate::demos::demo_input_otp::DemoInputOtp;
use crate::demos::demo_input_otp_separator::DemoInputOtpSeparator;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
