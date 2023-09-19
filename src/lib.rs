extern crate xlib;
extern crate xcb;

pub mod cdef;

// only used for transmuting into xcb::XCBConnection
struct XCBConnection
{
    _private: *mut xcb::cdef::XCBConnection
}

// only used for transmuting into xlib::Display
struct Display
{
    _private: *mut xlib::cdef::Display
}

pub fn x_get_xcb_connection(display: &xlib::Display) -> xcb::XCBConnection
{
    let connection = unsafe {
        let display_transmute: &Display = std::mem::transmute(display);
        cdef::XGetXCBConnection(display_transmute._private)
    };

    return unsafe
    {
        std::mem::transmute(XCBConnection
        {
            _private: connection
        })
    }
}
