use crate::{camera_3d::Camera3D, colors::Color};
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

struct RendererState {
    camera: Camera3D,
    context: GraphicsContext<Window>,
}

//---------------------------------------------------------------------------------------------------//

pub struct Style {
    pub stroke_size: f32,
    pub stroke_color: Color,
    pub bg_color: Color,
    pub starting_window_size: [u32; 2],
    pub group_colors: HashMap<u32, Color>,
}

pub struct Scale {
    pub physics_dt: f64,
    pub time_unit: (f64, String),
    pub pixel_distance: f64,
}

pub struct Particle3DRenderer {
    pub style: Style,
    pub scale: Scale,
}

//---------------------------------------------------------------------------------------------------//

impl Particle3DRenderer {
    /// Creates a default window.
    pub fn new() -> Particle3DRenderer {
        let mut group_colors = HashMap::new();
        group_colors.insert(0, crate::colors::CRIMSON);
        Particle3DRenderer {
            style: Style {
                stroke_size: 2.5,
                stroke_color: crate::colors::BLACK,
                bg_color: crate::colors::GREY,
                starting_window_size: [1000, 1000],
                group_colors,
            },
            scale: Scale {
                physics_dt: 1.0 / 120.0,
                time_unit: (1.0, "Seconds".to_string()),
                pixel_distance: 1.0,
            },
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

    pub fn run(self, mut system: System) {
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

        let mut renderer_state = RendererState {
            camera: Camera3D::new(),
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
                    VirtualKeyCode::Return => renderer_state.camera.reset(),
                    VirtualKeyCode::Space => system.running = !system.running,
                    VirtualKeyCode::R => todo!(),
                    VirtualKeyCode::S => {
                        if !system.running {
                            system.running = true;
                            system.step_forward(self.scale.physics_dt);
                            system.running = false;
                        }
                    }
                    VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.render_particles(&mut renderer_state, &system);
                    system.step_forward(self.scale.physics_dt);

                    //renderer_state.camera.pos += Vec3::new(0., 1.0, 0.);
                    //renderer_state.camera.focus_on_point(Vec3::zero());

                    let passed_sec = (time.elapsed().as_micros() as f64) * 10_f64.powi(-6);
                    renderer_state.context.window_mut().set_title(&format!(
                        "Simulation - fps: {:.0} - time: {:.2} {}",
                        1.0 / passed_sec,
                        system.time / self.scale.time_unit.0,
                        self.scale.time_unit.1
                    ));

                    time = Instant::now();
                }
                _ => (),
            };
        });
    }

    //--------------------------------------------------------------------//

    fn render_particles(&self, renderer_state: &mut RendererState, system: &System) {
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
        let stroke = Stroke::default();

        //--------------------------------------------------------------------//

        // get window width, height, and zoom info
        let width = renderer_state.context.window().inner_size().width as f64;
        let height = renderer_state.context.window().inner_size().width as f64;

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
        particles.sort_by(|a, b| {
            renderer_state
                .camera
                .dist_to_cam(a.0)
                .partial_cmp(&(-renderer_state.camera.dist_to_cam(b.0)))
                .unwrap()
        });
        particles.retain(|p| renderer_state.camera.dist_to_cam(p.0) > 0.0);

        for (pos, group) in particles {
            let color = self.style.group_colors.get(&group).unwrap();
            // get particle position and radius mapped to window space
            let (Vec3 { x, y, z: _ }, radius) = renderer_state
                .camera
                .perspective_sphere(pos, system.particle_radius);
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
            .iter()
            .map(|pixel| {
                Particle3DRenderer::rgb_to_softbuffer([pixel.red(), pixel.green(), pixel.blue()])
            })
            .collect();

        // write the contents of framebuffer to the window's framebuffer
        renderer_state
            .context
            .set_buffer(&framebuffer, width as u16, height as u16);
    }
}

impl Default for Particle3DRenderer {
    fn default() -> Self {
        Self::new()
    }
}

//---------------------------------------------------------------------------------------------------//
