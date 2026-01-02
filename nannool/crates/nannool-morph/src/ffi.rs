use std::os::raw::{c_char, c_int};

/// Opaque pointer to fsm structure in libfoma
#[repr(C)]
pub struct Fsm {
    _unused: [u8; 0],
}

/// Opaque pointer to apply_handle structure in libfoma
#[repr(C)]
pub struct ApplyHandle {
    _unused: [u8; 0],
}

extern "C" {
    /// Read a binary FST file
    pub fn fsm_read_binary_file(filename: *const c_char) -> *mut Fsm;

    /// Free a FSM, associated data such as alphabet and confusion matrix
    pub fn fsm_destroy(net: *mut Fsm) -> c_int;

    /// Initialize lookup handle for a FSM
    pub fn apply_init(net: *mut Fsm) -> *mut ApplyHandle;

    /// Free memory allocated by apply_init
    pub fn apply_clear(h: *mut ApplyHandle);

    /// Downward lookup (surface to underlying)
    pub fn apply_down(h: *mut ApplyHandle, word: *const c_char) -> *mut c_char;

    /// Upward lookup (underlying to surface)
    pub fn apply_up(h: *mut ApplyHandle, word: *const c_char) -> *mut c_char;

    /// Reset the iterator to start anew (though apply_down/up usually do this)
    pub fn apply_reset_enumerator(h: *mut ApplyHandle);
}
