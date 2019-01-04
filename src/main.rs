use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use image::{ImageBuffer, Pixel, Rgb};
use rand::Rng;
use rayon::prelude::*;
use structopt::StructOpt;
use time::Duration;

mod defaults;
mod materials;
mod shapes;
mod textures;
mod transform;
mod util;
use crate::util::{config, vector3::Vec3, world};

// Got this from here: https://www.reddit.com/r/rust/comments/a6pvjk/my_first_rust_project/ebx03gn/
#[derive(StructOpt)]
#[structopt(name = "rt1w")]
struct Opt {
    /// Show ETA for renders
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Input config file
    #[structopt(short = "c", long = "config", parse(from_os_str), default_value = "")]
    config_file: PathBuf,

    /// Input scene file
    #[structopt(short = "s", long = "scene", parse(from_os_str), default_value = "")]
    scene_file: PathBuf,

    /// Run in single-threaded mode
    #[structopt(short = "st", long = "single-threaded")]
    single_threaded: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let (config, cam) = config::load_from_json(opt.config_file.to_str().unwrap().to_string());
    let world = world::load_from_json(opt.scene_file.to_str().unwrap().to_string());

    let total_progress = f64::from(config.width * config.height);

    let mut img = ImageBuffer::new(config.width, config.height);

    let start_time = time::now();

    if !opt.single_threaded {
        println!("Rendering in multithreaded mode...");

        let current_progress = Arc::new(Mutex::new(0.0));
        let previous_progress = Arc::new(Mutex::new(0.0));
        let previous_time = Arc::new(Mutex::new(time::now()));

        // I have to give credit to https://github.com/rudolphalmeida/raytrac for letting me
        // see how easy it is to parallelize this whole thing. Whew!
        let result: Vec<Vec<Vec3>> = (0..config.height)
            .into_par_iter()
            .map(|y| {
                let row: Vec<Vec3> = (0..config.width)
                    .into_par_iter()
                    .map(|x| {
                        let mut rng = rand::thread_rng();
                        let mut col = Vec3::zero();
                        for _ in 0..config.samples {
                            let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(config.width);
                            let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(config.height);
                            let r = cam.get_ray(u, v);
                            col += world::color(&r, &world, 0);
                        }

                        // Thanks to the Rust Book Ch 16.3 for providing me this.
                        // I mean the algorithm for estimating the time is still bad, but at least it works!
                        let current_progress = Arc::clone(&current_progress);
                        let mut current_progress = current_progress.lock().unwrap();
                        *current_progress += 1.0;
                        let current_time = time::now();

                        let previous_time = Arc::clone(&previous_time);
                        let mut previous_time = previous_time.lock().unwrap();

                        let previous_progress = Arc::clone(&previous_progress);
                        let mut previous_progress = previous_progress.lock().unwrap();

                        if (current_time - *previous_time) >= time::Duration::milliseconds(1000) {
                            let progress = f64::from(previous_time.tm_nsec)
                                + (total_progress - *previous_progress)
                                    * (current_time - *previous_time).num_nanoseconds().unwrap()
                                        as f64
                                    / (*current_progress - *previous_progress);

                            let time = Duration::nanoseconds(progress as i64);
                            let hours = time.num_hours();
                            let mins = time.num_minutes() % 60;
                            let secs = time.num_seconds() % 60;

                            *previous_progress = *current_progress;
                            *previous_time = current_time;

                            if opt.verbose {
                                println!(
                                    "ETA: {:02}:{:02}:{:02} | Render progress: {:3.2} / 100.00%\r",
                                    hours,
                                    mins,
                                    secs,
                                    *current_progress / total_progress * 100.0
                                );
                            }
                        }

                        col /= f64::from(config.samples);
                        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
                        col
                    })
                    .collect();
                row
            })
            .collect();

        for y in 0..config.height {
            for x in 0..config.width {
                let col = result[y as usize][x as usize];
                let r = (255.99 * col[0]) as u8;
                let g = (255.99 * col[1]) as u8;
                let b = (255.99 * col[2]) as u8;
                let pixel = Rgb::from_channels(r, g, b, 0);
                img.put_pixel(x, config.height - 1 - y, pixel);
            }
        }
    } else {
        println!("Rendering...");
        let mut img = ImageBuffer::new(config.width, config.height);
        let mut rng = rand::thread_rng();
        let mut current_progress = 0.0;
        let mut previous_time = time::now();
        let mut previous_progress = 0.0;

        for j in 0..config.height {
            for i in 0..config.width {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..config.samples {
                    let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(config.width);
                    let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(config.height);
                    let r = cam.get_ray(u, v);
                    col += world::color(&r, &world, 0);
                }
                col /= f64::from(config.samples);
                col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
                let r = (255.99 * col[0]) as u8;
                let g = (255.99 * col[1]) as u8;
                let b = (255.99 * col[2]) as u8;
                let pixel = Rgb::from_channels(r, g, b, 0);
                img.put_pixel(i, config.height - 1 - j, pixel);

                current_progress += 1.0;
                let current_time = time::now();

                if (current_time - previous_time) >= time::Duration::milliseconds(1000) {
                    let progress = f64::from(previous_time.tm_nsec)
                        + (total_progress - previous_progress)
                            * (current_time - previous_time).num_nanoseconds().unwrap() as f64
                            / (current_progress - previous_progress);
                    let time = Duration::nanoseconds(progress as i64);

                    let hours = time.num_hours();
                    let mins = time.num_minutes() % 60;
                    let secs = time.num_seconds() % 60;

                    previous_progress = current_progress;
                    previous_time = current_time;

                    if opt.verbose {
                        println!(
                            "ETA: {:02}:{:02}:{:02} | Render progress: {:3.2} / 100.00%\r",
                            hours,
                            mins,
                            secs,
                            current_progress / total_progress * 100.0
                        );
                    }
                }
            }
        }
    }

    let _ = img.save(config.output_filename);

    let end_time = time::now();
    let total_hours = (end_time - start_time).num_hours();
    let total_mins = (end_time - start_time).num_minutes() % 60;
    let total_secs = (end_time - start_time).num_seconds() % 60;
    println!(
        "Finished in {:02}:{:02}:{:02}",
        total_hours, total_mins, total_secs
    );

    Ok(())
}
