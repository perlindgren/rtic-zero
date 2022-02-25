use cortex_m_semihosting::{debug, hprintln};
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    hprintln!("main").ok();
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
