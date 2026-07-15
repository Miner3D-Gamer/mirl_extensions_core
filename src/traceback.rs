#[cfg(feature = "std")]
#[macro_export]
/// Enables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `1`
macro_rules! enable_traceback {
    () => {
        unsafe {
            ::std::env::set_var("RUST_BACKTRACE", "1");
        }
    };
}

#[cfg(feature = "std")]
#[macro_export]
/// Enables the extended rust traceback by setting the environment variable `RUST_BACKTRACE` to `full`
macro_rules! enable_traceback_detailed {
    () => {
        unsafe {
            ::std::env::set_var("RUST_BACKTRACE", "full");
        }
    };
}

#[cfg(feature = "std")]
#[macro_export]
/// Disables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `0`
macro_rules! disable_traceback {
    () => {
        unsafe {
            ::std::env::set_var("RUST_BACKTRACE", "0");
        }
    };
}
