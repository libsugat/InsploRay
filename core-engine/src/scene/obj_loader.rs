use std::sync::Arc;

use glam::Vec3;

use crate::geometry::{Triangle, TriangleMesh};
use crate::materials::Material;

use tobj::{Mesh as TobjMesh, Material as TobjMaterial};

pub fn load_from_file(path: &str) ->Result<(Vec<TriangleMesh>, Vec<Triangle>, Vec<Arc<Material>>), Box<dyn std::error::Error>> {

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
    let mut tris = Vec::new();

    let mut mesh_counter = 0; 
    for model in models.into_iter() {
        let mesh: &TobjMesh = &model.mesh;

        // println!("{}", &model.name);
        // println!("  id   : {}", mesh_counter);
        // println!("  vecs : {}", mesh.positions.len() / 3);
        // println!("  indi : {}", mesh.indices.len() / 3);

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
                // mesh.material_id,
                mesh_counter
            );
            triangles.push(t);
        }

        let my_mesh = TriangleMesh {
            name: model.name,
            id: mesh_counter,
            vertices: ver_pos,
            normals: ver_nor,
            material_id: mesh.material_id
        };

        meshes.push(my_mesh);
        tris.extend(triangles);
        mesh_counter += 1;
    }

    Ok((meshes, tris, materials))
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
