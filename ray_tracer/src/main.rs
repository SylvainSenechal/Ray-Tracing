use rand::Rng;


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

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2.0 * v.dot(n)
        // v - (n * 2.0) * v.dot(n)
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

// todo : implement sur ray
fn ray_color(r: &Ray, world: &World, depth: i32, nb_ray: &mut i32) -> Vec3 {
    *nb_ray += 1;
    if depth <= 0 {
        return Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    if let Some(ray_hitten) = world.hit(r, 0.001, 1000000.0) {
        // let target = ray_hitten.p + ray_hitten.normal + Vec3::random_unit_vector();
        // return ray_color(&Ray{origin: ray_hitten.p, direction: target - ray_hitten.p}, world, depth - 1, nb_ray) * 0.5
        if let Some((scattered, attenuation)) = ray_hitten.material.scatter(r, &ray_hitten) {
            return ray_color(&scattered, world, depth - 1, nb_ray) * attenuation
        }
        return Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3{x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + Vec3{x: 0.5, y: 0.7, z: 1.0} * t
}

struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: Material
}

struct World {
    objects: Vec<Sphere>,
}

impl World {
    // todo add fn
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



// todo trait hit
impl Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (- half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = r.at(temp);
                let normal = (p - self.center) / self.radius;
                let front_face = r.direction.dot(normal) < 0.0;
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: if front_face {normal} else {- normal},
                    front_face: front_face,
                    material: self.material
                })
            }
            let temp = (- half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = r.at(temp);
                let normal = (p - self.center) / self.radius;
                let front_face = r.direction.dot(normal) < 0.0;
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: if front_face {normal} else {- normal},
                    front_face: front_face,
                    material: self.material
                })
            }
        }
        None
    }
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    fn new(aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
        let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
        let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray{
           origin: self.origin,
           direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
       }
    }
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3}
}

impl Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian {albedo} => {
                let scatter_direction = rec.normal + Vec3::random_unit_vector();
                let scattered = Ray{origin: rec.p, direction: scatter_direction};
                Some((scattered, *albedo))
            }
            Material::Metal {albedo} => {
                let reflected = Vec3::reflect(r.direction.unit_vector(), rec.normal);
                let scattered = Ray{origin: rec.p, direction: reflected};
                if scattered.direction.dot(rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    // todo : voir const si on met en maj
    let aspect_ratio: f32 = 16.0 / 9.0;
    let sample_per_pixel = 5;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;


    println!("P3");
    println!("{:?} {:?}", image_width, image_height);
    println!("{:?}", 255);

    let camera = Camera::new(aspect_ratio);
    let max_depth = 50;
    // let spheres = vec![
    //     Sphere{center: Vec3{x: 0.0, y: 0.0, z: - 1.0}, radius: 0.5},
    //     Sphere{center: Vec3{x: 0.0, y: - 100.5, z: - 1.0}, radius: 100.0}
    // ];

    let spheres = vec![
        Sphere{
            center: Vec3{x: 0.0, y: - 100.5, z: - 1.0},
            radius: 100.0,
            material: Material::Lambertian{albedo: Vec3{x: 0.8, y: 0.8, z: 0.0}}
        },
        Sphere{
            center: Vec3{x: 0.0, y: 0.0, z: - 1.0},
            radius: 0.5,
            material: Material::Lambertian{albedo: Vec3{x: 0.7, y: 0.3, z: 0.3}}
        },
        Sphere{
            center: Vec3{x: - 1.0, y: 0.0, z: - 1.0},
            radius: 0.5,
            material: Material::Metal{albedo: Vec3{x: 0.8, y: 0.8, z: 0.8}}
        },
        Sphere{
            center: Vec3{x: 1.0, y: 0.0, z: - 1.0},
            radius: 0.5,
            material: Material::Metal{albedo: Vec3{x: 0.8, y: 0.6, z: 0.2}}
        },
    ];

    let world = World{objects: spheres};
    let mut nb_ray = &mut 0;
    for height in (0..image_height).rev() {
        // eprintln!("{:?}", height);
        eprintln!("{:?}", nb_ray);
        for width in 0..image_width {
            let mut pixel_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
            for _ in 0..sample_per_pixel {

                let u: f32 = (width as f32 + rand::random::<f32>()) / (image_width as f32 - 1.);
                let v: f32 = (height as f32 + rand::random::<f32>()) / (image_height as f32 - 1.);

                let r: Ray = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, max_depth, nb_ray);
            }

            pixel_color.write_color(sample_per_pixel);
        }
    }
}

// fn main() {
//     let a = rand::random::<f32>;
//     println!("{:?}", a);
//     // let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
//     // a /= 10.0;
//     // println!("{:?}", a);
//     // println!("{:?}", a.unit_vector());
//     // println!("{:?}", a);
// }
