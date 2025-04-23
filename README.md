# Overview
Ferox is a ray-tracing renderer written in Rust. 

Currently, it is capable of detecting ray intersections with simple spheres and displaying those calculations in PNG form.

This project is made possible with the help of assets like https://www.scratchapixel.com and ssloy's tiny ray tracer (https://github.com/ssloy/tinyraytracer).

I've licensed ferox under a CC0 license with the intention of making it as freely accessible as possible. You can use it for whatever. I am using (read: creating) it to learn Rust and graphics programming.

# Usage
The crate can be installed using `cargo install ferox`. 

Ferox supports loading custom scene and environment map configurations with the `-s` (for "scene") and `-b` (for "background") arguments respectively.
- The scene argument accepts JSON inputs with a specific schema (to be specified in the documentation at a later junction. For now, you can use the `scene.json` within the repository as a guide).
- The background argument leverages the `image` crate to load most valid image data types, including `.hdr` files.

The default image output produces a file in the calling directory called `out.png`. This can be adjusted with the `-o` (for "output") argument. It should support most valid image data types.

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
- [x] Additional object types and meshes
- [x] Loadable scene configurations
- [x] CLI image generation and output options
- [x] Multi-threading (parallelization with Rayon)
- [ ] Loadable custom meshes
- [ ] Colored lighting
- [ ] Texture map support
- [ ] Acceleration structures
- [ ] GUI with parametric support
- [ ] Real-time calculations including camera movement
- [ ] Antialiasing
- [ ] Shading improvements
- [ ] Rasterization-based rendering options
- [ ] Refactor codebase for readability (this may be an endless endeavor)
