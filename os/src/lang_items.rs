use core::panic::PanicInfo;
use crate::{ERROR, sbi::shutdown};


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    if let Some(location) = info.location() {
        ERROR!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        ERROR!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}