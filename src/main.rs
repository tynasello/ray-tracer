use std::sync::Arc;
use rand::Rng;

use raytracer::{
    color::Color, light::LightSource, linalg::Vec3d, object::{Material, Object, RectangularPrism, Sphere}, Renderer, Scene
};

fn main() {
    let mut rng = rand::rng();

    let mut scenes = vec![
        Scene::new (
            Vec3d::new(0.0, 2.0, -1.0),
            Color::Black as usize,
            vec![
                LightSource::Ambient { intensity: 0.1 },
                LightSource::Point { intensity: 0.9, pos: Vec3d::new(-3.0, 4.0, -6.0) },
            ],
            vec![
                Box::new( 
                    RectangularPrism::new (
                        Vec3d::new(-40.0, 0.0, -40.0),
                        80.0,
                        -5.0,
                        50.0,
                        Color::Gray as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.3 }
                    )
                ),
                Box::new(
                    Sphere::new(
                        Vec3d::new(3.0, 2.0, -8.0),
                        2.0,
                        Color::Red as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    Sphere::new(
                        Vec3d::new(3.0, 0.5, -5.0),
                        0.5,
                        Color::Green as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),

                Box::new(
                    Sphere::new(
                        Vec3d::new(1.5, 0.4, -5.0),
                        0.4,
                        Color::Purple as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    Sphere::new(
                        Vec3d::new(-0.3, 0.7, -6.0),
                        0.7,
                        Color::SlateGray as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    Sphere::new(
                        Vec3d::new(-2.5, 1.0, -5.0),
                        1.0,
                        Color::Blue as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    Sphere::new(
                        Vec3d::new(-2.5, 1.2, -9.0),
                        1.2,
                        Color::Pink as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    RectangularPrism::new(
                        Vec3d::new(1.0, 0.0, -7.5),
                        0.5,
                        0.5,
                        0.5,
                        Color::SeaGreen as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
                Box::new(
                    RectangularPrism::new(
                        Vec3d::new(1.0, 0.0, -15.0),
                        3.0,
                        3.0,
                        3.0,
                        Color::DarkViolet as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.1 }
                    )
                ),
            ]
        ),
        
        Scene::new (
            Vec3d::new(0.0, 3.0, 0.0),
            Color::Black as usize,
            vec![
                LightSource::Ambient { intensity: 0.1 },
                LightSource::Point { intensity: 0.9, pos: Vec3d::new(0.0, 4.0, 0.0) },
            ],
            vec![
                Box::new( 
                    RectangularPrism::new (
                        Vec3d::new(-400.0, 0.0, -400.0),
                        800.0,
                        -5.0,
                        800.0,
                        Color::Gray as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.4 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(0.0, 1.0, -6.0),
                        1.0,
                        Color::DarkRed as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.2 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(2.0, 1.3, -9.0),
                        1.3,
                        Color::DarkOrange as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.2 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(-3.0, 3.0, -11.0),
                        3.0,
                        Color::Black as usize,
                        Material::Shiny { spclr_exp: 500.0, refl_rat: 0.7 }
                    )
                ),
            ]
        ),
        
        Scene::new (
            Vec3d::new(0.0, 1.5, -1.5),
            Color::Black as usize,
            vec![
                LightSource::Ambient { intensity: 0.1 },
                LightSource::Point { intensity: 0.4, pos: Vec3d::new(0.0, 1.0, -10.0) },
                LightSource::Directional { intensity: 0.4, dir: Vec3d::new(1.0, -1.0, -1.0) },
            ],
            vec![
                Box::new( 
                    RectangularPrism::new (
                        Vec3d::new(-400.0, 0.0, -400.0),
                        800.0,
                        -5.0,
                        800.0,
                        Color::WhiteSmoke as usize,
                        Material::Matte
                    )
                ),
                Box::new( 
                    RectangularPrism::new (
                        Vec3d::new(-400.0, 0.0, -15.0),
                        800.0,
                        40.0,
                        1.0,
                        Color::WhiteSmoke as usize,
                        Material::Matte
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(-1.5, 2.0, -6.0),
                        2.0,
                        Color::DeepPink as usize,
                        Material::Shiny { spclr_exp: 20.0, refl_rat: 0.2 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(3.0, 2.0, -8.0),
                        2.0,
                        Color::Pink as usize,
                        Material::Shiny { spclr_exp: 20.0, refl_rat: 0.8 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(0.5, 0.6, -4.0),
                        0.6,
                        Color::Teal as usize,
                        Material::Shiny { spclr_exp: 20.0, refl_rat: 0.0 }
                    )
                ),
                Box::new( 
                    Sphere::new (
                        Vec3d::new(2.0, 0.6, -5.5),
                        0.6,
                        Color::Pink as usize,
                        Material::Shiny { spclr_exp: 20.0, refl_rat: 0.0 }
                    )
                ),
            ]
        ),

        Scene::new (
            Vec3d::new(0.0, 1.0, 2.0),
            Color::Black as usize,
            vec![
                LightSource::Point { intensity: 1.0, pos: Vec3d::new(0.0, 10.0, -20.0) },
            ],
            (0..30).map(|_| {
                let radius = 1.0;
                
                let position = Vec3d::new(
                    rng.random_range(-15.0..15.0),
                    0.0 + radius,
                    rng.random_range(-30.0..0.0),
                );
                
                let material = if rng.random_bool(0.5) {
                    Material::Shiny { spclr_exp: rng.random_range(0.0..10.0), refl_rat: rng.random_range(0.0..0.7) }
                } else {
                    Material::Matte
                };
                
                Box::new(Sphere::new(position, radius, rng.random_range(0..0xFFFFFF), material)) as Box<dyn Object>
            })
            .chain(std::iter::once(
                Box::new( 
                    Sphere::new (
                        Vec3d::new(0.0, -5000.0, 0.0),
                        5000.0,
                        Color::Brown as usize,
                        Material::Matte
                    ) 
                ) as Box<dyn Object>
            ))
            .collect::<Vec<_>>()
        )
    ];

    /*
    
    Renderer

    */

    let mut renderer = Renderer::new (
        8,
        800,
        16.0 / 9.0, 
        1,
        Arc::new(scenes.swap_remove(0)),
        1
    );

    renderer.run();
}