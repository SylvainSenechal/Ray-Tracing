// use std::io;
// use std::fs::File;
// use std::io::Read;
// use std::time::SystemTime;
//
// fn read_input() -> Result<Vec<i32>, io::Error> {
//     let mut content = String::new();
//     File::open("input.txt")?.read_to_string(&mut content)?;
//     let mut mass_modules: Vec<i32> = Vec::new();
//
//     for line in content.lines() {
//         mass_modules.push(line.parse::<i32>().unwrap());
//     }
//
//     Ok(mass_modules)
// }
//
// fn compute_fuel(mass: &i32) -> i32 {
//     mass / 3 - 2
// }
//
// fn compute_fuel_recursive(mass: &i32) -> i32 {
//     match compute_fuel(mass) {
//         m if m <= 6 => m,
//         m => m + compute_fuel_recursive(&m)
//     }
// }
//
// fn part1(mass_modules: &Vec<i32>) -> i32 {
//     let mut total_fuel: i32 = 0;
//     for mass in mass_modules {
//         total_fuel += compute_fuel(mass);
//     }
//     total_fuel
// }
//
// fn part2(mass_modules: &Vec<i32>) -> i32 {
//     let mut total_fuel: i32 = 0;
//     for mass in mass_modules {
//         total_fuel += compute_fuel_recursive(mass);
//     }
//     total_fuel
// }
//
// fn main() {
//     let content = match read_input() {
//         Ok(val) => val,
//         Err(e) => panic!("{:?}", e),
//     };
//     let now = SystemTime::now();
//     let result1 = part1(&content);
//     let result2 = part2(&content);
//     println!("{:?}", result1);
//     println!("{:?}", result2);
//     println!("{:?}", now.elapsed());
// }

#[derive(Debug, Copy, Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Color {
    x: f32,
    y: f32,
    z: f32,
}

impl Color {
    fn write_color(&self) {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;
        println!("{:?} {:?} {:?}", ir, ig, ib);
    }
    fn multiply(&self, scalar: f32) -> Color {
        Color{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    fn add(&self, vec: &Color) -> Color {
        Color{
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl Vector3 {
    fn add(&self, vec: &Vector3) -> Vector3 {
        Vector3{
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
    fn sub(&self, vec: &Vector3) -> Vector3 {
        Vector3{
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
    fn div(&self, scalar: f32) -> Vector3 {
        Vector3{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
    fn multiply(&self, scalar: f32) -> Vector3 {
        Vector3{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn unit_vector(&self) -> Vector3 {
        self.div(self.length())
    }
    fn dot(&self, vec: Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }
}

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn at(&self, t: f32) -> Vector3 {
        self.origin.add(&self.direction.multiply(t))
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Vector3{x: 0., y: 0., z: -1.}, 0.5, &r);
    if t > 0. {
        let N: Vector3 = Vector3::unit_vector(&r.at(t).sub(&Vector3{x: 0., y: 0., z: -1.}));
        return Color{x: N.x + 1., y: N.y + 1., z: N.z + 1.}.multiply(0.5)
    }
    let unit_direction = Vector3::unit_vector(&r.direction);
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    Color{x:1.0, y: 1.0, z: 1.0}.multiply(1.0 - t).add(&Color{x: 0.5, y: 0.7, z: 1.0}.multiply(t))
}

fn hit_sphere(center: Vector3, radius: f32, r: &Ray) -> f32 {
    let oc: Vector3 = r.origin.sub(&center);
    let a: f32 = r.direction.length_squared();
    let half_b: f32 = oc.dot(r.direction);
    let c: f32 = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        return -1.
    } else {
        return (- half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    println!("P3");
    println!("{:?} {:?}", image_width, image_height);
    println!("{:?}", 255);

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin:  Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal: Vector3 = Vector3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical: Vector3 = Vector3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner: Vector3 = origin.sub(&horizontal.div(2.0)).sub(&vertical.div(2.0)).sub(&Vector3{x: 0.0, y: 0.0, z: focal_length});

    for height in (0..image_height).rev() {
        // eprintln!("{:?}", height);
        for width in 0..image_width {
            let u: f32 = width as f32 / (image_width as f32 - 1.);
            let v: f32 = height as f32 / (image_height as f32 - 1.);

            let r: Ray = Ray{
                origin: origin,
                direction: lower_left_corner.add(&horizontal.multiply(u)).add(&vertical.multiply(v)).sub(&origin),
            };

            let pixel_color: Color = ray_color(r);

            // let pixel_color: Color = Color{
            //     x: width as f32 / (image_width as f32 - 1.),
            //     y: height as f32 / (image_height as f32 - 1.),
            //     z: 0.25,
            // };
            pixel_color.write_color();

        }
    }
}

// fn main() {
//     let image_width :i32 = 256;
//     let image_height :i32 = 256;
//     println!("P3");
//     println!("{:?} {:?}", image_width, image_height);
//     println!("{:?}", 255);
//
//     for height in (0..image_height).rev() {
//         eprintln!("{:?}", height);
//         for width in 0..image_width {
//             let pixel_color = Color{
//                 x: width as f32 / (image_width as f32 - 1.),
//                 y: height as f32 / (image_height as f32 - 1.),
//                 z: 0.25,
//             };
//             pixel_color.write_color();
//
//         }
//     }
// }
