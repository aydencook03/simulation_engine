//! This is an example implementation renderer for the
//! [`rusty_physics_2d`](https://github.com/aydencook03/rusty_physics_engine) crate.
//! It is pretty simple, and is mainly used to test the 2d physics engine.
//!
//! It uses [`winit`](https://github.com/rust-windowing/winit) for the event_loop, window, and keyboard,
//! [`std::time`](https://doc.rust-lang.org/std/time/index.html) for timekeeping,
//! [`softbuffer`](https://github.com/john01dav/softbuffer) for accessing the window's framebuffer,
//! and [`tiny_skia`](https://github.com/RazrFalcon/tiny-skia) for turning shapes into
//! pixels (the rasterization algorithms).
//!
//!
//! # Controls:
//!
//! |  Key   |   Action    |
//! |--------|-------------|
//! | Arrows | Pan Around  |
//! | +/-    | Zoom In/Out |
//! | Enter  | Reset View  |
//! | Space  | Pause/Play  |
//! | R      | Reset Sim   |
//! | Q      | Quit        |

//---------------------------------------------------------------------------------------------------//

use rusty_physics_2d::{render_utils::View2D, system::System, vec2::Vec2};

use std::time::Instant;

use winit::{
    dpi::PhysicalSize, //LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::GraphicsContext;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub struct Renderer {
    pub render_fps: u32,
    pub physics_fps: u32,
    pub stroke_size: f32,
    pub stroke_color: [u8; 4],
    pub bg_color: [u8; 4],
    pub starting_width: u32,
    pub starting_height: u32,
}

struct Context {
    view: View2D,
    context: GraphicsContext<Window>,
}

impl Renderer {
    /// Creates a default window.
    pub fn new() -> Renderer {
        Renderer {
            render_fps: 30,
            physics_fps: 60,
            stroke_size: 2.5,
            stroke_color: rusty_physics_2d::render_utils::colors::BLACK,
            bg_color: rusty_physics_2d::render_utils::colors::GREY,
            starting_width: 1000,
            starting_height: 1000,
        }
    }

    /// Converts from an rgb color to the 32-bit binary format that softbuffer uses.
    ///
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    fn rgb_to_softbuffer(rgb: [u8; 3]) -> u32 {
        let [r, g, b] = rgb;
        let r = (r as u32) << 16;
        let g = (g as u32) << 8;
        let b = (b as u32) << 0;

        r | g | b
    }

    pub fn run(self: Self, mut system: System) {
        let event_loop = EventLoop::new();
        let window = {
            let size = PhysicalSize::new(self.starting_width, self.starting_height);
            WindowBuilder::new()
                .with_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        let mut context = Context {
            view: View2D::new(),
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
        };

        let mut time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // stop the event loop, and therefore close the window
                    *control_flow = ControlFlow::Exit;
                }
                Event::DeviceEvent {
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(code),
                            ..
                        }),
                    ..
                } => match code {
                    VirtualKeyCode::Left => context.view.pan_left(),
                    VirtualKeyCode::Right => context.view.pan_right(),
                    VirtualKeyCode::Up => context.view.pan_up(),
                    VirtualKeyCode::Down => context.view.pan_down(),
                    VirtualKeyCode::Equals => context.view.zoom_in(),
                    VirtualKeyCode::Minus => context.view.zoom_out(),
                    VirtualKeyCode::Return => context.view.reset(),
                    VirtualKeyCode::Space => system.pause_play(),
                    VirtualKeyCode::R => todo!(),
                    VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    render(&self, &mut context, &system);
                    let passed_sec = (time.elapsed().as_micros() as f64) * 10_f64.powi(-6);
                    context.context.window_mut().set_title(&format!(
                        "Simulation - fps: {:.0} - time: {:.2}",
                        1.0 / passed_sec,
                        system.time
                    ));
                    system.step_forward(passed_sec);
                    time = Instant::now();
                }
                _ => (),
            };
        });
    }
}

//---------------------------------------------------------------------------------------------------//

fn render(renderer: &Renderer, context: &mut Context, system: &System) {
    //--------------------------------------------------------------------//

    // create particle style
    let mut particle_style = Paint::default();
    particle_style.anti_alias = true;

    // create stroke styles
    let mut stroke_style = Paint::default();
    stroke_style.anti_alias = true;
    stroke_style.set_color_rgba8(
        renderer.stroke_color[0],
        renderer.stroke_color[1],
        renderer.stroke_color[2],
        renderer.stroke_color[3],
    );
    let mut stroke = Stroke::default();

    //--------------------------------------------------------------------//

    // get window width, height, and zoom info
    let width = context.context.window().inner_size().width as f64;
    let height = context.context.window().inner_size().width as f64;
    stroke.width = renderer.stroke_size * (context.view.parameterized_zoom() as f32);

    // create buffer
    let mut draw_buffer = Pixmap::new(width as u32, height as u32).unwrap();

    // paint the background
    draw_buffer.fill(tiny_skia::Color::from_rgba8(
        renderer.bg_color[0],
        renderer.bg_color[1],
        renderer.bg_color[2],
        renderer.bg_color[3],
    ));

    //--------------------------------------------------------------------//

    for particle in &system.particles {
        let color = rusty_physics_2d::render_utils::colors::CRIMSON;
        // get particle position and radius mapped to window space
        let (Vec2 { x, y }, radius) = context
            .view
            .map_to_view(particle.pos, system.particle_radius);
        particle_style.set_color_rgba8(color[0], color[1], color[2], color[3]);

        let path = {
            let mut pb = PathBuilder::new();
            pb.push_circle(
                (x + width / 2.0) as f32,
                (height / 2.0 - y) as f32,
                radius as f32,
            );
            pb.finish().unwrap()
        };

        // draw the particle outline
        draw_buffer.stroke_path(&path, &stroke_style, &stroke, Transform::identity(), None);

        // fill in the particle outline
        draw_buffer.fill_path(
            &path,
            &particle_style,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    //--------------------------------------------------------------------//

    // convert the draw_buffer to the format that Softbuffer uses
    let framebuffer: Vec<u32> = draw_buffer
        .pixels()
        .into_iter()
        .map(|pixel| Renderer::rgb_to_softbuffer([pixel.red(), pixel.green(), pixel.blue()]))
        .collect();

    // write the contents of framebuffer to the window's framebuffer
    context
        .context
        .set_buffer(&framebuffer, width as u16, height as u16);

    //--------------------------------------------------------------------//
}
