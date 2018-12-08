use image::{ImageBuffer, Pixel, Rgb};
use rand::Rng;

use std::env;

mod defaults;
mod materials;
mod shapes;
mod textures;
mod util;
use crate::util::{
    camera::Camera,
    config::{self, Config},
    hitable_list::HitableList,
    vector3::Vec3,
    world,
};

fn get_default() -> ((Config, Camera), HitableList) {
    (
        (Config::default(), Camera::default()),
        world::random_scene(),
    )
}

fn help() {
    println!("rt1w -- A Rust implementation of the book Ray Tracing in One Weekend.");
    println!("Arguments:");
    println!("   --config [config.json]     Input a configuration JSON file.");
    println!("   --scene [scene.json]       Input a scene JSON file.");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    /*
    let total_progress = (nx * ny) as f32;
    let mut current_progress = 0.0;
    */

    // TODO: Refactor the code so camera data is part of scene rather than config files
    // It kinda made sense when I first started making this...
    let ((config, cam), mut world) = match args.len() {
        1 | 2 | 4 => {
            help();
            println!("Rendering with default settings.");
            get_default()
        }
        3 => {
            let cmd = &args[1];
            let file = &args[2];
            match &cmd[..] {
                "--config" => (
                    config::load_from_json(file.to_string()),
                    world::random_scene(),
                ),
                "--scene" => (
                    (Config::default(), Camera::default()),
                    world::load_from_json(file.to_string()),
                ),
                _ => {
                    eprintln!("Can't load the file. Reverting to defaults...");
                    get_default()
                }
            }
        }
        5 => {
            let cmd1 = &args[1];
            let cmd2 = &args[3];
            let file1 = &args[2];
            let file2 = &args[4];
            match (&cmd1[..], &cmd2[..]) {
                ("--config", "--scene") => (
                    config::load_from_json(file1.to_string()),
                    world::load_from_json(file2.to_string()),
                ),
                ("--scene", "--config") => (
                    config::load_from_json(file2.to_string()),
                    world::load_from_json(file1.to_string()),
                ),
                (_, _) => {
                    eprintln!("Can't load the files. Reverting to defaults...");
                    get_default()
                }
            }
        }
        _ => {
            help();
            println!("Rendering with default settings.");
            get_default()
        }
    };

    let mut img = ImageBuffer::new(config.width, config.height);

    let mut rng = rand::thread_rng();

    println!("Rendering...");
    for j in 0..config.height {
        for i in 0..config.width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..config.samples {
                let u = (i as f64 + rng.gen::<f64>()) / config.width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / config.height as f64;
                let r = cam.get_ray(u, v);
                col += world::color(&r, &mut world, 0);
            }
            col /= config.samples as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let r = (255.99 * col[0]) as u8;
            let g = (255.99 * col[1]) as u8;
            let b = (255.99 * col[2]) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(i, config.height - 1 - j, pixel);

            /*
            current_progress += 1.0;
            println!(
                "Render progress: {:3.1}/100.0%\r",
                current_progress / total_progress * 100.0
            );
            */
        }
    }

    let _ = img.save(config.output_filename);

    Ok(())
}
