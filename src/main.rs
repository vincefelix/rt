mod camera;
mod color;
mod common;
mod cube;
mod cylindre;
mod hittable;
mod hittable_list;
mod material;
mod plane;
mod ray;
mod sphere;
mod vec3;

use std::io;
use std::rc::Rc;

use crate::vec3::Vec3;
use camera::Camera;
use color::Color;
use cube::Cube;
use cylindre::Cylinder;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use plane::Plane;
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let plane_material = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0))); // Couleur orange
    let plane = Plane::new(
        Point3::new(0.0, -1.0, 0.0), // Position de la surface plane sous les objets
        Vec3::new(0.0, 1.0, 0.0),    // Normale vers le haut pour être visible
        plane_material,
    );
    world.add(Box::new(plane));

    let plane_material = Rc::new(Lambertian::new(Color::new(1.0, 1.0, 0.0)));
    let plane = Plane::new(
        Point3::new(0.0, -1.0001, 0.3),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    );
    world.add(Box::new(plane));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = common::random_double();
    //         let center = Point3::new(
    //             a as f64 + 0.9 * common::random_double(),
    //             0.2,
    //             b as f64 + 0.9 * common::random_double(),
    //         );

    //         if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             if choose_mat < 0.8 {
    //                 // Diffuse
    //                 let albedo = Color::random() * Color::random();
    //                 let sphere_material = Rc::new(Lambertian::new(albedo));
    //                 world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
    //             } else if choose_mat < 0.95 {
    //                 // Metal
    //                 let albedo = Color::random_range(0.5, 1.0);
    //                 let fuzz = common::random_double_range(0.0, 0.5);
    //                 let sphere_material = Rc::new(Metal::new(albedo, fuzz));
    //                 world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
    //             } else {
    //                 // Glass
    //                 let sphere_material = Rc::new(Dielectric::new(1.5));
    //                 world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
    //             }
    //         }
    //     }
    // }

    // let material1 = Rc::new(Dielectric::new(1.5));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     material1,
    // )));

    // let albedo = Color::random_range(0.5, 1.0);
    // let fuzz = common::random_double_range(0.0, 0.5);
    // let sphere_material = Rc::new(Metal::new(albedo, fuzz));

    // let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-4.0, 1.0, 0.0),
    //     1.0,
    //     material2,
    // )));

    let cube_material = Rc::new(Metal::new(Color::new(0.8, 0.3, 0.3), 0.0));
    let cube = Cube::new(
        Point3::new(-1.0, 0.0, -1.0),
        Point3::new(1.0, 2.0, 1.0),
        cube_material,
    );
    world.add(Box::new(cube));

    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))); //(Color::new(0.7, 0.6, 0.5), 0.0));
    let cylinder = Cylinder::new(
        Point3::new(-4.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.8,
        4.0,
        cylinder_material,
    );
    world.add(Box::new(cylinder));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World

    let world = random_scene();

    // Camera

    // let lookfrom = Point3::new(13.0, 2.0, 3.0);
    // let lookat = Point3::new(0.0, 0.0, 0.0);

    // let lookfrom = Point3::new(5.0, 1.0, -6.0);
    // let lookat = Point3::new(2.0, 1.0, -3.0);

    // let vup = Point3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = 10.0;
    // let aperture = 0.1;

    // let lookfrom = Point3::new(0.0, 5.0, 0.0);  // Exemple de vue de haut
    // let lookat = Point3::new(0.0, 0.0, 0.0);
    // let vup = Vec3::new(0.0, 0.0, -1.0);  // Choix d'orientation correcte pour la vue de haut

    // let dist_to_focus = 10.0;
    //  let aperture = 0.1;

    // let cam = Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     20.0,
    //     ASPECT_RATIO,
    //     aperture,
    //     dist_to_focus,
    // );

    // let lookfrom = Point3::new(0.0, 7.0, -10.0); // La caméra est reculée et élevée
    // let lookat = Point3::new(0.0, 0.0, 0.0); // La caméra regarde vers le centre de la scène
    // let vup = Vec3::new(0.0, 1.0, 0.0); // "Up" reste aligné avec l'axe y pour une orientation correcte

    // let dist_to_focus = 10.0;
    // let aperture = 0.1;

    // let cam = Camera::new(
    //     lookfrom,
    //     lookat,
    //     vup,
    //     45.0,         // Angle de champ de vision (45 degrés est un bon départ)
    //     ASPECT_RATIO, // Ratio de l'image
    //     aperture,
    //     dist_to_focus,
    // );

    let lookfrom = Point3::new(0.0, 7.0, -10.0); // La caméra est reculée et élevée pour capturer le plan
    let lookat = Point3::new(0.0, 0.0, 0.0); // La caméra regarde vers le centre de la scène
    let vup = Vec3::new(0.0, 1.0, 0.0); // "Up" reste aligné avec l'axe y pour une orientation correcte

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        45.0,         // Angle de champ de vision (45 degrés est un bon départ)
        ASPECT_RATIO, // Ratio de l'image
        aperture,
        dist_to_focus,
    );

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
