use crate::build_system::BuildSystem;
use crate::manifest::Manifest;

/// Represents the usage of a given build system
pub struct Usage<'a> {
    build: &'a str,
    run: Option<&'a str>,
}

impl<'a> From<Manifest> for Usage<'a> {
    fn from(manifest: Manifest) -> Self {
        match manifest.build_system() {
            BuildSystem::Cargo => Self {
                build: "build",
                run: Some("run"),
            },
            BuildSystem::Stack => Self {
                build: "build",
                run: Some("exec")
            },
            BuildSystem::Maven => Self {
                build: "package",
                // TODO: Check for `<exec.mainClass>` in pom.xml
                // Reference: https://stackoverflow.com/questions/1089285/maven-run-project
                run: Some("exec:java"),
            },
            BuildSystem::QMake => Self {
                build: "&& make",
                // TODO: check for file stem of the QMake file
                // e.g.: if the file is named qmake.pro, then the resulting
                // binary will be named `qmake`
                run: None,
            },
            BuildSystem::CMake => Self {
                build: ".",
                run: None,
            },
            BuildSystem::Node => todo!(),
            BuildSystem::Make => Self {
                // TODO: parse Make targets
                build: "",
                run: None,
            },
        }
    }
}
