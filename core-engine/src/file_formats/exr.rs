use glam::{Vec3};
use exr::prelude::*;

use crate::ImageBuffer;

/// HDR skybox loaded from EXR
#[derive(Default)]
pub struct ExrImage {
    pub rgb : ImageBuffer<Vec3>,
}

impl ExrImage {
    pub fn load_exr_image(path: &str) -> Result<ExrImage> {
        let image_2d = exr::prelude::read_first_rgba_layer_from_file(
            path,
            |resolution, _| {
                let image_data = vec![Vec3::ZERO; resolution.width() * resolution.height()];
                ExrImage {
                    rgb: ImageBuffer {
                        buffer: image_data,
                        width: resolution.width(),
                        height: resolution.height(),
                    },
                }
            },
            |exr, pos, (r, g, b, _): (f32, f32, f32, f32)| {
                // skybox.pixels_buffer[pos.y() * skybox.width + pos.x()] = Vec3::new(r, g, b);
                exr.rgb.set_pixel(pos.x(), pos.y(), Vec3::new(r, g, b));
            },
        );

        match image_2d {
            Ok(img) => {
                let skybox = img.layer_data.channel_data.pixels;
                Ok(skybox)
            }
            Err(err) => Err(err),
        }
    }

    pub fn save_to_files(&self, path: &str) {
        
        // fn vec3_to_planes(v: &[Vec3]) -> [Vec<f32>; 3] {
        //     let mut x = Vec::with_capacity(v.len());
        //     let mut y = Vec::with_capacity(v.len());
        //     let mut z = Vec::with_capacity(v.len());
        //     for c in v {
        //         x.push(c.x);
        //         y.push(c.y);
        //         z.push(c.z);
        //     }
        //     [x, y, z]
        // }

        // let rgba_planes = vec3_to_planes(self.rgb.buffer.as_slice());
        // let alpha = vec![1.0f32; self.rgb.width * self.rgb.height];
        let [size_x, size_y]= self.rgb.get_dimensions();
        let size = (size_x as usize, size_y as usize);

        let rgba_layer = Layer::new(
            size,
            LayerAttributes::named("beauty"),
            Encoding::SMALL_LOSSLESS,
            SpecificChannels::rgba(|pos:Vec2<usize>| {
                let flipped_y = size_y as usize - 1 - pos.y();
                let radiance = self.rgb[(pos.x(), flipped_y)];
                (radiance.x, radiance.y, radiance.z, 1.0)
            })
                // SpecificChannels::build()
        // .with_channel("L").with_channel("B")
        // .with_pixel_fn(|position: Vec2<usize>| {
            // let (l, b) = my_image.lookup_color_at(position.x(), position.y());
            // (l as f32, f16::from_f32(b))
        // };
        );
        // let normal_planes = vec3_to_planes(normal);
        // let albedo_planes = vec3_to_planes(albedo);
        
        let attributes = ImageAttributes::new(
            IntegerBounds::from_dimensions(size)
        );
        let mut layers = SmallVec::new();
        layers.push(rgba_layer);
        
        let mut image = Image::from_layers(
            attributes,
            layers
        );

        image.attributes.other.insert(
            Text::from("Renderer"),
            AttributeValue::Text("InsploRay".into())
        );

        image.write().to_file(path).unwrap();
    }

}
