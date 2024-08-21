# Ray Tracer Documentation

This documentation provides an overview of how to use the ray tracer, including creating and rendering different elements, adjusting brightness, and moving the camera.

## Prerequisites
Ensure you have Rust installed. If not, you can install it by following the instructions at Rust's official website.

## Usage
### Running the Ray Tracer
To run the ray tracer and render images, use the following command format:


```
cargo run <object_name>
```
### Available object_name Values
sphere: Renders a scene containing a sphere.
cube: Renders a scene containing a cube.
cylinder: Renders a scene containing a cylinder.
flat: Renders a scene containing a flat plane.
scene: Renders a complete scene with a sphere, cube, cylinder, and plane.
flat_and_cube: Renders a scene containing a flat plane and a cube.
all: Renders individual images for each of the above elements and a complete scene.
Example Usage
To render a scene containing a sphere:

```
cargo run sphere
```
To render all scenes individually:


```
cargo run all
```
Output
Rendered images are saved in the image directory with .ppm extensions. If the image directory does not exist, it will be created automatically.

### Adjusting the Camera
The camera position, orientation, and field of view (FOV) can be adjusted within the main function:

Position the Camera (lookfrom): Change the camera's position in 3D space.

```
let lookfrom = Point3::new(-1.0, 8.0, -10.0);
```
x, y, z: Adjust these values to move the camera in space.
Focus the Camera (lookat): Set the point in the scene the camera is focused on.

```
let lookat = Point3::new(0.0, 1.0, 0.0);
```
Adjusting these values will change the direction the camera is looking.
Camera Orientation (vup): Set the "up" direction for the camera.

```
let vup = Vec3::new(0.0, 1.0, 0.0);
```
Changing this vector rotates the camera around its viewing axis.
Field of View (fov): Adjust the camera's field of view (FOV).

```javascript
let camera = Camera::new(
    lookfrom,
    lookat,
    vup,
    40.0,  // FOV in degrees
    ASPECT_RATIO,
    aperture,
    dist_to_focus,
);
```
A smaller FOV value zooms in, while a larger value zooms out.
Adjusting Brightness
Brightness is affected by the SAMPLES_PER_PIXEL constant:


```rust
const SAMPLES_PER_PIXEL: i32 = 100;
```
Increase SAMPLES_PER_PIXEL to reduce noise and create a brighter image.
Decrease SAMPLES_PER_PIXEL for a darker image but with more noise.
Creating New Elements
To create new elements (like additional shapes or different scenes):

### Add a New Scene Function:
Create a new function to define the new scene. For example:


``` javascript
fn create_custom_scene() -> HittableList {
    let mut world = HittableList::new();
    // Add objects to the world
    world
}
```

### Update the main Function:
Add the new scene to the main function so it can be selected:


``` javascript
let world = match object_name.as_str() {
    "sphere" => create_sphere_scene(),
    "custom" => create_custom_scene(),  // Add this line
    // other cases...
    _ => unreachable!(),
};
```


### Viewing Shadows
To enhance shadow visibility:

Ensure that objects in the scene can cast shadows by placing them in positions where light would naturally be obstructed.
Adjust the camera angle to capture shadows by modifying lookfrom and lookat vectors.
Troubleshooting
If an object doesn't render correctly, check the camera parameters and the object's position.
Ensure you are using the correct object_name when running cargo run.
Conclusion
This ray tracer provides a flexible platform for rendering 3D scenes with different objects. By adjusting camera settings and scene configurations, you can generate a wide range of images, exploring various lighting and shadow effects.