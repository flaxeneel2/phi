use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use zip::ZipArchive;
use crate::{error, log, warn};
use crate::util::jar_util::{get_jar_reader, get_main_class};

#[derive(Debug, PartialEq)]
pub enum ServerType {
    Bukkit,
    Forge,
    Fabric
}

impl ServerType {
    pub fn detect() -> Self {
        if Path::new("libraries/net/minecraftforge/forge.jar").exists() {
            let mut reader = ZipArchive::new(File::open(Path::new("libraries/net/minecraftforge/forge.jar")).unwrap_or_else(|e| {
                error!("{:?}", e);
                exit(1);
            })).unwrap();
            if reader.by_name("data/unix_args.txt").is_ok() {
                return Self::Forge
            }
        } else {
            let jar_loc = env::var("JAR_LOCATION").unwrap_or_else(|_e| {
                error!("jar location not set!");
                exit(1)
            });
            if Path::new(&jar_loc).exists() {
                let class = get_main_class();
                if class.eq_ignore_ascii_case("net/fabricmc/installer/ServerLauncher") {
                    return Self::Fabric
                }
                if class.contains("io/papermc") {
                    return Self::Bukkit
                }
                let mut reader = get_jar_reader();
                if reader.by_name("org/bukkit/Bukkit").is_ok() {
                    return Self::Bukkit
                }
            }
        }
        warn!("Unknown jar type! Assuming it is bukkit");
        Self::Bukkit
    }
}