extern crate libc;
extern crate gl;
extern crate glutin;

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(r#"Darking Engine"#)
        .with_dimensions(1024, 768)
        .with_vsync()
        .build(&events_loop)
        .unwrap();

    unsafe {
        window.make_current().unwrap()
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.6, 0.6, 0.6, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event: glutin::WindowEvent::Closed, .. } => {
                    running = false;
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }
}
