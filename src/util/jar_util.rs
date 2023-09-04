use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use serde_yaml::Value;
use zip::ZipArchive;
use crate::error;

pub fn get_main_class() -> String {
    let mut reader = get_jar_reader();
    let mut meta = match reader.by_name("META-INF/MANIFEST.MF") {
        Ok(meta) => {
            meta
        },
        Err(err) => {
            error!("Cannot read manifest! Error: {}", err);
            exit(1)
        }
    };
    let mut contents = "".to_string();
    meta.read_to_string(&mut contents).unwrap();
    let meta_parsed: Value = match serde_yaml::from_str(&contents) {
        Ok(meta_parsed) => {
            meta_parsed
        },
        Err(err) => {
            error!("Failed to parse meta! Error: {}", err);
            exit(1)
        }
    };
    match meta_parsed.get("Main-Class") {
        Some(class) => {
            class.as_str().unwrap().to_string().replace('.', "/" )
        },
        None => {
            error!("The meta doesnt contain a main class? what");
            exit(1)
        }
    }
}

pub fn get_jar_reader() -> ZipArchive<File> {
    let jar_loc = env::var("JAR_LOCATION").unwrap_or_else(|_e| {
        error!("JAR_LOCATION env variable needs to be reset!");
        exit(1);
    });
    ZipArchive::new(File::open(Path::new(&jar_loc)).unwrap_or_else(|_| {
        error!("Jar not found!");
        exit(1);
    })).unwrap()
}