extern crate xcb;
extern crate xlib;

#[link(name = "X11-xcb")]
extern "system" {
    pub fn XGetXCBConnection(display: *mut xlib::cdef::Display) -> *mut xcb::cdef::XCBConnection;
}
