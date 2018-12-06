use image::{ImageBuffer, Pixel, Rgb};
use rand::Rng;

mod materials;
mod shapes;
mod util;
use crate::util::{camera::Camera, vector3::Vec3, world};

fn main() -> std::io::Result<()> {
    let nx = 800; // original: 200
    let ny = 600; // original: 100
    let ns = 100; // original: 100

    let look_from = Vec3::new(13.0, 2.0, 5.0);
    let look_at = Vec3::new(0.0, 0.5, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    //let dist_to_focus = 10.0;
    let aperture = 0.2;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    let mut img = ImageBuffer::new(nx, ny);

    let total_progress = (nx * ny) as f32;
    let mut current_progress = 0.0;

    let _r = (std::f64::consts::PI / 4.0).cos();

    let mut world = world::random_scene();
    /*let mut world = HitableList::new(vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vec3::new(0.1, 0.1, 0.1))),
    ))]);*/

    let mut rng = rand::thread_rng();

    for j in 0..ny {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col += world::color(&r, &mut world, 0);
            }
            col /= ns as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let r = (255.99 * col[0]) as u8;
            let g = (255.99 * col[1]) as u8;
            let b = (255.99 * col[2]) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(i, j, pixel);
            current_progress += 1.0;

            print!(
                "Render progress: {:3.1}/100.0%\r",
                current_progress / total_progress * 100.0
            );
        }
    }

    let _ = img.save("output.png");

    Ok(())
}
