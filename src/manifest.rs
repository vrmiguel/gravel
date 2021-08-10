use std::path::{PathBuf};

use crate::build_system::BuildSystem;

#[derive(Debug)]
pub struct Manifest {
    path: PathBuf,
    build_system: BuildSystem
}

impl Manifest {
    pub fn new(path: PathBuf, build_system: BuildSystem) -> Self {
        Self {
            path,
            build_system 
        }
    }

    pub fn build_system(&self) -> BuildSystem {
        self.build_system.clone()
    }
}