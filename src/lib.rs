extern crate xcb;
extern crate xlib;

pub mod cdef;

// only used for transmuting into xcb::XCBConnection
struct XCBConnection {
    _private: *mut xcb::cdef::XCBConnection,
}

// only used for transmuting into xlib::Display
struct Display {
    _private: *mut xlib::cdef::Display,
}

/// Returns the XCB connection associated with an Xlib Display.
///
/// # Parameters
/// ## display
/// Specifies the connection to the X server.
///
/// # Return value
/// The XCB connection associated with the given Xlib Display.
///
/// # Remarks
/// The XCB connection returned by x_get_xcb_connection can be used with functions
/// from the XCB library, in the same way as an XCB connection created with XCB.
/// However, it probably is a good idea to use x_close_display instead of
/// xcb_disconnect to disconnect from the X server.
pub fn x_get_xcb_connection(display: &xlib::Display) -> xcb::XCBConnection {
    let connection = unsafe {
        let display_transmute: &Display = std::mem::transmute(display);
        cdef::XGetXCBConnection(display_transmute._private)
    };

    return unsafe {
        std::mem::transmute(XCBConnection {
            _private: connection,
        })
    };
}
