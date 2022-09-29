//! Things that would be common to any 2d-renderer as well as components needed for a drawable object
//! and some nice colors.
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

use crate::vec2::Vec2;

//---------------------------------------------------------------------------------------------------//
// Some nice colors.

pub mod colors {
    pub const WHITE: [u8; 4] = [255, 255, 255, 255];
    pub const BLACK: [u8; 4] = [0, 0, 0, 255];
    pub const GREY: [u8; 4] = [40, 40, 40, 255];
    pub const CRIMSON: [u8; 4] = [220, 20, 60, 255];
    pub const EARTH_BLUE: [u8; 4] = [10, 30, 220, 255];
    pub const FOREST_GREEN: [u8; 4] = [1, 79, 55, 255];
}

//---------------------------------------------------------------------------------------------------//
// A useful object that can keep track of 2d camera panning and zooming.

/// A two dimensional view into the simulation.
#[derive(Copy, Clone)]
pub struct View2D {
    /// amount by which the view is offset from the (0, 0) coordinate in the simulation
    pub view_offset: Vec2,
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
            view_offset: Vec2::zero(),
            zoom: 1.0,
            pan_step: 20.0,
            zoom_step: 0.15,
        }
    }

    /// Reset the view.
    pub fn reset(self: &mut Self) {
        self.view_offset = Vec2::zero();
        self.zoom = 1.0;
    }

    /// Used to map the zoom parameter to the actual zoom amount.
    ///
    /// It uses exp(zoom - 1.0). This is useful because the zoom amount should never become negative.
    pub fn parameterized_zoom(self: &Self) -> f64 {
        std::f64::consts::E.powf(self.zoom - 1.0)
    }

    /// Pan the view to the right.
    pub fn pan_right(self: &mut Self) {
        self.view_offset.x += self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view to the left.
    pub fn pan_left(self: &mut Self) {
        self.view_offset.x -= self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view up.
    pub fn pan_up(self: &mut Self) {
        self.view_offset.y += self.pan_step / self.parameterized_zoom();
    }

    /// Pan the view down.
    pub fn pan_down(self: &mut Self) {
        self.view_offset.y -= self.pan_step / self.parameterized_zoom();
    }

    /// Zoom the view in.
    pub fn zoom_in(self: &mut Self) {
        self.zoom += self.zoom_step;
    }

    /// Zoom the view out.
    pub fn zoom_out(self: &mut Self) {
        self.zoom -= self.zoom_step;
    }

    /// Maps a circle and its properties to the transformed (panned, zoomed) view space.
    ///
    /// The 2d view into the simulation is likely to be panned around or zoomed in and out, so this function
    /// maps a set of coordinates in the simulation space to what they would be on the panned and zoomed view.
    pub fn map_to_view(self: &Self, pos: Vec2, radius: f64) -> (Vec2, f64) {
        let zoom = self.parameterized_zoom();
        // create affine transformation data
        let identity = [[1.0, 0.0], [0.0, 1.0]];
        let scale = [[zoom, 0.0], [0.0, zoom]];
        let pan = self.view_offset * -1.0;
        let vec = pos
            .affine_transformation(identity, pan)
            .affine_transformation(scale, Vec2::zero());
        let radius = radius * zoom;

        (vec, radius)
    }
}

//---------------------------------------------------------------------------------------------------//
