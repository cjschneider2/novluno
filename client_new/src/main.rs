use std::time::Instant;
use glow::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("novluno")
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    // crete and configure imgui
    let mut imgui = imgui::Context::create();
    let mut last_frame = Instant::now();

    // create platform handler
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {

            Event::NewEvents(_) => {
                last_frame = imgui.io_mut().update_delta_time(last_frame);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::MainEventsCleared => {
                platform.prepare_frame(imgui.io_mut(), &window).unwrap();
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
                // app-render: draw stuff here that's under the UI
                platform.prepare_render(&ui, &window);
                {
                    ui.window("Hello world")
                        .size([300.0, 100.0], Condition::FirstUseEver)
                        .build(|| {
                            ui.text("Hello world!");
                            ui.text("こんにちは世界！");
                            ui.text("This...is...imgui-rs!");
                            ui.separator();
                            let mouse_pos = ui.io().mouse_pos;
                            ui.text(format!(
                                "Mouse Position: ({:.1},{:.1})",
                                mouse_pos[0], mouse_pos[1]
                            ));
                        });
                }
                let draw_data = ui.render();
                // app-render: draw stuff here that's over the UI
            }
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            },
        }
    })
}
