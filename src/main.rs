use image::{ImageBuffer, Pixel, Rgb};
use rand::Rng;

use std::env;

mod defaults;
mod materials;
mod shapes;
mod util;
use crate::util::{
    camera::Camera,
    config::{self, Config},
    vector3::Vec3,
    world,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    /*
    let total_progress = (nx * ny) as f32;
    let mut current_progress = 0.0;
    */

    /*
    let config = if args.len() == 2 {
        config::load_from_json(args[1].clone())
    } else {
        Config::default()
    };*/

    let ((config, cam), mut world) = match (args.get(1), args.get(2)) {
        (Some(config_file), Some(scene_file)) => (
            config::load_from_json(config_file.to_string()),
            world::load_from_json(scene_file.to_string()),
        ),
        (Some(config_file), _) => (
            config::load_from_json(config_file.to_string()),
            world::random_scene(),
        ),
        (_, _) => (
            (Config::default(), Camera::default()),
            world::random_scene(),
        ),
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
            img.put_pixel(i, j, pixel);

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
