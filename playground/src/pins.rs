use ariel_os::hal::peripherals;

#[cfg(context = "st-nucleo-l552ze-q")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PC7 });
