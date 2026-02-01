use std::sync::{Arc};
use std::time::{Duration, Instant};

use crossbeam::channel::Receiver;
use glam::{Vec3};

use crate::accumulators::{Accumulator, TileAccumulator};
use crate::cameras::{PinholeCamera, SharedCamera, SharedCameraBox};
use crate::concurrency::Threadpool;
use crate::file_formats::ExrImage;
use crate::integrator::Integrator;
use crate::scene::Scene;

pub struct RayTracer {
    width: u32,
    height: u32,
    frame_buffer: Vec<u32>,
    last_render_time: Duration,

    pub active_camera: SharedCamera,
    integrator: Integrator,
    accumulator: Accumulator,
    threadpool: Option<Threadpool>,
    threadpool_result_rx: Option<Receiver<TileAccumulator>>,
}

impl RayTracer {
    // temperory function
    pub fn save_exr(&self, path: &str) {
        let buff = self.accumulator.get_image_buffer();

        let exr = ExrImage {
            rgb: buff
        };

        exr.save_to_files(path);
    }

    pub fn set_tp_size(&mut self, threads: usize) {
        if let Some(tp) = self.threadpool.take() {
            drop(tp);
        }
        if let Some(rx) = self.threadpool_result_rx.take() {
            drop(rx);
        }
        let (tp, result_rx) = Threadpool::new(threads);
        self.threadpool = Some(tp);
        self.threadpool_result_rx = Some(result_rx);
    }

    pub fn new(width: u32, height: u32) -> Self {
        let camera = PinholeCamera::new(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::ZERO,
            35.0,
            55.0,
            [width, height],
        );

        let integrator = Integrator {
            bounces: 10,
            max_compulsory_bounces: 5,
        };
        let accumulator = Accumulator::new(width, height);
        let shared_acc = accumulator;

        Self {
            width: 0,
            height: 0,
            frame_buffer: vec![],
            active_camera: Arc::new(camera),
            last_render_time: Duration::from_secs(0),
            accumulator: shared_acc,

            integrator,
            threadpool: None,
            threadpool_result_rx: None,
        }
    }

    pub fn set_active_camera(&mut self, camera: SharedCamera) {
        self.active_camera = camera;
    }

    pub fn get_current_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    #[inline]
    pub fn prepare_pixels(&mut self, scene: &Arc<Scene>) {
        self.render(scene);
    }

    fn set_size(&mut self, size: [u32; 2]) {
        self.width = size[0];
        self.height = size[1];

        if self.accumulator.get_resolution() != size {
            self.accumulator = Accumulator::new(size[0], size[1]);
        }

        if self.active_camera.get_image_resolutions() != size {
            let mut new_cam : SharedCameraBox = self.active_camera.clone_box();
            new_cam.set_image_resolutions(size);
            self.set_active_camera(Arc::from(new_cam));

        }
    }

    fn dispatch_tile_render_job(
        &mut self,
        scene: &Arc<Scene>,
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
            let mut accumulator = TileAccumulator::new(tile_x, tile_y, tile_width, tile_height);

            for dy in 0..tile_height {
                for dx in 0..tile_width {
                    let x = tile_x + dx;
                    let y = tile_y + dy;

                    let color =
                        integrator.compute_incomming_radience(&local_scene, x, y, camera.as_ref(), sampler);

                    accumulator.accumulate(dx as u32, dy as u32, color);
                }
            }

            accumulator
        });

        true
    }

    pub fn render(&mut self, scene: &Arc<Scene>) {
        let render_start_time = Instant::now();

        let tile_size = 32;
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
                self.accumulator.merge_tile(tile_acc);
            }
        }

        self.last_render_time = render_start_time.elapsed();
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.set_size([width, height]);
        self.accumulator = Accumulator::new(width, height);
    }

    pub fn get_output(&mut self) -> &[u32] {
        self.accumulator.write_to_image_buffer(&mut self.frame_buffer);
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
