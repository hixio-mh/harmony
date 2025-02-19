/// A reference to the mesh.
#[derive(Default)]
pub struct Mesh {
    pub mesh_name: String,
}

impl Mesh {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            mesh_name: name.into(),
        }
    }
}
