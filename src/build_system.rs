use std::{convert::TryFrom, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum BuildSystem {
    /// File to look for: Cargo.toml
    Cargo,
    /// File to look for: pom.xml
    Maven,
    /// Files to look for: *.pro
    QMake,
    /// Files to look for: CMakeLists.txt
    CMake,
    /// File to look for: package.json
    Node,
    /// File to look for: Makefile, makefile or GNUmakefile
    Make,
}

fn check_extension(path: &PathBuf) -> Result<BuildSystem, ()> {
    let extension = path.extension().ok_or(())?;
    match extension {
        ext if ext == "pro" => Ok(BuildSystem::QMake),
        // TODO: moar?
        _ => Err(()),
    }
}

impl TryFrom<&PathBuf> for BuildSystem {
    type Error = ();

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        match path.as_os_str() {
            p if p == "Cargo.toml" => Ok(Self::Cargo),
            p if p == "CMakeLists.txt" => Ok(Self::CMake),
            p if p == "pom.xml" => Ok(Self::Maven),
            p if p == "package.json" => Ok(Self::Node),
            p if p == "Makefile" => Ok(Self::Make),
            p if p == "makefile" => Ok(Self::Make),
            p if p == "GNUmakefile" => Ok(Self::Make),
            _ => check_extension(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, path::PathBuf};

    use crate::build_system::BuildSystem;

    #[test]
    fn parse_build_system_from_file_path() {
        use std::convert::TryInto;
        
        let files: Vec<BuildSystem> = vec![
            "Cargo.toml",
            "CMakeLists.txt",
            "pom.xml",
            "package.json",
            "Makefile",
            "makefile",
            "GNUmakefile",
        ]
        .into_iter()
        .map(PathBuf::from)
        .map(|path| (&path).try_into())
        .map(Result::unwrap)
        .collect();
        
        assert_eq!(
            files,
            vec![
                BuildSystem::Cargo,
                BuildSystem::CMake,
                BuildSystem::Maven,
                BuildSystem::Node,
                BuildSystem::Make,
                BuildSystem::Make,
                BuildSystem::Make
            ]
        );
    }
}
