use crate::base::Camera;
use std::sync::Arc;

pub type SharedCamera = Arc<dyn Camera + Send + Sync>;
pub type SharedCameraBox = Box<dyn Camera + Send + Sync>;

pub mod pinhole_camera;
pub use pinhole_camera::PinholeCamera;
