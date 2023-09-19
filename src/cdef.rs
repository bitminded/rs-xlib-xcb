extern crate xlib;
extern crate xcb;

#[link(name = "X11-xcb")]
extern "system"
{
    pub fn XGetXCBConnection(display: *mut xlib::cdef::Display) -> *mut xcb::cdef::XCBConnection;
}
