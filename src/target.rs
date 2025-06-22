use crate::planet::*;
use macroquad::prelude::*;

pub struct Target<'a> {
    pub planet: &'a Planet,
}

// impl<'a> Target<'a> {
//     pub fn draw(&self, ) {
//         let half_thickness = thickness / 2.;
//         let gap_radius = radius * gap_percent;
// 
//         draw_line(x - radius - half_thickness, y - radius, x - gap_radius, y - radius, thickness, color);
//         draw_line(x - radius, y - radius - half_thickness, x - radius, y - gap_radius, thickness, color);
//         draw_line(x + radius + half_thickness, y - radius, x + gap_radius, y - radius, thickness, color);
//         draw_line(x + radius, y - radius - half_thickness, x + radius, y - gap_radius, thickness, color);
//         draw_line(x - radius - half_thickness, y + radius, x - gap_radius, y + radius, thickness, color);
//         draw_line(x - radius, y + radius + half_thickness, x - radius, y + gap_radius, thickness, color);
//         draw_line(x + radius + half_thickness, y + radius, x + gap_radius, y + radius, thickness, color);
//         draw_line(x + radius, y + radius + half_thickness, x + radius, y + gap_radius, thickness, color);
//     }
// }
