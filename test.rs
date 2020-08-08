use std::time::SystemTime;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {


    fn random(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: min + (max - min) * 0.3,
            y: min + (max - min) * 0.4,
            z: min + (max - min) * 0.9,
        }
    }
}

fn sum(input: Vec<i32>) -> i32 {
    input.iter().map(|e| e + 1).sum()
}

fn main() {
    let now = SystemTime::now();
    let mut img: Vec<Vec<i32>> = vec![];
    img.push(vec![1,2,3]);
    img.push(vec![10,20]);
    println!("{:?}", img.len());
    // img.iter()

    for vec in &img {
        for elem in vec {
            println!("{:?}", elem);
        }
    }

    for vec in img.iter() {
        for elem in vec.iter() {
            println!("{:?}", elem);
        }
    }

    let v: Vec<Vec<Vec3>> = (0..3)
    .map(|_| (0..2)
    .map(|_| Vec3::random(0.0, 1.0))
    .collect())
    .collect();
    println!("{:?}", v);

    let a = vec![1,2,3,4,5];
    let b = sum(a);
    println!("{:?}", b);
    // // todo : voir const si on met en maj
    // // todo voir trait object pas juste sphere
    // let aspect_ratio: f32 = 3.0 / 2.0;
    // let sample_per_pixel = 10;
    // let image_width: i32 = 300;
    // let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    // let max_depth = 50;
    //
    // println!("P3");
    // println!("{:?} {:?}", image_width, image_height);
    // println!("{:?}", 255);
    //
    // let lookfrom = Vec3{x: 13.0, y: 2.0, z: 3.0};
    // let lookat = Vec3{x: 0.0, y: 0.0, z: 0.0};
    // let vup = Vec3{x: 0.0, y: 1.0, z: 0.0};
    // let dist_to_focus = 10.0;
    // let aperture = 0.1;
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     20.0,
    //     aspect_ratio,
    //     aperture,
    //     dist_to_focus
    //  );
    //
    // let world = random_scene();
    // let mut nb_ray = &mut 0;
    // for height in (0..image_height).rev() {
    //     // eprintln!("{:?}", height);
    //     eprintln!("{:?}", nb_ray);
    //     for width in 0..image_width {
    //         let mut pixel_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
    //         for _ in 0..sample_per_pixel {
    //
    //             let u: f32 = (width as f32 + rand::random::<f32>()) / (image_width as f32 - 1.);
    //             let v: f32 = (height as f32 + rand::random::<f32>()) / (image_height as f32 - 1.);
    //
    //             let r: Ray = camera.get_ray(u, v);
    //
    //             pixel_color += ray_color(&r, &world, max_depth, nb_ray);
    //         }
    //
    //         pixel_color.write_color(sample_per_pixel);
    //     }
    // }
    eprintln!("{:?}", now.elapsed());
}
