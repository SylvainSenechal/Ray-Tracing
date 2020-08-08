# Ray-Tracing
This repo contains my Rust & Javascript implementations of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as well as benchmarks

The javascript version is available live at todo add link website.
Crates : Rand & Rayon for multithreading
Rust project is build with --release flag on

# Visual Results
Rendered in 292s at 1200 width, 800 height, pixels sampled 500 times, ray bouncing max depth at 50
![GitHub Logo](/benchmarks/1200x500x50RustMulti.png)

# Benchmarks
Benchmarks are done with images rendered with 400 pixels width, 3 / 2 ratio ( => 266 pixels height), pixels sampled 100 times & ray bouncing max depth set at 50.
Results obtained on a Ryzen 2600

Run number | Javascript | Rust | Rust multi threaded | Multi threading speedup
-|-|-|-
1 | todo | 40.20s | 6.68s
2 | todo | 39.58s | 6.85s
3 | todo | 40.57s | 6.75s
**Average** |todo | 40.12s | 6.76s | x5.935

Multi threading speedup of 5.935 on 6 cores CPU => almost perfect scaling & multi threading is extremely fast & easy to set up with Rayon

