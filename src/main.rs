use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::rc::Rc;
use std::env;

mod camera;
mod color;
mod common;
mod cube;
mod cylindre;
mod hittable;
mod hittable_list;
mod material;
// mod plane;
mod ray;
mod rectangle;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use cube::Cube;
use cylindre::Cylinder;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
// use plane::Plane;
use ray::Ray;
use rectangle::Rectangle;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray, world: &dyn hittable::Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = hittable::HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

// Fonction pour sauvegarder une image de la scène
fn save_scene_image(cam: &Camera, world: &HittableList, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32, file_name: &str) {
    let file = File::create(file_name).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + common::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + common::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, world, max_depth);
            }
            color::write_color(&mut writer, pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone rendering: {}", file_name);
}

// Fonctions de création des scènes
fn create_ground_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    world
}

fn create_cube_scene() -> HittableList {
    let mut world = create_ground_scene();
    let cube_material = Rc::new(Metal::new(Color::new(0.8, 0.3, 0.3), 0.1));
    let cube = Cube::new(
        Point3::new(-1.0, 0.0, -1.0),
        Point3::new(1.0, 2.0, 1.0),
        cube_material,
    );
    world.add(Box::new(cube));
    world
}

fn create_cylinder_scene() -> HittableList {
    let mut world = create_ground_scene();
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let cylinder = Cylinder::new(
        Point3::new(-4.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.8,
        4.0,
        cylinder_material,
    );
    world.add(Box::new(cylinder));
    world
}

fn create_sphere_scene() -> HittableList {
    let mut world = create_ground_scene();
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.2));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn create_plane_scene() -> HittableList {
    let mut world = create_ground_scene();
    // let plane_material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.8));
    let plane_material = Rc::new(Dielectric::new(0.3));
    let rect = Rectangle::new(
        Point3::new(0.0, -0.0001, -2.0),  // Positionnement plus bas
        Vec3::new(2.0, 0.0, 0.0),        // Largeur
        Vec3::new(0.0, 0.0, -2.0),        // Profondeur
        plane_material,
    );
    world.add(Box::new(rect));
    world
}

fn render_all_individual_images(cam: &Camera, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32) {
    let cube_scene = create_cube_scene();
    save_scene_image(cam, &cube_scene, image_width, image_height, samples_per_pixel, max_depth, "image/cube.ppm");

    let cylinder_scene = create_cylinder_scene();
    save_scene_image(cam, &cylinder_scene, image_width, image_height, samples_per_pixel, max_depth, "image/cylinder.ppm");

    let sphere_scene = create_sphere_scene();
    save_scene_image(cam, &sphere_scene, image_width, image_height, samples_per_pixel, max_depth, "image/sphere.ppm");

    let plane_scene = create_plane_scene();
    save_scene_image(cam, &plane_scene, image_width, image_height, samples_per_pixel, max_depth, "image/plane.ppm");

    let scene = create_world_with_scene();
    save_scene_image(cam, &scene, image_width, image_height, samples_per_pixel, max_depth, "image/scene.ppm");

    let flat_and_cube_scene = create_flat_plane_and_cube();
    save_scene_image(cam, &flat_and_cube_scene, image_width, image_height, samples_per_pixel, max_depth, "image/flat_and_cube.ppm");

}

fn create_flat_plane_and_cube() -> HittableList {
    let mut world = create_ground_scene();

    // Ajout du cube
    let cube_material = Rc::new(Metal::new(Color::new(0.8, 0.3, 0.3), 0.1));
    let cube = Cube::new(
        Point3::new(-1.0, 0.0, -1.0),
        Point3::new(1.0, 2.0, 1.0),
        cube_material,
    );
    world.add(Box::new(cube));


    // Ajout de la surface plane
    // let plane_material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.8));
    let plane_material = Rc::new(Dielectric::new(0.3));
    let rect = Rectangle::new(
        Point3::new(0.0, -0.0001, -2.0),  
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0), 
        plane_material,
    );
    world.add(Box::new(rect));

    world
}

fn create_world_with_scene() -> HittableList {
    let mut world = create_ground_scene();

    // Ajout du cube
    let cube_material = Rc::new(Metal::new(Color::new(0.8, 0.3, 0.3), 0.1));
    let cube = Cube::new(
        Point3::new(-1.0, 0.0, -1.0),
        Point3::new(1.0, 2.0, 1.0),
        cube_material,
    );
    world.add(Box::new(cube));

    // Ajout du cylindre
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let cylinder = Cylinder::new(
        Point3::new(-4.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.8,
        4.0,
        cylinder_material,
    );
    world.add(Box::new(cylinder));



    // Ajout de la sphère
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));



    // Ajout de la surface plane
    // let plane_material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.8));
    let plane_material = Rc::new(Dielectric::new(0.3));
    let rect = Rectangle::new(
        Point3::new(0.0, -0.0001, -2.0),  
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -2.0), 
        plane_material,
    );
    world.add(Box::new(rect));

    world
}

fn main() {
    // Récupération des arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <object_name>");
        eprintln!("object_name must be one of: sphere, cube, cylinder, flat, scene, flat_and_cube, all");
        return;
    }

    // Vérification et création du dossier 'image'
    let output_dir = "image";
    if fs::metadata(output_dir).is_err() {
        fs::create_dir_all(output_dir).expect("Failed to create 'image' directory");
    }

    // Configuration commune de la caméra et du rendu
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let lookfrom = Point3::new(6.0, 8.0, -20.0);
    let lookat = Point3::new(-7.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let object_name = &args[1];

    if object_name == "all" {
        render_all_individual_images(&camera, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    } else {
        let image_name = match object_name.as_str() {
            "sphere" => "image/sphere.ppm",
            "cube" => "image/cube.ppm",
            "cylinder" => "image/cylinder.ppm",
            "flat" => "image/flat-plane.ppm",
            "scene" => "image/scene.ppm",
            "flat_and_cube" => "image/flat_and_cube.ppm",
            _ => {
                eprintln!("Error: Unknown object '{}'", object_name);
                eprintln!("object_name must be one of: sphere, cube, cylinder, flat, scene, flat_and_cube, all");
                return;
            }
        };

        let world = match object_name.as_str() {
            "sphere" => create_sphere_scene(),
            "cube" => create_cube_scene(),
            "cylinder" => create_cylinder_scene(),
            "flat" => create_plane_scene(),
            "scene" => create_world_with_scene(),
            "flat_and_cube" => create_flat_plane_and_cube(),
            _ => unreachable!(),
        };

        // Sauvegarde l'image de l'objet demandé
        save_scene_image(&camera, &world, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, image_name);
    }
}
