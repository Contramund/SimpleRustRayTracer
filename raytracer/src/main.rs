mod lib;
use lib::{III_vector::Vec3, PPM::{PPM, RGB}, objects::{Sphere, Light, Scene, SurfaceOptions, LightOptions, SurfaceType}};

fn main() {
    let img_x = 800;
    let img_y = 800;

    let mut img = PPM::new(img_x, img_y);
    let root = Vec3::new(-1.0, 0.0, 0.0);
    
    // Scene init
    let mut scene = Scene::new();

    // SurfaceOptions :
    // [1] --- SPECULAR 
    // [2] --- DIFFUSE : 1.0 as usual, can make surface brighter at all 
    // [3] --- AMBIENT : 1.0 as usual, 0.0 if there's no light in the dark
    // [4] --- SHININESS : larger for mirror-like objects 
    // [5] --- SURFACE_TYPE : Solid(color), Mirror, Transparent(coefficent)
    
    // Spheres
    let s_c1 = Vec3::new(1.0, 0.3, 0.5);
    let col1 = RGB::new(77, 248, 255);
    let sopt1 = SurfaceOptions::new(1.3, 1.5, 1.0, 100.0, SurfaceType::Solid(col1));
    let s1 = Sphere::new(s_c1, 0.7, sopt1).unwrap();
    scene.add_obj(s1);

    let s_c2 = Vec3::new(1.0, -0.5, 0.25);
    let col2 = RGB::new(0, 255, 0);
    let sopt2 = SurfaceOptions::new(0.8, 1.5, 1.0, 2.0, SurfaceType::Solid(col2));
    let s2 = Sphere::new(s_c2, 0.5, sopt2).unwrap();
    scene.add_obj(s2);

    let s_c3 = Vec3::new(1.5, -0.0, -0.7);
    let sopt3 = SurfaceOptions::new(50.0, 1.0, 0.0, 100.0, SurfaceType::Mirror);
    let s3 = Sphere::new(s_c3, 0.5, sopt3).unwrap();
    scene.add_obj(s3);

    let s_c4 = Vec3::new(1.2, -1.7, -1.0);
    let sopt4 = SurfaceOptions::new(50.0, 1.0, 0.0, 100.0, SurfaceType::Mirror);
    let s4 = Sphere::new(s_c4, 0.8, sopt4).unwrap();
    scene.add_obj(s4);

    let s_c5 = Vec3::new(-0.1, 0.4, 0.2);
    let sopt5 = SurfaceOptions::new(50.0, 1.0, 0.0, 100.0, SurfaceType::Transparent(1.3));
    let s5 = Sphere::new(s_c5, 0.2, sopt5).unwrap();
    scene.add_obj(s5);

    // Lights
    let l_c1 = Vec3::new(-0.6, 0.8, 1.3);
    let lopt1 = LightOptions::new(70.0, 100.0, 5.0);
    let l1 = Light::new(l_c1, lopt1);
    scene.add_light(l1);

    let l_c2 = Vec3::new(-1.0, -0.7, 1.0);
    let lopt2 = LightOptions::new(60.0, 70.0, 5.0);
    let l2 = Light::new(l_c2, lopt2);
    scene.add_light(l2);

    // Render
    let x_range = 1.5;
    let y_range = 1.5;
    for x in 0..img_x {
        for y in 0..img_y {
            // let mut p = RGB::new((x*256/img_x) as u8, (y*256/img_y) as u8, 0);
            let mut p = RGB::new(10,10,10);
            if let Some(col) = scene.color_on_dir(root , Vec3::new(
                    1.0,
                    x_range - (2.0 * x_range * (x as f32) / (img_x as f32) ),
                    y_range - (2.0 * y_range * (y as f32) / (img_y as f32) ) 
                    )) { p = col } 
            img.set_pixel(x, y, p);
        }
    }

    // Save as ppm
    img.save_file();
}
