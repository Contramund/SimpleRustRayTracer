pub mod III_vector {
    use std::ops::{Mul, Add, Sub, Div};

    #[derive(Clone,Copy)]
    pub struct Vec3<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Vec3<T> {
        pub fn new(a: T, b: T, c: T) -> Vec3<T> {
            Vec3::<T> {
                x: a,
                y: b,
                z: c,
            }
        }
    }

    impl<T: Add<Output=T>> Add for Vec3<T> {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    
    impl<T: Sub<Output=T>> Sub for Vec3<T> {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl<T: Add<Output=T> + Mul<Output=T>> Mul for Vec3<T> {
        type Output = T;
        fn mul(self, rhs: Self) -> T {
            self.x * rhs.x + 
            self.y * rhs.y + 
            self.z * rhs.z
        }
    }


    impl<T: Copy + Sub<Output=T> + Mul<Output=T>> Vec3<T> {
        pub fn prod(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
            Vec3::<T> {
                x: a.y*b.z - a.z*b.y, 
                y: a.z*b.x - a.x*b.z,
                z: a.x*b.y - a.y*b.z,
            }
        }
    }

    impl<T: Copy + Mul<Output=T>> Vec3<T> {
        pub fn scale(self, rhs: T) -> Self {
            Vec3::<T> {
                x: self.x * rhs, 
                y: self.y * rhs, 
                z: self.z * rhs, 
            }
        }
    }

    impl<T> Vec3<T>
        where T: num::Float {
            pub fn len(self) -> T {
                (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
            }
    }

    impl<T> Vec3<T>
        where T: Copy + Add<Output=T> + Mul<Output=T> +
            Div<Output=T> + num::Float {
        pub fn norm(self) -> Self {
            let n = self.len();
            Vec3::<T> {
                x: self.x/n, 
                y: self.y/n,
                z: self.z/n,
            }
        }
    }

    impl<T: Copy + Eq + num::Zero> num::Zero for Vec3<T> {
        fn zero() -> Self {
            Vec3::<T>::new(num::zero(), num::zero(), num::zero(),)
        }

        fn is_zero(&self) -> bool {
            self.x + self.y + self.z == num::zero()
        }
    }

    #[derive(Clone, Copy)]
    pub struct Ray<T>{
        pub root: Vec3<T>,
        pub dir: Vec3<T>,
    }
}

pub mod PPM {
    use std::io::Write;
    use std::fs::OpenOptions;

    #[derive(Clone, Copy)]
    pub struct RGB {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl RGB {
        pub fn new(rr: u8, gg: u8, bb: u8) -> RGB {
            RGB {
                r: rr,
                g: gg,
                b: bb,
            }
        }
    }
 
    pub struct PPM {
        height: u32,
        width: u32,
        data: Vec<u8>,
    }
 
    impl PPM {
        pub fn new(h: u32, w: u32) -> PPM {
            let size = 3 * h * w;
            let buf = vec![0; size as usize];
            PPM { 
                height: h,
                width: w,
                data: buf,
            }
        }
 
        fn buffer_size(&self) -> u32 {
            3 * self.height * self.width
        }
 
        fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
            let offset = (y * self.width * 3) + (x * 3);
            if offset < self.buffer_size() {
                Some(offset as usize)
            } else {
                None
            }
        }
 
        pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
            match self.get_offset(x, y) {
                Some(offset) => {
                    self.data[offset] = color.r;
                    self.data[offset + 1] = color.g;
                    self.data[offset + 2] = color.b;
                    true
                },
                None => false
            }
        }

        pub fn save_file(&mut self) {
            let mut picture_number = 0;
            let mut file = loop {
                if let Ok(i) = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(format!("Picture{}.ppm", picture_number)) {
                        println!("Output will be stored in \"Picture{}\".ppm",
                            picture_number);
                        break i
                } else {
                    picture_number += 1;
                }
            };
    
            file.write(format!("P6 {} {} 255\n", self.height, self.width).as_bytes()).unwrap();
            if let Ok(a) = file.write(&self.data) {
                println!("Done --- {}", a);
            } else {
                println!("Ooops...");
            }
        }
    }
}

pub mod objects {
    use super::III_vector::{Vec3, Ray}; 
    use super::PPM::RGB;

    #[derive(Clone, Copy)]
    pub enum SurfaceType{
        Solid(RGB),
        Mirror,
        Transparent(f32),
    }

    #[derive(Clone, Copy)]
    pub struct SurfaceOptions {
        spec_ref: f32,
        diff_ref: f32,
        amb_ref: f32, 
        shininess: f32,
        surface_type: SurfaceType,
    }

    impl SurfaceOptions {
        pub fn new(sp: f32, df: f32, amb: f32, sh: f32, so: SurfaceType) -> SurfaceOptions {
            SurfaceOptions {
                spec_ref: sp,
                diff_ref: df,
                amb_ref : amb,
                shininess: sh,
                surface_type: so,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Sphere {
        center: Vec3<f32>,
        radius: f32,
        opt: SurfaceOptions,
    }

    impl Sphere {
        pub fn new(c: Vec3<f32>, r: f32, o: SurfaceOptions) -> Option<Sphere> {
            if r*r > num::zero() {
                return Some(Sphere{
                    center: c,
                    radius: r,
                    opt: o,
                })
            }
            else {
                return None;
            }
        }
    }

    impl Sphere {
        pub fn ray_intersect(self, root: Vec3<f32>, dir: Vec3<f32> ) -> Option<f32> {
            let new_sphere_center = self.center - root; 
            let norm_dir = dir.norm();
            let scal = norm_dir * new_sphere_center;
            let proj = norm_dir.scale(scal);
            let h = new_sphere_center - proj;

            if new_sphere_center*new_sphere_center <= self.radius*self.radius || scal <= 0.0 {
                return None;
            }

            if h*h <= self.radius * self.radius {
                return Some(proj.len() - (self.radius*self.radius - h*h).sqrt());
            } else {
                return None;
            }
        }

        // get the ray from the point on the sphere and returns the outgoing ray
        pub fn dir_passed_transparent_sphere(&self, ray: Ray<f32> ) -> Option<Ray<f32>> {
            if let SurfaceType::Transparent(coeff) = self.opt.surface_type {
                let n_norm = (self.center - ray.root).norm();
                let proj = n_norm * ray.dir.norm(); // it is also cos(\a)
                let neg_dir = n_norm.scale(2.0*proj) - ray.dir.norm();
                let sin_a = (1.0 - proj*proj).sqrt(); // suppose 0 < a < 90 
                let sin_b = sin_a * coeff; // suppose 0 < b < 90
                if sin_b >= 1.0 { // if cannot pass into, so reflect
                    return Some(Ray{
                        root: ray.root,
                        dir: n_norm.scale(-2.0*proj) + ray.dir.norm(),
                    });
                }
                let cos_b = (1.0 - sin_b*sin_b).sqrt();
                let tg_2b = (2.0 * sin_b * cos_b) / (cos_b*cos_b - sin_b*sin_b);
                let dir_to_n = n_norm - neg_dir.scale(proj);
                let outgoing_ray = neg_dir.scale(tg_2b.signum()) + dir_to_n.norm().scale(tg_2b.abs());
                let n_to_ray = ray.dir.norm() - n_norm.scale(proj);
                let outgoing_point = self.center + (n_norm.scale(tg_2b.signum()) +
                    n_to_ray.norm().scale(tg_2b.abs())).norm().scale(self.radius);

                return Some(Ray{
                    root: outgoing_point,
                    dir: outgoing_ray,
                });
            } else {
                return None;
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct LightOptions {
        spec_ref: f32,
        diff_ref: f32,
        amb_ref: f32, 
    }

    impl LightOptions {
        pub fn new(sp: f32, df: f32, amb: f32) -> LightOptions {
            LightOptions {
                spec_ref: sp,
                diff_ref: df,
                amb_ref : amb,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Light {
       center: Vec3<f32>,
       opt: LightOptions,
    }

    impl Light {
        pub fn new(c: Vec3<f32>, o: LightOptions) -> Light{
            Light {
                center: c,
                opt: o,
            }
        }
    }

    pub struct Scene {
        obj: Vec<Sphere>,
        lights: Vec<Light>,
    }

    impl Scene {
        pub fn new() -> Scene {
            let v = Vec::<Sphere>::new();
            let l = Vec::<Light>::new();
            Scene{obj: v, lights: l}
        }

        pub fn add_obj(&mut self, s: Sphere) {
            self.obj.push(s);
        }

        pub fn add_light(&mut self, l: Light) {
            self.lights.push(l);
        }
    }

    impl Scene {
        pub fn ind_viewing(&self, root: Vec3<f32>, dir: Vec3<f32> ) -> Option<usize> {
            if self.obj.is_empty(){ return None; }
            let mut nearest_ind = 0 as usize;
            let mut nearest_dist : Option<f32> = None;

            for ind in 0..self.obj.len() {
                if let Some(d) = self.obj[ind].ray_intersect(root, dir) {
                    if let Some(d_orig) = nearest_dist {
                        if d < d_orig {
                            nearest_ind = ind;
                            nearest_dist = Some(d);
                        }
                    } else {
                        nearest_ind = ind;
                        nearest_dist = Some(d);
                    }
                }
            }
                    
            match nearest_dist {
                Some(_) => Some(nearest_ind),
                None => None,
            }
        }
    }

    impl Scene{
        pub fn color_on_dir(&self, root: Vec3<f32>, dir: Vec3<f32>) -> Option<RGB> {
            // find the object we're looking at 
            if self.obj.is_empty(){ return None; }
            let mut nearest_ind = 0 as usize;
            let mut nearest_dist : Option<f32> = None;

            for ind in 0..self.obj.len() {
                if let Some(d) = self.obj[ind].ray_intersect(root, dir) {
                    if let Some(d_orig) = nearest_dist {
                        if d < d_orig {
                            nearest_ind = ind;
                            nearest_dist = Some(d);
                        }
                    } else {
                        nearest_ind = ind;
                        nearest_dist = Some(d);
                    }
                }
            }

            // handle founded object
            let s: Sphere;
            if let Some(dist) = nearest_dist {
                s = self.obj[nearest_ind];
                let mut p = root + dir.norm().scale(dist);
                let mut bright: f32 = 0.0;

                // evaluate the color of point on founded sphere
                for ind in 0..self.lights.len() {
                    let l = self.lights[ind];

                    // ambient light
                    bright += l.opt.amb_ref * s.opt.amb_ref;

                    // check if there is an sphere obscuring the light
                    if let Some(obstruct) = self.ind_viewing(l.center, p - l.center + (s.center - p).scale(0.001)) {
                        // problem of accuracy : if the point located on the edge 
                        // of visibility of some light origin then there is a chance,
                        // that ind.vieving() will 'miss' and return as the aim-surface
                        // the wrong one. 
                        if obstruct != nearest_ind { 
                            continue;
                        }
                    }

                    let scal = (p-l.center).norm() * (s.center - p).norm();
                    if let SurfaceType::Solid(_) = s.opt.surface_type { // suppose the surface being ideal mirror or ideal linse
                        if scal > 0.0 {
                            bright += l.opt.diff_ref * scal * s.opt.diff_ref;

                            debug_assert!(l.opt.diff_ref * scal * s.opt.diff_ref >= 0.0,
                                    "Diff light : \n\tl.diff_ref = {},\n\tscal = {},\n\ts.diff_ref = {}\nIn total gave {} < 0",
                                    l.opt.diff_ref, scal, s.opt.diff_ref, l.opt.diff_ref * scal * s.opt.diff_ref);
                        }
                    }

                    let v_refl = (s.center-p).norm().scale(2.0*scal) - (p-l.center).norm();
                    let spec_proj = v_refl.norm() * dir.norm();
                    if spec_proj > 0.0 {
                        bright += l.opt.spec_ref * spec_proj.powf(s.opt.shininess) * s.opt.spec_ref;
                        debug_assert!(l.opt.spec_ref * spec_proj.powf(s.opt.shininess) * s.opt.spec_ref >= 0.0, 
                                "Spec light:\n\tl.spec_ref = {},\n\tspec_proj = {},\n\ts.shininess = {},\n\ts.spec_ref ={}\nIn total gave {} < 0",
                                l.opt.spec_ref, spec_proj, s.opt.shininess, s.opt.spec_ref, l.opt.spec_ref * spec_proj.powf(s.opt.shininess) * s.opt.spec_ref);
                    }
                }

                // init the foreground color 
                let mut r = 10;
                let mut g = 10;
                let mut b = 10;

                match s.opt.surface_type {
                    SurfaceType::Solid(s_color) => {
                        r = (s_color.r as f32 * bright/255.0).floor() as u32;
                        g = (s_color.g as f32 * bright/255.0).floor() as u32;
                        b = (s_color.b as f32 * bright/255.0).floor() as u32;
                        if r > 255 { r = 255; }
                        if g > 255 { g = 255; }
                        if b > 255 { b = 255; }
                    }

                    SurfaceType::Mirror => {
                        let mut norm = dir.norm() * (s.center-p).norm();
                        let mut next_dir = (p-s.center).norm().scale(2.0*norm) + dir.norm();
                        let mut origin = true;

                        // let suppose the 4th reflection being miserably small
                        for mut mirrors_on_the_way in 0..4{
                            if let Some(next_ind) = self.ind_viewing(p, next_dir) {
                                match self.obj[next_ind].opt.surface_type {
                                    SurfaceType::Solid(_) => {
                                        let seing = self.color_on_dir(p, next_dir).unwrap();
                                        r = seing.r as u32;
                                        g = seing.g as u32;
                                        b = seing.b as u32;
                                        break;
                                    }

                                    SurfaceType::Mirror => {
                                        let next_sphere_center = self.obj[next_ind].center - p; 
                                        let norm_dir = next_dir.norm();
                                        let proj = norm_dir.scale(norm_dir * next_sphere_center);
                                        let h = next_sphere_center - proj;
                                        p = p + norm_dir.scale(proj.len() - (self.obj[next_ind].radius.powi(2) - h*h).sqrt());
                                        norm = next_dir.norm() * (self.obj[next_ind].center - p).norm();
                                        next_dir = (p - self.obj[next_ind].center).norm().scale(2.0*norm) + next_dir.norm();
                                    }

                                    SurfaceType::Transparent(_) => {
                                        let next_sphere_center = self.obj[next_ind].center - p; 
                                        let norm_dir = next_dir.norm();
                                        let proj = norm_dir.scale(norm_dir * next_sphere_center);
                                        let h = next_sphere_center - proj;
                                        p = p + norm_dir.scale(proj.len() - (self.obj[next_ind].radius.powi(2) - h*h).sqrt());
                                        let ray = self.obj[next_ind].dir_passed_transparent_sphere(Ray{
                                            root: p,
                                            dir : next_dir,
                                        }).unwrap();
                                        next_dir = ray.dir;
                                        p = ray.root;
                                        mirrors_on_the_way -= 1;
                                    }
                                }
                            } else {
                                if origin {
                                    // the infinity's color on the mirror and transparent surfaces
                                    r = 5;
                                    g = 5;
                                    b = 5;
                                } else {
                                    // the color of infinity of reflrctions
                                    r = 15;
                                    g = 15;
                                    b = 15;
                                }
                                break; 
                            }
                            origin = false;
                        }
                        bright += 255.0;
                        r = (r as f32 * bright/255.0).floor() as u32;
                        g = (g as f32 * bright/255.0).floor() as u32;
                        b = (b as f32 * bright/255.0).floor() as u32;
                        if r > 255 { r = 255; }
                        if g > 255 { g = 255; }
                        if b > 255 { b = 255; }
                    }

                    SurfaceType::Transparent(_) => {
                        let ray = s.dir_passed_transparent_sphere(Ray{
                            root: p,
                            dir: dir }).unwrap();
                        if let Some(color) = self.color_on_dir(ray.root, ray.dir) {
                            r = color.r as u32;
                            g = color.g as u32;
                            b = color.b as u32;
                        } else {
                            r = 15;
                            g = 15;
                            b = 15;
                        }

                        bright += 255.0;
                        r = (r as f32 * bright/255.0).floor() as u32;
                        g = (g as f32 * bright/255.0).floor() as u32;
                        b = (b as f32 * bright/255.0).floor() as u32;
                        if r > 255 { r = 255; }
                        if g > 255 { g = 255; }
                        if b > 255 { b = 255; }
                    }
                }
                return Some(RGB::new( r as u8, g as u8, b as u8));
            } else {
                return None;
            }
        }
    }
}
