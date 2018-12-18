use std::fs;

use serde_json::Value;

use crate::defaults;
use crate::util::{camera::Camera, vector3::Vec3};

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub output_filename: String,
}

impl Config {
    pub fn new(width: u32, height: u32, samples: u32, output_filename: String) -> Config {
        Config {
            width,
            height,
            samples,
            output_filename,
        }
    }

    pub fn default() -> Config {
        Config {
            width: defaults::WIDTH,
            height: defaults::HEIGHT,
            samples: defaults::SAMPLES,
            output_filename: defaults::OUTPUT_FILENAME.to_string(),
        }
    }
}

pub fn load_from_json(filename: String) -> (Config, Camera) {
    if filename == "".to_string() {
        println!("Defaulting to config defaults...");
        return (Config::default(), Camera::default());
    }

    println!("Loading config JSON file from {}...", filename);
    let data = match fs::read_to_string(filename) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            println!("Defaulting to config defaults...");
            return (Config::default(), Camera::default());
        }
    };

    let values: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            println!("Defaulting to config defaults...");
            return (Config::default(), Camera::default());
        }
    };

    let output_filename = values["config"]["output"].as_str();
    let output_filename = match output_filename {
        Some(s) => s,
        _ => defaults::OUTPUT_FILENAME,
    };

    let width = values["config"]["width"].as_u64();
    let height = values["config"]["height"].as_u64();
    let samples = values["config"]["samples"].as_u64();
    let (width, height, samples) = match (width, height, samples) {
        (Some(w), Some(h), Some(s)) => (w as u32, h as u32, s as u32),
        (_, _, _) => (defaults::WIDTH, defaults::HEIGHT, defaults::SAMPLES),
    };

    let from = vec![
        values["camera"]["from"]["x"].as_f64(),
        values["camera"]["from"]["y"].as_f64(),
        values["camera"]["from"]["z"].as_f64(),
    ];
    let to = vec![
        values["camera"]["to"]["x"].as_f64(),
        values["camera"]["to"]["y"].as_f64(),
        values["camera"]["to"]["z"].as_f64(),
    ];
    let (fx, fy, fz, tx, ty, tz) = match (from[0], from[1], from[2], to[0], to[1], to[2]) {
        (Some(fx), Some(fy), Some(fz), Some(tx), Some(ty), Some(tz)) => (fx, fy, fz, tx, ty, tz),
        (_, _, _, _, _, _) => {
            println!("Can't read camera look from/to values...");
            println!("Defaulting to camera defaults");
            (
                defaults::LOOK_FROM.0,
                defaults::LOOK_FROM.1,
                defaults::LOOK_FROM.2,
                defaults::LOOK_TO.0,
                defaults::LOOK_TO.1,
                defaults::LOOK_TO.2,
            )
        }
    };
    let look_from = Vec3::new(fx, fy, fz);
    let look_to = Vec3::new(tx, ty, tz);

    let vertical = vec![
        values["camera"]["vertical"]["x"].as_f64(),
        values["camera"]["vertical"]["y"].as_f64(),
        values["camera"]["vertical"]["z"].as_f64(),
    ];
    let (vx, vy, vz) = match (vertical[0], vertical[1], vertical[2]) {
        (Some(vx), Some(vy), Some(vz)) => (vx, vy, vz),
        (_, _, _) => (
            defaults::VERTICAL.0,
            defaults::VERTICAL.1,
            defaults::VERTICAL.2,
        ),
    };
    let vup = Vec3::new(vx, vy, vz);

    let fov = values["camera"]["fov"].as_f64();
    let vfov = match fov {
        Some(vfov) => vfov,
        _ => defaults::FOV,
    };

    let aspect = values["camera"]["aspect_ratio"].as_f64();
    let aspect = match aspect {
        Some(aspect) => aspect,
        _ => width as f64 / height as f64,
    };

    let aperture = values["camera"]["aperture"].as_f64();
    let aperture = match aperture {
        Some(aperture) => aperture,
        _ => defaults::APERTURE,
    };

    let focus_distance = values["camera"]["focus_distance"].as_f64();
    let focus_dist = match focus_distance {
        Some(focus_dist) => focus_dist,
        _ => (look_from - look_to).length(),
    };

    let (t0, t1) = (
        values["camera"]["t0"].as_f64(),
        values["camera"]["t1"].as_f64(),
    );
    let (t0, t1) = match (t0, t1) {
        (Some(t0), Some(t1)) => (t0, t1),
        (_, _) => (defaults::T0, defaults::T1),
    };

    println!("Loaded config JSON file.");
    println!(
        "Rendering a {}x{} image at {} samples/pixel...",
        width, height, samples
    );
    println!("Output filename: {}", output_filename);
    println!(
        "Camera positioned at ({},{},{}) looking at ({},{},{})",
        look_from.x, look_from.y, look_from.z, look_to.x, look_to.y, look_to.z
    );
    println!("Camera settings:");
    println!("   FOV: {}", vfov);
    println!("   Aspect ratio: {}", aspect);
    println!("   Aperture: {}", aperture);
    println!("   Focus distance: {}", focus_dist);
    println!("   Shutter open from t={} to {}", t0, t1);

    (
        Config::new(width, height, samples, output_filename.to_string()),
        Camera::new(
            look_from, look_to, vup, vfov, aspect, aperture, focus_dist, t0, t1,
        ),
    )
}
