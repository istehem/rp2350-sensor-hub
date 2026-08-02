#[cfg(test)]
mod defmt_stubs {
    #[no_mangle]
    extern "C" fn _defmt_panic() -> ! {
        panic!("defmt panic in test");
    }

    // Return u8 (0 = None, 1+ = Some(index)), not Option<u8>
    #[no_mangle]
    extern "C" fn _defmt_acquire() -> u8 {
        0 // Return 0 to indicate "no logger acquired"
    }

    // Use raw pointer and length, not &[u8]
    #[no_mangle]
    extern "C" fn _defmt_write(_bytes: *const u8, _len: usize) {
        // No-op for tests
    }

    #[no_mangle]
    extern "C" fn _defmt_timestamp() -> u64 {
        0
    }

    // Also add these if missing:
    #[no_mangle]
    extern "C" fn _defmt_flush() {
        // No-op for tests
    }

    #[no_mangle]
    extern "C" fn _defmt_release() {
        // No-op for tests
    }
}
