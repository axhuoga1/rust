use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Error> {
    // 创建窗口
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % size.width as usize) as u32;
                    let y = (i / size.width as usize) as u32;

                    // 设置像素值 (RGBA)
                    pixel.copy_from_slice(&[x as u8, y as u8, 128, 255]);
                }
                pixels.render().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
