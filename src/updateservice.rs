use librustdesk::*;

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {
    hbb_common::init_log(false, "updateservice");
    crate::update_service::start_service_dispatcher();
}
