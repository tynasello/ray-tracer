pub mod color;
pub mod linalg;
pub mod object;
pub mod light;
pub mod utils;

use std::{f64::{EPSILON, INFINITY}, sync::{Arc, Mutex, RwLock}, thread};

use color::Color;
use linalg::{Mat3, Ray, Vec3d};
use object::{Material, Object, closest_intersection};
use light::LightSource;
use rand::Rng;
use utils::Range;

/*

Screen

The underlying structure supporting the drawable canvas

*/

struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize
}

impl Screen {
    fn build(screen_width: usize, screen_height: usize) -> Self {
        let mut window = minifb::Window::new(
            "Press ESC to exit",
            screen_width,
            screen_height,
            minifb::WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Unable to open window: {}", e);
        });
    
        window.set_target_fps(60);

        Self {
            window,
            buffer: vec![0; screen_width * screen_height],
            width: screen_width,
            height: screen_height
        }
    }

    fn render_buffer(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }
}

/*

Canvas

*/

struct Canvas {
    buffer: Mutex<Vec<Vec<usize>>>,
    width: usize, 
    height: usize
}

impl Canvas {
    fn new(screen_width: usize, screen_height: usize, canvas_unit_size: usize) -> Self {
        Self {
            buffer: Mutex::new(vec![vec![0; screen_width]; screen_height]),
            width: screen_width / canvas_unit_size,
            height: screen_height / canvas_unit_size,
        }
    }

    fn clear(&self) {
        let mut buffer = self.buffer.lock().unwrap(); 
        for row in buffer.iter_mut() {
            for p in row {
                *p = 0;
            }
        }
    }
}

/*

Camera / Viewport

*/

struct Camera {
    origin: Vec3d,      // The eye point. Rays are traced from this point.

    // The viewport is the rectangle through which the camera looks through, i.e. emits rays through
    vp_width: f64,
    vp_height: f64,
    vp_depth: isize,    // Depth of viewport location in z+ direction from camera. Absolute of this value is the focal length.

    y_rot: f64,         // Current horizontal rotation (deg)
    x_rot: f64,         // Current vertical rotation (deg)
    rot_m: Mat3         // Matrix holds camera transformations to apply on rays being traced
}

impl Camera {
    fn new(origin: Vec3d, aspect_ratio: f64) -> Self {
        let viewport_height = 1.0;
        Self {
            origin,
            vp_width: viewport_height * aspect_ratio,
            vp_height: viewport_height,
            vp_depth: viewport_height as isize * -1,
            y_rot: 0.0,
            x_rot: 0.0,
            rot_m: Mat3::identity()
        }
    }
}

/*

Scene

Describes the entities in 3D space
Positive directions are right in x, up in y, out of screen in z

*/

pub struct Scene {
    camera_origin: Vec3d,
    bg_col: usize,
    lights: Vec<LightSource>,
    objs: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new(camera_origin: Vec3d, bg_col: usize, lights: Vec<LightSource>, objs: Vec<Box<dyn Object>>) -> Self {
        Self {
            camera_origin,
            bg_col,
            lights,
            objs
        }
    }

    fn trace_ray(&self, ray: &Ray, t_range: &Range<f64>, ray_refl_limit: u32) -> usize {
        // Trace a ray and if we encounter an object, return its color
        // Check all points along the ray, where the ray at t is within a given range (inclusive)
        // Set a limit on the number of times a ray is aloud to reflect
    
        match closest_intersection(&self.objs, ray, t_range) {
            Some((obj, intxp)) => {               
                // Find the sum of the intensities of light contributed by all sources on the intersection point

                let mut direct_light_intensity = 0.0;

                // Light contributed by sources directly on object 

                for light in self.lights.iter() {
                    if let LightSource::Ambient { intensity } = light {
                        // Ambient source
                        direct_light_intensity += intensity;

                    } else {
                        // Point or directional source
                        let (intxp_light_dir, light_intensity, ) = if let LightSource::Point { intensity, pos } = light {
                            (pos - &intxp, *intensity)
                        } else if let LightSource::Directional { intensity, dir } = light {
                            (dir * -1.0, *intensity)
                        } else {
                            (Vec3d::new(0.0, 0.0, 0.0), 0.0)
                        };
                        
                        let intxp_light_ray = Ray::new (
                            intxp.clone(),
                            intxp_light_dir.clone()
                        );

                        // Check for objects that exist along the ray from the intersection point to the light source.
                        // If this is the case, the point is shadowed, and the source contributes no direct light.
                        if let LightSource::Point { intensity: _, pos } = light {
                            if let Some((_, shdw_intxp)) = closest_intersection(&self.objs, &intxp_light_ray, &Range{min: EPSILON * 1000000.0, max: INFINITY}) {
                                if (&intxp - &shdw_intxp).magnitude() < (&intxp - pos).magnitude() {
                                    continue;
                                }
                            }
                        } else if let LightSource::Directional { intensity: _, dir } = light {
                            let ray = Ray::new(intxp.clone(), dir * -1.0);
                            if let Some(_) = closest_intersection(&self.objs, &ray, &Range{min: EPSILON * 1000000.0, max: INFINITY}) {
                                continue;
                            }
                        }

                        // Get the normal vector of the object going through the intersection point. This method will be defined differently for every object type
                        if let Some(mut norm) = obj.get_normal(&intxp) {
                            
                            if &norm * &intxp_light_dir < 0.0 { // Ensure norm and ray from intersection point to light are in the same direction. Important to do this because of triangles.
                                norm = &norm * -1.0;
                            }

                            // Diffuse reflection
                            let n_dot_il: f64 = &norm * &intxp_light_dir;
                            if n_dot_il > 0.0 { // Don't account for lights behind surfaces (will have negative dot product)
                                direct_light_intensity += light_intensity * n_dot_il / (norm.magnitude() * intxp_light_dir.magnitude()); // cos(angle between norm and ray from intersection point to light source) * intensity
                            }

                            // Specular reflection
                            if let Material::Shiny { spclr_exp, refl_rat: _} = obj.get_material() {
                                let intxp_light_refl_dir = intxp_light_dir.reflect(&norm);
                                let intxp_o_dir = ray.origin() - &intxp;
                                let ilr_dot_io = &intxp_light_refl_dir * &intxp_o_dir;
                                if ilr_dot_io > 0.0 { // Don't account for lights when angle between reflected vector of intersection point to light source and intersection point to ray origin is > 90 (will have negative dot product)
                                    direct_light_intensity += light_intensity * (ilr_dot_io / (intxp_light_refl_dir.magnitude() * intxp_o_dir.magnitude())).powf(*spclr_exp); // cos (angle between reflected ray from intersection point to light source and vectory from intersection point to ray origin) ^ spec_exp * intensity
                                }
                            }
                        }
                    }
                }
                let direct_color = Color::scale(*obj.get_color() as usize, direct_light_intensity);
    
                // Light contributed by sources indirectly through reflections. Only shiny objects reflect light.

                match obj.get_material() {
                    Material::Shiny { spclr_exp: _, refl_rat } => {
                        if ray_refl_limit <= 0 || *refl_rat <= 0.0 {
                            return direct_color;
                        }
                        
                        if let Some(mut norm) = obj.get_normal(&intxp) {
                            if &norm * ray.dir() < 0.0 {
                                norm = &norm * -1.0;
                            }
                            
                            let refl_ray = Ray::new (
                                intxp,
                                (ray.dir() * -1.0).reflect(&norm)
                            );
                            
                            let reflected_color = self.trace_ray(&refl_ray, &Range{min: EPSILON * 1000000.0, max: t_range.max}, ray_refl_limit - 1);
                            
                            // Add direct and indirect colors
                            Color::add(Color::scale(direct_color, 1.0 - *refl_rat), Color::scale(reflected_color, *refl_rat))
                        } else {
                            direct_color
                        }
                    },
                    _ => direct_color
                }
            },

            _ => self.bg_col // No light along ray
        }
    }
}

/*

Ray Tracing 3D Renderer

*/

pub struct Renderer {
    screen: Screen,
    canvas: Arc<Canvas>,
    camera: Arc<RwLock<Camera>>,
    scene: Arc<Scene>,
    canvas_unit_size: usize, // The square length of pixels that a canvas unit will take up, e.g. a value of 2 means one canvas unit will take up a 2x2 square of pixels
    num_threads: usize,
    num_samples: usize, // Number of samples used when performing anti-aliasing
    rays: Arc<Vec<Vec<Ray>>>, // The rays that are traced into the scene
    thread_buffers: Vec<Arc<Mutex<Vec<Vec<usize>>>>> // The canvas is split into buffers for each thread to own and operate on
}

impl Renderer {
    pub fn new(num_threads: usize, screen_width: usize, aspect_ratio: f64, canvas_unit_size: usize, scene: Arc<Scene>, num_samples: usize) -> Self {
        let screen_height = (screen_width as f64 / aspect_ratio) as usize;

        if screen_width % canvas_unit_size != 0 || screen_height % canvas_unit_size != 0 {
            panic!("Window dimensions must be a multiple of pixel size")
        }

        let canvas = Canvas::new(screen_width, screen_height, canvas_unit_size);

        let camera = Camera::new(scene.camera_origin.clone(), screen_width as f64 / screen_height as f64);

        let rays = (0..canvas.height).map(|row|
                (0..canvas.width).map(|col|
                    Ray::new(
                        camera.origin.clone(),
                        Vec3d::new(
                            (col as isize - canvas.width as isize / 2) as f64 * camera.vp_width / canvas.width as f64,
                            (canvas.height as isize / 2 - row as isize) as f64 * camera.vp_height / canvas.height as f64,
                            camera.vp_depth as f64
                        )
                    )
                ).collect()
            ).collect();

        let thread_buffers = (0..num_threads).map(|_| 
            Arc::new(Mutex::new(vec![vec![0; canvas.width]; canvas.height]))
        ).collect();

        Self {
            camera: Arc::new(RwLock::new(camera)),
            scene,
            canvas: Arc::new(canvas),
            screen: Screen::build(screen_width, screen_height),
            canvas_unit_size,
            num_threads,
            num_samples,
            rays: Arc::new(rays),
            thread_buffers
        }
    }

    pub fn run(&mut self) {
        while self.screen.window.is_open() && !self.screen.window.is_key_down(minifb::Key::Escape) {
            self.update_camera();
            self.canvas.clear();
            self.trace_rays();
            self.render_canvas();
        }
    }

    fn update_camera(&self) {       
        let mut camera  = self.camera.write().unwrap(); 

        let x_speed = 0.3;
        let z_speed = 0.3;

        let y_rot_speed = 5.0;
        let x_rot_speed = 3.0;

        for key in self.screen.window.get_keys() {
            match key {
                
                // Move left, right, forward, backward
                minifb::Key::A => {
                    let step = &(&camera.rot_m * &Vec3d::new(-1.0, 0.0, 0.0)).normalize() * x_speed;
                    camera.origin = &camera.origin + &Vec3d::new(step.x(), 0.0, step.z());
                }
                minifb::Key::D => {
                    let step = &(&camera.rot_m * &Vec3d::new(1.0, 0.0, 0.0)).normalize() * x_speed;
                    camera.origin = &camera.origin + &Vec3d::new(step.x(), 0.0, step.z());
                }
                minifb::Key::W => {
                    let step = &(&camera.rot_m * &Vec3d::new(0.0, 0.0, -1.0)).normalize() * z_speed;
                    camera.origin = &camera.origin + &Vec3d::new(step.x(), 0.0, step.z());
                }
                minifb::Key::S => {
                    let step = &(&camera.rot_m * &Vec3d::new(0.0, 0.0, 1.0)).normalize() * z_speed;
                    camera.origin = &camera.origin + &Vec3d::new(step.x(), 0.0, step.z());
                }
                
                // Look left, right, up, down
                minifb::Key::Left => {
                    camera.y_rot += y_rot_speed;
                }
                minifb::Key::Right => {
                    camera.y_rot -= y_rot_speed;
                }
                minifb::Key::Up => {
                    camera.x_rot = (camera.x_rot + x_rot_speed).min(89.0);
                }
                minifb::Key::Down => {
                    camera.x_rot = (camera.x_rot - x_rot_speed).max(-35.0);
                }

                _ => {}
            }
        }

        let y_rot_matrix = Mat3::rotation_y(camera.y_rot);
        let x_rot_matrix = Mat3::rotation_matrix(&(&y_rot_matrix * &Vec3d::new(1.0, 0.0, 0.0)), camera.x_rot);
        camera.rot_m = &x_rot_matrix * &y_rot_matrix;
    }

    pub fn trace_rays(&self) {
        let mut handles = vec![];
        
        let chunk_size = self.canvas.height / self.num_threads; // Each thread renders this many rows
        
        for thread_i in 0..self.num_threads {
            let scene = Arc::clone(&self.scene);
            let canvas = Arc::clone(&self.canvas);
            let camera = Arc::clone(&self.camera);
            let rays = Arc::clone(&self.rays);
            let thread_buffer = Arc::clone(&self.thread_buffers[thread_i]);

            let row_start = (thread_i * chunk_size) as usize;
            let row_end = if thread_i == self.num_threads - 1 { canvas.height } else { row_start + chunk_size };

            let num_samples = self.num_samples;

            let handle = thread::spawn(move || {
                let camera = camera.read().unwrap();
                let mut thread_buffer = thread_buffer.lock().unwrap();
                let mut rng = rand::rng();

                // Render a canvas unit at (col, row)
                // Sample to perform anti-aliasing
                
                for row in row_start..row_end {
                    for col in 0..canvas.width {
                        let mut total_color = (0, 0, 0);

                        for _ in 0..num_samples {
                            let jitter_x: f64 = if num_samples > 1 {rng.random::<f64>() - 0.5} else {0.0};
                            let jitter_y: f64 = if num_samples > 1 {rng.random::<f64>() - 0.5} else {0.0};
                            
                            let ray = &rays[row][col];
                            
                            // Use rotation matrix to rotate each ray (gives effect of changing camera orientation)
                            // Add random jitter for anti-aliasing
                            
                            let transformed_ray = Ray::new(
                                camera.origin.clone(),
                                &camera.rot_m * &(ray.dir() + &(&Vec3d::new(jitter_x, jitter_y, 0.0) * 0.0005))
                            );
                            
                            let color = scene.trace_ray(
                                &transformed_ray, 
                                &Range{min: camera.vp_depth.abs() as f64, max: 100.0},
                                2
                            );

                            total_color.0 += Color::r(color);
                            total_color.1 += Color::g(color);
                            total_color.2 += Color::b(color);
                        }

                        thread_buffer[row][col] = (total_color.0 / num_samples).min(255) << 16 | (total_color.1 / num_samples).min(255) << 8 | (total_color.2 / num_samples).min(255);
                    }
                }

                // Merge thread buffers into canvas buffer

                let mut buffer = canvas.buffer.lock().unwrap();

                for row in row_start..row_end {
                    for col in 0..canvas.width {
                        buffer[row][col] = thread_buffer[row][col] as usize;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn render_canvas(&mut self) {
        let canvas_buffer = &self.canvas.buffer.lock().unwrap();

        for canvas_row in 0..self.canvas.height {
            for canvas_col in 0..self.canvas.width {
                let screen_row_start = canvas_row * self.canvas_unit_size;
                let screen_col_start = canvas_col * self.canvas_unit_size;
                for screen_row in screen_row_start .. screen_row_start + self.canvas_unit_size {
                    for screen_col in screen_col_start .. screen_col_start + self.canvas_unit_size {
                        self.screen.buffer[screen_row * self.screen.width + screen_col] = canvas_buffer[canvas_row][canvas_col] as u32;
                    }
                }
            }
        }
        
        self.screen.render_buffer();
    }
}