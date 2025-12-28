use std::sync::Arc;

use glam::Vec3;

use crate::geometry::{Mesh, Triangle};
use crate::materials::Material;

use tobj::{Mesh as TobjMesh, Material as TobjMaterial};

pub fn load_from_file(path: &str) ->Result<(Vec<Mesh>, Vec<Arc<Material>>), Box<dyn std::error::Error>> {

    // Load the OBJ file (single-threaded, no material loading)
    let (models, tobj_materials_res) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let mut materials: Vec<Arc<Material>> = Vec::new();
    if let Ok(tobj_materials) = tobj_materials_res {
        for mat in tobj_materials {
            let material = convert_material(&mat);
            materials.push(Arc::new(material));
        }
    }

    let mut meshes = Vec::new();

    for model in models.into_iter() {
        let mesh: &TobjMesh = &model.mesh;
        let mut triangles = Vec::with_capacity(mesh.indices.len()/3);

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            let v0 = Vec3::new(
                mesh.positions[3 * i0],
                mesh.positions[3 * i0 + 1],
                mesh.positions[3 * i0 + 2],
            );

            let v1 = Vec3::new(
                mesh.positions[3 * i1],
                mesh.positions[3 * i1 + 1],
                mesh.positions[3 * i1 + 2],
            );
            let v2 = Vec3::new(
                mesh.positions[3 * i2],
                mesh.positions[3 * i2 + 1],
                mesh.positions[3 * i2 + 2],
            );

            triangles.push(Triangle::new((v0, v1, v2), match mesh.material_id {
                None => {
                    -1
                },
                Some(mat_id) => {
                    mat_id as i32
                }
            }));
        }

        meshes.push(Mesh::new(model.name, triangles));
    }

    Ok((meshes, materials))
}

fn convert_material(mat: &TobjMaterial) -> crate::materials::Material {
    let bxdf = crate::materials::Lambertian {
        // name: mat.name.clone(),
        albedo: match mat.diffuse {
            None => Vec3::ONE,
            Some(diffuse) => {
                Vec3::from_array(diffuse)
            }
        },
        // roughness: match mat.shininess {
        //     None => 0.0,
        //     Some(shininess) => {
        //         1.0 - shininess.min(1000.0) / 1000.0 // convert specular exponent to roughness
        //     }
        // },
        // metalic: mat.illumination_model.unwrap_or(0) as f32 / 10.0, // crude approximation
        emission_color: Vec3::ZERO,
        emissive_power: 0.0, // max component as power
    };

    Material {
        shaders: vec![Arc::new(bxdf)],
        weights: vec![1.0]
    }
}
