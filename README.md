# Overview
Ferox is a ray-tracing renderer written in Rust. 

Currently, it is capable of detecting ray intersections with simple spheres and displaying those calculations in PNG form.

This project is made possible with the help of assets like https://www.scratchapixel.com and ssloy's tiny ray tracer (https://github.com/ssloy/tinyraytracer).

# Usage
The crate can be installed using `cargo install ferox`. 

In its current state, running `ferox` will just render and output the default scene as specified in `main.rs` in whatever directory ferox is called from.

To experiment with the engine, it is recommended to download the source code and adjust the scene parameters manually.

I am planning on adding support for JSON-based scene loading and overrideable output settings, but those will be secondary to the core raytracing features that remain to be implemented.

# Roadmap
- [x] Image output
- [x] Sphere outlines
- [x] Sphere materials
- [x] Lighting
- [x] Specularity
- [x] Shadows
- [x] Reflections
- [x] Refraction
- [x] Environment map support
- [ ] Additional object types and meshes
- [ ] Refactor codebase for readability
- [ ] Loadable scene configurations
- [ ] CLI image generation and output options
- [ ] GUI with parametric support
- [ ] Real-time calculations and camera movement
