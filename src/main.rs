use std::path::PathBuf;

use image::{ImageBuffer, Pixel, Rgb};
use rand::Rng;
use time::Duration;

use structopt::StructOpt;

mod defaults;
mod materials;
mod shapes;
mod textures;
mod transform;
mod util;
use crate::util::{
    camera::Camera,
    config::{self, Config},
    hitable_list::HitableList,
    vector3::Vec3,
    world,
};

// Got this from here: https://www.reddit.com/r/rust/comments/a6pvjk/my_first_rust_project/ebx03gn/
#[derive(StructOpt)]
#[structopt(name = "rt1w")]
struct Opt {
    /// Show verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Input config file
    #[structopt(short = "c", long = "config", parse(from_os_str), default_value = "")]
    config_file: PathBuf,

    /// Input scene file
    #[structopt(short = "s", long = "scene", parse(from_os_str), default_value = "")]
    scene_file: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let (config, cam) = config::load_from_json(opt.config_file.to_str().unwrap().to_string());
    let world = world::load_from_json(opt.scene_file.to_str().unwrap().to_string());

    let total_progress = (config.width * config.height) as f64;
    let mut current_progress = 0.0;
    let mut current_time = time::now();
    let mut previous_time = time::now();
    let mut previous_progress = 0.0;

    let mut img = ImageBuffer::new(config.width, config.height);

    let mut rng = rand::thread_rng();

    let start_time = time::now();
    println!("Rendering...");

    // TODO: Change this so everything's computed first and stored into a vector (try in parallel!)
    // before writing it to the image and saving.
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
            current_time = time::now();

            if (current_time - previous_time) >= time::Duration::milliseconds(1000) {
                let progress = previous_time.tm_nsec as f64
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
                        "ETA: {:02}:{:02}:{:02} | Render progress: {:3.1} / 100.0%\r",
                        hours,
                        mins,
                        secs,
                        current_progress / total_progress * 100.0
                    );
                }
            }
        }
    }

    let _ = img.save(config.output_filename);

    current_time = time::now();
    let total_hours = (current_time - start_time).num_hours();
    let total_mins = (current_time - start_time).num_minutes() % 60;
    let total_secs = (current_time - start_time).num_seconds() % 60;
    println!(
        "Finished in {:02}:{:02}:{:02}",
        total_hours, total_mins, total_secs
    );

    Ok(())
}
