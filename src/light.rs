use crate::linalg::Vec3d;

pub enum LightSource {
    // A light source contributes some intensity of light (a fraction) to the scene
    // The sum of all light sources should equal to 1.0
    
    // In the real world, points in space are hit by scattered rays. 
    // To attempt to simulate this phenomena, we use an ambient source, which adds some light to every point
    Ambient { intensity: f64 },

    // Emit light equally in all directions from a position, e.g. a lightbulb
    Point { intensity: f64, pos: Vec3d },

    // Light travelling along any vector with a given direction. Every point in space can be struck by these rays
    // This type of source can model the sun's rays on the earth because of the large difference in size
    Directional { intensity: f64, dir: Vec3d },
}