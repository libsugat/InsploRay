use std::sync::Arc;

use imgui::Ui;

use insploray::renderer::RayTracer;
use insploray::scene::Scene;
use insploray::cameras::Camera;
use insploray::cameras::PinholeCamera;
use insploray::Vec3;

pub struct Viewport {
    dimensions : [u32; 2],
    pub renderer : RayTracer,
    pub scene : Arc<Scene>,
    pub camera : Arc<PinholeCamera>,
}

impl Viewport {

    pub fn set_dimensions(&mut self, width : u32, height : u32) {
        self.dimensions = [width, height];
        if self.renderer.get_current_size() != self.dimensions {
            self.renderer.update(width, height);
        }
    }

    pub fn prepare_buffer(&mut self) {
        self.renderer.render(&self.scene);
    }

    pub fn get_buffer(&mut self) -> &[u32] {
        self.renderer.get_output()
    }

    pub fn draw_scene_setting_window(&mut self, ui : &Ui, _viewport_size: &[f32; 2]) {

        ui.window("Scene Settings")
            .size([300.0, 400.0], imgui::Condition::FirstUseEver)
            .position([200.0, 500.0], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("Get Camera Setting") {
                    let cam = &self.camera;
                    println!("Camera:");
                    println!("\tposition : {:?}", cam.position);
                    println!("\trotation : {:?}", cam.rotation);
                    println!("\timage_size : {:?}", cam.image_size);
                    println!("\tfocallength : {:?}", cam.focal_length);
                    println!("\tsensorsize : {:?}", cam.sensor_size);
                }
            });


    }

}

impl Default for Viewport {
    fn default() -> Self {
        let position = Vec3::new(9.5, 2.25, 0.0);
        // let position = Vec3::new(7.67618, 3.2679, 1.66982);
        // let rotation = Vec3::new(0.0, 1.30277, 0.22899);
        let rotation = Vec3::new(0.0, std::f32::consts::PI / 2.0, 0.0);
        let mut cam = PinholeCamera::new(
            position, 
            Vec3::ZERO,
            // 55.0,
            // 35.0,
            50.0,
            36.0,
            [0,0]
        );
        cam.set_rotation(rotation);
        let camera =  Arc::new(cam);

        let mut renderer = RayTracer::new(0, 0);
        renderer.set_tp_size(4);
        renderer.set_active_camera(camera.clone());
        let mut scene = Scene::get_example_scene();
        scene.build_bvh();

        Self {
            dimensions: [0,0],
            camera,
            renderer,
            scene: Arc::new(scene),
        }
    }
}
