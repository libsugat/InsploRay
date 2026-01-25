use glam::Vec3;

use crate::geometry::{Mesh, Triangle};
use crate::materials::Material;
use crate::materials::shaders::BxDFImpl;

use tobj::{Mesh as TobjMesh, Material as TobjMaterial};

pub fn load_from_file(path: &str) ->Result<(Vec<Mesh>, Vec<Material>), Box<dyn std::error::Error>> {

    // Load the OBJ file (single-threaded, no material loading)
    let (models, tobj_materials_res) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let mut materials: Vec<Material> = Vec::new();
    if let Ok(tobj_materials) = tobj_materials_res {
        for mat in tobj_materials {
            let material = convert_material(&mat);
            materials.push(material);
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
    let emmissive = mat.unknown_param.get("Ke");
    let bxdf = crate::materials::Lambertian {
        // name: mat.name.clone(),
        albedo: match mat.diffuse {
            None => Vec3::ONE,
            Some(diffuse) => {
                Vec3::from_array(diffuse)
            }
        },
        emission_color: match emmissive {
            None => Vec3::ZERO,
            Some(values_string) => {
                let nums : Vec<f32> = values_string.split_whitespace()
                    .map(|num_str| num_str.parse::<f32>().unwrap())
                    .collect();
                Vec3::new(nums[0], nums[1], nums[2])
            }
        },
        emissive_power: 1.0, // max component as power
    };

    Material {
        shaders: vec![BxDFImpl::Lambertian(bxdf)],
        weights: vec![1.0]
    }
}
