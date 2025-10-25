use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crossbeam::channel::Receiver;
use glam::Vec3;

use crate::accumulators::{Accumulator, TileAccumulator};
use crate::cameras::{PinholeCamera, SharedCamera};
use crate::concurrency::Threadpool;
use crate::integrator::Integrator;
use crate::scene::Scene;

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer: Vec<u32>,
    last_render_time: Duration,

    pub active_camera: SharedCamera,
    // pub scene : Arc<Scene>
    integrator: Integrator,
    accumulator: Arc<RwLock<Accumulator>>,
    threadpool: Option<Threadpool>,
    threadpool_result_rx: Option<Receiver<TileAccumulator>>,
}

impl RayTracer {
    pub fn new(width: u32, height: u32) -> Self {
        let camera = PinholeCamera::new(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::ZERO,
            35.0,
            55.0,
            [width, height],
        );

        let integrator = Integrator {
            bounces: 5,
            max_compulsory_bounces: 2,
        };
        let accumulator = Accumulator::new(width, height);
        let shared_acc = Arc::new(RwLock::new(accumulator));

        let (tp, result_rx) = Threadpool::new(4);

        Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera: Arc::new(RwLock::new(camera)),
            last_render_time: Duration::from_secs(0),
            accumulator: shared_acc,

            integrator,
            threadpool: Some(tp),
            threadpool_result_rx: Some(result_rx),
        }
    }

    pub fn set_active_camera(&mut self, camera: SharedCamera) {
        self.active_camera = camera;
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    #[inline]
    pub fn prepare_pixels(&mut self, scene: &Arc<RwLock<Scene>>) {
        self.render(scene);
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];

        let mut accum_guard = self.accumulator.write().unwrap();
        if accum_guard.get_resolution() != size {
            *accum_guard = Accumulator::new(size[0], size[1]);
        }
        drop(accum_guard);

        let mut cam = self.active_camera.write().unwrap();
        cam.set_image_resolutions(size);
        drop(cam);
    }

    fn dispatch_tile_render_job(
        &mut self,
        scene: &Arc<RwLock<Scene>>,
        tile_size: u32,
        tile_x: u32,
        tile_y: u32,
    ) -> bool {
        let tp = match &mut self.threadpool {
            Some(threadpool) => threadpool,
            None => {
                return false;
            }
        };

        let mut integrator = self.integrator.clone();
        let camera = Arc::clone(&self.active_camera);
        let local_scene = Arc::clone(scene);

        // Compute tile bounds
        let tile_width = (tile_size).min(self.width - tile_x);
        let tile_height = (tile_size).min(self.height - tile_y);

        tp.execute(move |sampler| {
            let scene_guard = local_scene.read().unwrap();
            let mut accumulator = TileAccumulator::new(tile_x, tile_y, tile_width, tile_height);

            for dy in 0..tile_height {
                for dx in 0..tile_width {
                    let x = tile_x + dx;
                    let y = tile_y + dy;

                    let color =
                        integrator.compute_incomming_radience(&scene_guard, x, y, &camera, sampler);

                    accumulator.accumulate(dx as u32, dy as u32, color);
                }
            }

            accumulator
        });

        true
    }

    pub fn render(&mut self, scene: &Arc<RwLock<Scene>>) {
        let render_start_time = Instant::now();

        let tile_size = 64;
        let mut jobs_dispached = 0;

        for tile_y in (0..self.height).step_by(tile_size as usize) {
            for tile_x in (0..self.width).step_by(tile_size as usize) {
                jobs_dispached +=
                    self.dispatch_tile_render_job(scene, tile_size, tile_x, tile_y) as u32;
            }
        }

        for _ in 0..jobs_dispached {
            let job_result = self.threadpool_result_rx.as_ref().unwrap().recv();
            if let Ok(tile_acc) = job_result {
                let mut acc_guard = self.accumulator.write().unwrap();
                acc_guard.merge_tile(tile_acc);
                drop(acc_guard);
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.set_size([width, height]);
        let mut accum_guard = self.accumulator.write().unwrap();
        *accum_guard = Accumulator::new(width, height);
    }

    pub fn get_output(&mut self) -> &[u32] {
        let accum_guard = self.accumulator.read().unwrap();
        accum_guard.write_to_image_buffer(&mut self.frame_buffer);
        drop(accum_guard);
        &self.frame_buffer
    }

    pub fn get_last_render_time(&self) -> Duration {
        self.last_render_time
    }
}

impl Default for RayTracer {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Drop for RayTracer {
    fn drop(&mut self) {
        if let Some(tp) = self.threadpool.take() {
            drop(tp);
        }
        if let Some(rx) = self.threadpool_result_rx.take() {
            drop(rx);
        }
    }
}
