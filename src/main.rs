extern crate libc;
extern crate gl;
extern crate glutin;

fn main() {
    let window = glutin::Window::new().unwrap();

    unsafe {
        let _ = window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.6, 0.6, 0.6, 1.0);
    }

    for event in window.wait_events() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        let _ = window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
