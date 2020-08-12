use std::time::SystemTime;
use rayon::prelude::*;


// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, vec: Vec3) {
        *self = Self {x: self.x + vec.x, y: self.y + vec.y, z: self.z + vec.z}
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {x: self.x / scalar, y: self.y / scalar, z: self.z / scalar}
    }
}

impl Vec3 {
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    fn dot(&self, vec: Vec3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    fn cross(&self, vec: Vec3) -> Vec3 {
        Vec3{
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }

    fn write_color(mut self, sample_per_pixel: i32) {
        self /= sample_per_pixel as f32;
        let ir = (256.0 * clamp(self.x.sqrt(), 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(self.y.sqrt(), 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(self.z.sqrt(), 0.0, 0.999)) as i32;
        println!("{:?} {:?} {:?}", ir, ig, ib);
    }

    fn random(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: min + (max - min) * rand::random::<f32>(),
            y: min + (max - min) * rand::random::<f32>(),
            z: min + (max - min) * rand::random::<f32>(),
        }
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(- 1.0, 1.0);
            if p.length() < 1.0 {
                return p
            }
        }
    }
    fn random_unit_vector() -> Vec3 {
        let a: f32 = 2.0 * std::f32::consts::PI * rand::random::<f32>();
        let z: f32 = - 1.0 + 2.0 * rand::random::<f32>();
        let r: f32 = (1.0 - z *z).sqrt();
        Vec3{x: r * a.cos(), y: r * a.sin(), z: z}
    }
    fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3{
                x: - 1.0 + 2.0 * rand::random::<f32>(),
                y: - 1.0 + 2.0 * rand::random::<f32>(),
                z: 0.0,
            };
            if p.length() < 1.0 {
                return p
            }
        }
    }
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2.0 * v.dot(n)
        // v - (n * 2.0) * v.dot(n)
    }

    fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (- uv).dot(n);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * (- (1.0 - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min
    }
    else if val > max {
        return max
    }
    val
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, vec: Vec3) -> Vec3 {
        Vec3 {x: self.x + vec.x, y: self.y + vec.y, z: self.z + vec.z}
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3 {x: self.x - vec.x, y: self.y - vec.y, z: self.z - vec.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {x: self.x * vec.x, y: self.y * vec.y, z: self.z * vec.z}
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {x: self.x / scalar, y: self.y / scalar, z: self.z / scalar}
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {x: - self.x, y: - self.y, z: - self.z}
    }
}

// todo : implement sur ray ?
fn ray_color(r: &Ray, world: &World, depth: i32) -> Vec3 {
     // param : nb_ray: &mut i32
    // *nb_ray += 1;
    if depth <= 0 {
        return Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    if let Some(ray_hitten) = world.hit(r, 0.001, 1000000.0) {
        if let Some((scattered, attenuation)) = ray_hitten.material.scatter(r, &ray_hitten) {
            return ray_color(&scattered, world, depth - 1) * attenuation
        }
        return Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3{x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + Vec3{x: 0.5, y: 0.7, z: 1.0} * t
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: Material
}

struct World {
    objects: Vec<HitableObject>,
}

// enum HitableObject {
//     Sphere{center: Vec3, radius: f32, material: Material}
// }

struct Sphere{center: Vec3, radius: f32, material: Material}
struct MovingSphere{center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: Material}
impl MovingSphere {
    fn center(center0: Vec3, center1: Vec3, time0: f32, time1: f32, time: f32) -> Vec3 {
        center0 + (center1 - center0) * ((time - time0) / (time1 - time0))
    }
}


enum HitableObject {
    Sphere(Sphere),//{center: Vec3, radius: f32, material: Material},
    MovingSphere(MovingSphere)//{center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: Material}
}


impl World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut return_record = None;

        for object in &self.objects {
            if let Some(hitten) = object.hit(&r, t_min, closest_so_far) {
                closest_so_far = hitten.t;
                return_record = Some(hitten);
            }
        }
        return return_record
    }
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for HitableObject {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            HitableObject::Sphere(Sphere {center, radius, material}) => {
                let oc = r.origin - *center;
                let a = r.direction.length_squared();
                let half_b = oc.dot(r.direction);
                let c = oc.length_squared() - *radius * *radius;
                let discriminant = half_b * half_b - a * c;
                if discriminant > 0.0 {
                    let root = discriminant.sqrt();

                    let temp = (- half_b - root) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.at(temp);
                        let normal = (p - *center) / *radius;
                        let front_face = r.direction.dot(normal) < 0.0;
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: if front_face {normal} else {- normal},
                            front_face: front_face,
                            material: *material
                        })
                    }
                    let temp = (- half_b + root) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.at(temp);
                        let normal = (p - *center) / *radius;
                        let front_face = r.direction.dot(normal) < 0.0;
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: if front_face {normal} else {- normal},
                            front_face: front_face,
                            material: *material
                        })
                    }
                }
                None
            }
            HitableObject::MovingSphere(MovingSphere {center0, center1, time0, time1, radius, material}) => {
                let oc = r.origin - MovingSphere::center(*center0, *center1, *time0, *time1, r.time);
                let a = r.direction.length_squared();
                let half_b = oc.dot(r.direction);
                let c = oc.length_squared() - *radius * *radius;
                let discriminant = half_b * half_b - a * c;
                if discriminant > 0.0 {
                    let root = discriminant.sqrt();

                    let temp = (- half_b - root) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.at(temp);
                        let normal = (p - MovingSphere::center(*center0, *center1, *time0, *time1, r.time)) / *radius;
                        let front_face = r.direction.dot(normal) < 0.0;
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: if front_face {normal} else {- normal},
                            front_face: front_face,
                            material: *material
                        })
                    }
                    let temp = (- half_b + root) / a;
                    if temp < t_max && temp > t_min {
                        let p = r.at(temp);
                        let normal = (p - MovingSphere::center(*center0, *center1, *time0, *time1, r.time)) / *radius;
                        let front_face = r.direction.dot(normal) < 0.0;
                        return Some(HitRecord {
                            t: temp,
                            p: p,
                            normal: if front_face {normal} else {- normal},
                            front_face: front_face,
                            material: *material
                        })
                    }
                }
                None
            }
        }
    }
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32
}

impl Camera {
    fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32, t0: f32, t1: f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            lens_radius: lens_radius,
            time0: t0,
            time1: t1
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
       let rd = Vec3::random_in_unit_disk() * self.lens_radius;
       let offset = self.u * rd.x + self.v * rd.y;
       Ray{
          origin: self.origin + offset,
          direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
          time: self.time0 + (self.time1 - self.time0) * rand::random::<f32>()
      }
    }
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3, fuzz: f32},
    Dielectric {ref_idx: f32}
}

impl Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian {albedo} => {
                let scatter_direction = rec.normal + Vec3::random_unit_vector();
                let scattered = Ray{origin: rec.p, direction: scatter_direction, time: r.time};
                Some((scattered, *albedo))
            }
            Material::Metal {albedo, fuzz} => {
                let reflected = Vec3::reflect(r.direction.unit_vector(), rec.normal);
                let scattered = Ray{origin: rec.p, direction: reflected + Vec3::random_in_unit_sphere() * *fuzz, time: 0.0};
                if scattered.direction.dot(rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
            Material::Dielectric {ref_idx} => {
                let etai_over_etat = if rec.front_face {1.0 / *ref_idx} else {*ref_idx};
                let unit_direction = r.direction.unit_vector();

                let cos_theta = (- unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                if etai_over_etat * sin_theta > 1.0 {
                    let reflected = Vec3::reflect(unit_direction, rec.normal);
                    let scattered = Ray{origin: rec.p, direction: reflected, time: 0.0};
                    return Some((scattered, Vec3{x: 1.0, y: 1.0, z: 1.0}))
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if rand::random::<f32>() < reflect_prob {
                    let reflected = Vec3::reflect(unit_direction, rec.normal);
                    let scattered = Ray{origin: rec.p, direction: reflected, time: 0.0};
                    return Some((scattered, Vec3{x: 1.0, y: 1.0, z: 1.0}))
                }

                let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
                let scattered = Ray{origin: rec.p, direction: refracted, time: 0.0};
                Some((scattered, Vec3{x: 1.0, y: 1.0, z: 1.0}))
            }
        }
    }
}

// fn random_scene() -> World {
//     let mut spheres: Vec<Box<dyn Hitable>> = Vec::<Box<dyn Hitable>>::new();
//     // spheres.push(Box::new(Sphere{
//     //     center: Vec3{x: 0.0, y: - 1000.0, z: 0.0},
//     //     radius: 1000.0,
//     //     material: Material::Lambertian{albedo: Vec3{x: 0.5, y: 0.5, z: 0.5}}
//     // }));
//
//     for a in - 11..11 {
//         for b in -11..11 {
//             let choose_mat = rand::random::<f32>();
//             let center = Vec3{
//                 x: a as f32 + 0.9 * rand::random::<f32>(),
//                 y: 0.2,
//                 z: b as f32 + 0.9 * rand::random::<f32>()
//             };
//
//             if (center - Vec3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
//                     spheres.push(Box::new(Sphere{
//                         center: center,
//                         radius: 0.2,
//                         material: Material::Lambertian{albedo: albedo}
//                     }))
//                 } else if choose_mat < 0.95 {
//                     let albedo = Vec3::random(0.0, 0.5);
//                     let fuzz = rand::random::<f32>() * 0.5;
//                     spheres.push(Box::new(Sphere{
//                         center: center,
//                         radius: 0.2,
//                         material: Material::Metal{albedo: albedo, fuzz: fuzz}
//                     }))
//                 } else {
//                     spheres.push(Box::new(Sphere{
//                         center: center,
//                         radius: 0.2,
//                         material: Material::Dielectric{ref_idx: 1.5}
//                     }))
//                 }
//             }
//         }
//     }
//
//     // spheres.push(Box::new(Sphere{
//     //     center: Vec3{x: 0.0, y: 1., z: 0.0},
//     //     radius: 1.0,
//     //     material: Material::Dielectric{ref_idx: 1.5}
//     // }));
//     // spheres.push(Box::new(Sphere{
//     //     center: Vec3{x: - 4.0, y: 1.0, z: 0.0},
//     //     radius: 1.0,
//     //     material: Material::Lambertian{albedo: Vec3{x: 0.4, y: 0.2, z: 0.1}}
//     // }));
//     // spheres.push(Box::new(Sphere{
//     //     center: Vec3{x: 4.0, y: 1.0, z: 0.0},
//     //     radius: 1.0,
//     //     material: Material::Metal{albedo: Vec3{x: 0.7, y: 0.6, z: 0.4}, fuzz: 0.0}
//     // }));
//
//     World{objects: spheres}
// }

fn random_scene() -> World {
    let mut spheres = vec![];
    spheres.push(HitableObject::Sphere( Sphere{
        center: Vec3{x: 0.0, y: - 1000.0, z: 0.0},
        radius: 1000.0,
        material: Material::Lambertian{albedo: Vec3{x: 0.5, y: 0.5, z: 0.5}}
    }));

    for a in - 11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3{
                x: a as f32 + 0.9 * rand::random::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rand::random::<f32>()
            };

            if (center - Vec3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let center2 = center + Vec3{
                        x: 0.0,
                        y: 0.5 * rand::random::<f32>(),
                        z: 0.0,
                    };
                    spheres.push(HitableObject::MovingSphere( MovingSphere{
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian{albedo: albedo}
                    }))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.0, 0.5);
                    let fuzz = rand::random::<f32>() * 0.5;
                    spheres.push(HitableObject::Sphere( Sphere{
                        center: center,
                        radius: 0.2,
                        material: Material::Metal{albedo: albedo, fuzz: fuzz}
                    }))
                } else {
                    spheres.push(HitableObject::Sphere( Sphere{
                        center: center,
                        radius: 0.2,
                        material: Material::Dielectric{ref_idx: 1.5}
                    }))
                }
            }
        }
    }

    spheres.push(HitableObject::Sphere( Sphere{
        center: Vec3{x: 0.0, y: 1., z: 0.0},
        radius: 1.0,
        material: Material::Dielectric{ref_idx: 1.5}
    }));
    spheres.push(HitableObject::Sphere( Sphere{
        center: Vec3{x: - 4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Material::Lambertian{albedo: Vec3{x: 0.4, y: 0.2, z: 0.1}}
    }));
    spheres.push(HitableObject::Sphere( Sphere{
        center: Vec3{x: 4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Material::Metal{albedo: Vec3{x: 0.7, y: 0.6, z: 0.4}, fuzz: 0.0}
    }));

    World{objects: spheres}
}

fn main() {
    let now = SystemTime::now();

    // todo : voir const si on met en maj
    // todo voir trait object pas juste sphere
    let aspect_ratio: f32 = 3.0 / 2.0;
    let sample_per_pixel = 100;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let max_depth = 50;

    println!("P3");
    println!("{:?} {:?}", image_width, image_height);
    println!("{:?}", 255);

    let lookfrom = Vec3{x: 13.0, y: 2.0, z: 3.0};
    let lookat = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let vup = Vec3{x: 0.0, y: 1.0, z: 0.0};
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0
     );

    let world = random_scene();
    // let mut nb_ray = &mut 0;

    // for height in (0..image_height).rev() {
    //     // eprintln!("{:?}", height);
    //     for width in 0..image_width {
    //         let mut pixel_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
    //         for _ in 0..sample_per_pixel {
    //             let u: f32 = (width as f32 + rand::random::<f32>()) / (image_width as f32 - 1.);
    //             let v: f32 = (height as f32 + rand::random::<f32>()) / (image_height as f32 - 1.);
    //             let r: Ray = camera.get_ray(u, v);
    //             pixel_color += ray_color(&r, &world, max_depth);
    //         }
    //         pixel_color.write_color(sample_per_pixel);
    //     }
    // }


    let v: Vec<Vec<Vec3>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|height| (0..image_width)
        .map(|width| {
            let mut pixel_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
            for _ in 0..sample_per_pixel {
                let u: f32 = (width as f32 + rand::random::<f32>()) / (image_width as f32 - 1.);
                let v: f32 = (height as f32 + rand::random::<f32>()) / (image_height as f32 - 1.);
                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            pixel_color
        })
        .collect())
        .collect();
    write_image(v, sample_per_pixel);

    eprintln!("{:?}", now.elapsed());
}

fn write_image(image: Vec<Vec<Vec3>>, sample_per_pixel: i32) {
    for col in image {
        for mut pixel in col {
            pixel /= sample_per_pixel as f32;
            let ir = (256.0 * clamp(pixel.x.sqrt(), 0.0, 0.999)) as i32;
            let ig = (256.0 * clamp(pixel.y.sqrt(), 0.0, 0.999)) as i32;
            let ib = (256.0 * clamp(pixel.z.sqrt(), 0.0, 0.999)) as i32;
            println!("{:?} {:?} {:?}", ir, ig, ib);
        }
    }
}
