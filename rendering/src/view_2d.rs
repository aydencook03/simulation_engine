//! Things that would be common to any 2d-renderer.
//!
//! This module contains the View2D object, which any renderer can use to easily keep track of things like
//! panning, zooming, background color, etc. It also contains methods to help map between the simulation's coordinates
//! to a coordinate on the renderer's window, which can be hard to implement manually due to the zoom and pan.
//!
//! # Coordinate Convention
//!
//! This module assumes that the (0, 0) coordinate is in the center, and that (+, +) is to the top right... ie,
//! it uses standard euclidean space.
//!
//! For example, if (0, 0) is in the top left of the window for your renderer, after getting the
//! transformed coordinates (using [`View2D::map_to_view`]), you would need to draw them as so:
//! ```rust
//! draw_point((x + width / 2.0), (height / 2.0 - y));
//! ```

use engine::math::Vec3;

//---------------------------------------------------------------------------------------------------//
// A useful object that can keep track of 2d camera panning and zooming.

/// A two dimensional view into the simulation.
#[derive(Copy, Clone)]
pub struct View2D {
    /// amount by which the view is offset from the (0, 0) coordinate in the simulation
    pub view_offset: Vec3,
    /// zoom parameter
    pub zoom: f64,
    /// amount by which panning increases the view offset
    pub pan_step: f64,
    /// amount by which zooming changes the zoom parameter
    pub zoom_step: f64,
}

impl View2D {
    /// Create a new default view.
    pub fn new() -> View2D {
        View2D {
            view_offset: Vec3::zero(),
            zoom: 1.0,
            pan_step: 20.0,
            zoom_step: 0.15,
        }
    }

    /// Reset the view.
    pub fn reset(&mut self) {
        self.view_offset = Vec3::zero();
        self.zoom = 1.0;
    }

    /// Used to map the zoom parameter to the actual zoom amount.
    ///
    /// It uses exp(zoom - 1.0). This is useful because the zoom amount should never become negative.
    pub fn parameterized_zoom(&self) -> f64 {
        std::f64::consts::E.powf(self.zoom - 1.0)
    }

    /// Pan the view to the right.
    pub fn pan_right(&mut self) {
        self.view_offset.x += self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view to the left.
    pub fn pan_left(&mut self) {
        self.view_offset.x -= self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view up.
    pub fn pan_up(&mut self) {
        self.view_offset.y += self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view down.
    pub fn pan_down(&mut self) {
        self.view_offset.y -= self.pan_step / self.parameterized_zoom();
    }

    /// Zoom the view in.
    pub fn zoom_in(&mut self) {
        self.zoom += self.zoom_step;
    }

    /// Zoom the view out.
    pub fn zoom_out(&mut self) {
        self.zoom -= self.zoom_step;
    }

    /// Maps a circle and its properties to the transformed (panned, zoomed) view space.
    ///
    /// The 2d view into the simulation is likely to be panned around or zoomed in and out, so this function
    /// maps a set of coordinates in the simulation space to what they would be on the panned and zoomed view.
    pub fn map_to_view(&self, pos: Vec3, radius: f64) -> (Vec3, f64) {
        let zoom = self.parameterized_zoom();
        let vec = zoom * (pos - self.view_offset);
        let radius = radius * zoom;

        (vec, radius)
    }
}

impl Default for View2D {
    fn default() -> Self {
        Self::new()
    }
}

//---------------------------------------------------------------------------------------------------//
