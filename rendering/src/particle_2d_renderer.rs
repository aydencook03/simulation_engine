//! This is an example implementation of a 2d renderer for the physics engine.
//! It is pretty simple, and is mainly used to test the engine.
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
//! |  Key   |   Action     |
//! |--------|--------------|
//! | Arrows | Pan Around   |
//! | +/-    | Zoom In/Out  |
//! | Enter  | Reset View   |
//! | Space  | Pause/Play   |
//! | F      | Step Forward |
//! | S      | Save Image   |
//! | Q      | Quit         |

//---------------------------------------------------------------------------------------------------//

use crate::{colors::Color, view_2d::View2D};
use engine::{math::Vec3, system::System};

use std::{collections::HashMap, time::Instant};

use winit::{
    dpi::PhysicalSize, //LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::GraphicsContext;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

//---------------------------------------------------------------------------------------------------//

pub struct Particle2DRenderer {
    pub save_animation: bool,
    pub style: Style2D,
    pub scale: Scale2D,
    pub user_function: Option<fn(&mut Particle2DRenderer, &mut System)>,
}

pub struct Scale2D {
    pub physics_dt: f64,
    pub time_unit: (f64, String),
    pub pixel_distance: f64,
    pub starting_zoom: f64,
}

pub struct Style2D {
    pub stroke_size: f32,
    pub stroke_color: Color,
    pub bg_color: Color,
    pub starting_window_size: [u32; 2],
    pub group_colors: HashMap<u32, Color>,
}

//--------------------------------------------------------------------//

struct SoftbufferContext {
    view: View2D,
    context: GraphicsContext<Window>,
    frame: u32,
}

//---------------------------------------------------------------------------------------------------//

impl Particle2DRenderer {
    /// Creates a default window.
    pub fn new(
        user_function: Option<fn(&mut Particle2DRenderer, &mut System)>,
    ) -> Particle2DRenderer {
        let mut group_colors = HashMap::new();
        group_colors.insert(0, crate::colors::CRIMSON);
        Particle2DRenderer {
            save_animation: false,
            style: Style2D {
                stroke_size: 2.5,
                stroke_color: crate::colors::BLACK,
                bg_color: crate::colors::GREY,
                starting_window_size: [1000, 1000],
                group_colors,
            },
            scale: Scale2D {
                physics_dt: 1.0 / 120.0,
                time_unit: (1.0, "Seconds".to_string()),
                pixel_distance: 1.0,
                starting_zoom: 1.0,
            },
            user_function,
        }
    }

    /// Converts from an rgb color to the 32-bit binary format that softbuffer uses.
    ///
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    fn rgb_to_softbuffer(rgb: [u8; 3]) -> u32 {
        let [r, g, b] = rgb;
        let r = (r as u32) << 16;
        let g = (g as u32) << 8;
        let b = b as u32; // << 0;

        r | g | b
    }

    pub fn run(mut self, mut system: System) {
        let event_loop = EventLoop::new();
        let window = {
            let size = PhysicalSize::new(
                self.style.starting_window_size[0],
                self.style.starting_window_size[1],
            );
            WindowBuilder::new()
                .with_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        let mut context = SoftbufferContext {
            view: View2D::new(),
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
            frame: 0,
        };
        context.view.zoom = self.scale.starting_zoom;

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
                    VirtualKeyCode::Space => system.running = !system.running,
                    VirtualKeyCode::F => {
                        if !system.running {
                            system.running = true;
                            system.step_forward(self.scale.physics_dt);
                            system.running = false;
                        }
                    }
                    VirtualKeyCode::R => todo!(),
                    VirtualKeyCode::S => todo!(),
                    VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    if let Some(function) = self.user_function {
                        function(&mut self, &mut system);
                    }
                    self.render_particles(&mut context, &system);
                    system.step_forward(self.scale.physics_dt);

                    let passed_sec = (time.elapsed().as_micros() as f64) * 10_f64.powi(-6);
                    context.context.window_mut().set_title(&format!(
                        "Simulation - fps: {:.0} - time: {:.2} {}",
                        1.0 / passed_sec,
                        system.time / self.scale.time_unit.0,
                        self.scale.time_unit.1,
                    ));

                    time = Instant::now();
                }
                _ => (),
            };
        });
    }

    //--------------------------------------------------------------------//

    fn render_particles(&self, context: &mut SoftbufferContext, system: &System) {
        // create particle style
        let mut particle_style = Paint {
            anti_alias: true,
            ..Default::default()
        };

        // create stroke styles
        let mut stroke_style = Paint {
            anti_alias: true,
            ..Default::default()
        };
        stroke_style.set_color_rgba8(
            self.style.stroke_color[0],
            self.style.stroke_color[1],
            self.style.stroke_color[2],
            self.style.stroke_color[3],
        );
        let mut stroke = Stroke::default();

        //--------------------------------------------------------------------//

        // get window width, height, and zoom info
        let width = context.context.window().inner_size().width as f64;
        let height = context.context.window().inner_size().width as f64;
        stroke.width = self.style.stroke_size * (context.view.parameterized_zoom() as f32);

        // create buffer
        let mut draw_buffer = Pixmap::new(width as u32, height as u32).unwrap();

        // paint the background
        draw_buffer.fill(tiny_skia::Color::from_rgba8(
            self.style.bg_color[0],
            self.style.bg_color[1],
            self.style.bg_color[2],
            self.style.bg_color[3],
        ));

        //--------------------------------------------------------------------//

        let mut particles = Vec::new();
        for particle in &system.particles {
            particles.push((particle.pos, particle.group));
        }
        particles.sort_by(|a, b| a.0.z.partial_cmp(&b.0.z).unwrap());

        for (pos, group) in particles {
            let color = self.style.group_colors.get(&group).unwrap();
            // get particle position and radius mapped to window space
            let (Vec3 { x, y, z: _ }, radius) =
                context.view.map_to_view(pos, system.particle_radius);
            particle_style.set_color_rgba8(color[0], color[1], color[2], color[3]);

            let path = {
                let mut pb = PathBuilder::new();
                pb.push_circle(
                    (x + width / 2.0) as f32,
                    (height / 2.0 - y) as f32,
                    radius as f32,
                );
                /* pb.push_rect(
                    (x + width / 2.0 - radius) as f32,
                    (height / 2.0 - y - radius) as f32,
                    (2.0*radius) as f32,
                    (2.0*radius) as f32,
                ); */
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

        // tiny-skia = {version = "0.8", features = ["png-format"]}
        // draw_buffer.save_png(format!("./target/{:05}.png", context.frame)).expect("error");
        context.frame += 1;
        // imagemagick `$convert -delay 100/fps -loop 0 ./target/*.png ./media/name.gif`
        // rm ./target/*.png

        // convert the draw_buffer to the format that Softbuffer uses
        let framebuffer: Vec<u32> = draw_buffer
            .pixels()
            .iter()
            .map(|pixel| {
                Particle2DRenderer::rgb_to_softbuffer([pixel.red(), pixel.green(), pixel.blue()])
            })
            .collect();

        // write the contents of framebuffer to the window's framebuffer
        context
            .context
            .set_buffer(&framebuffer, width as u16, height as u16);
    }
}

impl Default for Particle2DRenderer {
    fn default() -> Self {
        Self::new(None)
    }
}

//---------------------------------------------------------------------------------------------------//
