use std::sync::Arc;

use glam::Vec3;

use crate::geometry::{Triangle, TriangleMesh};
use crate::materials::Material;
use crate::scene::Scene;

use tobj::{Mesh as TobjMesh, Material as TobjMaterial};

/// This is used to load the mesh and material data from obj scene format
/// 
/// It should be taken care that these function is called before building bvh
pub fn load_from_obj(scene: &mut Scene, path: &str) ->Result<(), Box<dyn std::error::Error>> {
    let (models, tobj_materials_res) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let old_material_length = scene.materials.len();
    let old_mesh_buff_length = scene.meshes.len();

    let mut materials: Vec<Arc<Material>> = Vec::new();
    if let Ok(tobj_materials) = tobj_materials_res {
        for mat in tobj_materials {
            let material = convert_material(&mat);
            materials.push(Arc::new(material));
        }
    }

    let mut meshes = Vec::new();
    let mut tris = Vec::new();

    for (mesh_i, model) in models.into_iter().enumerate() {
        let mesh: &TobjMesh = &model.mesh;

        let mut ver_pos = Vec::with_capacity(mesh.positions.len() / 3);
        for i in (0..mesh.positions.len()).step_by(3) {
            let x = mesh.positions[i];
            let y = mesh.positions[i+1];
            let z = mesh.positions[i+2];
            ver_pos.push(Vec3::new(x, y, z));
        }
        let mut ver_nor = Vec::with_capacity(mesh.normals.len() / 3);
        for i in (0..mesh.normals.len()).step_by(3) {
            let x = mesh.normals[i];
            let y = mesh.normals[i+1];
            let z = mesh.normals[i+2];
            ver_nor.push(Vec3::new(x, y, z));
        }

        let mut triangles = Vec::with_capacity(mesh.indices.len()/3);

        for i in (0..mesh.indices.len()).step_by(3) {
            let t = Triangle::new(
                mesh.indices[i],
                mesh.indices[i + 1],
                mesh.indices[i + 2],
                (mesh_i + old_mesh_buff_length) as u32
            );
            triangles.push(t);
        }

        let my_mesh = TriangleMesh {
            name: model.name,
            id: (mesh_i + old_mesh_buff_length) as u32,
            vertices: ver_pos,
            normals: ver_nor,
            material_id: mesh.material_id.map(|m_id| m_id + old_material_length)
        };

        meshes.push(my_mesh);
        tris.extend(triangles);
    }

    scene.meshes.extend(meshes);
    if let Some(tris_buffer) = &mut scene.tris_vec {
        tris_buffer.extend(tris);
    }
    else {
        scene.tris_vec = Some(tris);
    }
    scene.materials.extend(materials);

    Ok(())
}

fn convert_material(mat: &TobjMaterial) -> crate::materials::Material {
    let emmissive = mat.unknown_param.get("Ke");
    let bxdf = crate::materials::shaders::Lambertian {
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
        shaders: vec![Arc::new(bxdf)],
        weights: vec![1.0]
    }
}
