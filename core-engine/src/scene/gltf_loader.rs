use std::sync::Arc;

use crate::geometry::Mesh;
use crate::materials::BxDF;

pub fn load_from_file(_path: &str) ->Result<(Vec<Mesh>, Vec<Arc<dyn BxDF>>), Box<dyn std::error::Error>> {
    todo!()
}

