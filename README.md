# InsploRay (Path Tracer)

InsploRay is a CPU based path tracing renderer. 

_InsploRay: Inspire(inspiration) + Explore(Exploration) + Ray Tracing_

The primary goal of writing a path tracer was to get a head start before getting into low level systems programming, which now grew into a project of its own. It’s being designed with modularity in mind.

## 🧩 Current Features:
- Ray sphere intersection
- Watertight Ray/Triangle Intersection _([Reference](https://jcgt.org/published/0002/01/05/paper.pdf))_
- Lambertian Diffuse
- EXR skybox support _(for HDR environment lighting and background)_
- Multithreaded
- Simulate a PinHole Camera
- Very Basic material system 
    - Albedo
    - Emissive Color
    - Emissive Strength
    - Metallic _(only Isotropic)_
- BVH (Binned SAH building)
- Basic Tone Mapping
- More under way✨...

⚠️ _Limitations and caveats apply — see below._

## ⚙️ Installation and Usage

#### Step 1: Clone the repository
```bash
  git clone https://github.com/libsugat/InsploRay.git
  cd InsploRay
```
#### Step 2: Compile/Build and run the Code
```bash
  cargo run # for debug build
```
or
```bash
  cargo run --release # for release/optimized build
```

## 🧰 Project Setup and Development
The project is managed as a `Cargo Workspace` keeping the core logic isolated from the interfaces.

| Crate        | Location     |Status      |Description                                                   |
|--------------|--------------|------------|--------------------------------------------------------------|
| Core Engine  |`/core-engine`|Active      |The heart of the renderer. Handles rays, BVH, and materials.  |
| Cli          |`/cli`        |Main        |The primary interface for offline rendering and scene loading.|
| Frontend     |`/frontend`   |_Deprecated_|Experimental ImGui-based interactive viewport.                |

## 🧭 How to Contribute

- **Open an issue first** if you want to work on something — bug, feature, or idea. (Just make sure there are no duplicate issues.)
- Then, make the changes and **open a pull request** (PR).
- Keep PRs small and focused if possible — it makes things easier to understand.    
- Code formatting using `clippy` is appreciated.

>⚠️ **Few small constraints**:  
> Please keep performance and readability in mind.  
> Avoid excessive type casting — e.g., use `usize` only for indexing; otherwise prefer types like `u32`, etc. Only use `u32` for indexing when it comes to memory footprint
> Maintain clean build (no build warnings)

That’s it for now! No strict rules — I’m here to learn too, and happy to figure things out with you as we go. 💬

Feel free to ask questions, suggest changes, or just explore the code!

## 🐛 Known issue & Limitations
- [ ] The specular bsdf is incorrect
- [ ] Textures are not supported
- [ ] Camera is currently hardcoded, rightnow.
- [ ] As an experiment of NEE _(Next Event Estimation)_, the top point light is also hardcoded in the current Integrator
- [ ] Does not have a proper scene and material loader, `obj` is a current work around
- [ ] Does not save normal and albedo buffers for denoising 

## 🔜 My Side Plans
Order unknown because I am BTech student unable to manage my time currently....
- [x] Ray Triangle Intersection
- [x] Loading Scene (`.obj`)
- [x] Metallic BRDF (Isotropic)
- [x] Save Image (`EXR` and/or `PNG`)
- [x] BVH (Binned SAH)
- [ ] Loading Scene (`.glb`/`.gltf`)
- [ ] Textures Support
- [ ] Specular BRDF
- [ ] Metallic BRDF (Anotropic)
- [ ] Better Scene Representation in memory
- [ ] MIS (Multiple Importance Sampling) in Primary (or currently only) Integrator
- [ ] Integration of [Intel Embree](https://www.embree.org/) as an optional acceleration backend.

## License

Licensed under [AGPLv3](./LICENSE).  
For closed-source, commercial, SaaS, or academic use without attribution, please contact via Github Issues

## 👤 Author
This project was started by me ([@libsugat](https://www.github.com/libsugat))
— who knew **nothing** about rendering or graphics programming when it began!

