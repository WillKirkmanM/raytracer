<p align="center">
  <img src="https://avatars.githubusercontent.com/u/138057124?s=200&v=4" width="150" />
</p>
<h1 align="center">Raytracer</h1>

<p align="center">Multithreaded, Physically-Based, Multi-Material, Anti-Aliased & Sphere Rendered RayTracer Engine</p>


## Features
- Physically-Based Light Transport
- Multiple Materials (Lambertain Diffuse, Metal, Dielectric / Glass)
- Anti-Aliasing
- Multithreaded Rendering with Rayon
- Sphere Rendering with Different Materials

## Clone the repository
```shell
git clone https://github.com/WillKirkmanM/raytracer.git
cd raytracer
```

## Build & Run in Release Mode (Recommended for Performance)
```shell
cargo run --release
```

The rendered image will be saved as output.png in the project directory.

Project Structure
- `main.rs`: Entry point and scene setup
- `vec3.rs`: 3D vector operations
- `ray.rs`: Ray definition
- `hittable.rs`: Traits for objects that can be hit by rays
- `sphere.rs`: Sphere implementation
- `camera.rs`: Camera model
- `material.rs`: Material definitions (Lambertian, Metal, Dielectric)

## Performance
The renderer utilizes Rayon for parallel processing to speed up the rendering process. For better performance:

```shell
# Build with optimizations
cargo build --release

# Run with optimizations
cargo run --release
```