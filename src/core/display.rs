#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(screen: usize) -> usize;
    fn XCloseDisplay(display: usize);
    fn XDefaultRootWindow(display: usize) -> usize;
    fn XStoreName(display: usize, window: usize, name: *const u8) -> i32;
    fn XFlush(display: usize) -> i32;
}

pub struct XDisplay {
    display: usize,
    window: usize,
}

impl XDisplay {
    pub fn new() -> Self {
        let display = unsafe { XOpenDisplay(0) };
        let window = unsafe { XDefaultRootWindow(display) };
        Self { display, window }
    }

    pub fn set_name(&self, name: &str) {
        let name = name.as_bytes();
        unsafe { XStoreName(self.display, self.window, name.as_ptr()) };
        unsafe { XFlush(self.display) };
    }
    pub fn close(&self) {
        unsafe { XCloseDisplay(self.display) };
    }
}
impl Drop for XDisplay {
    fn drop(&mut self) {
        self.close();
    }
}
