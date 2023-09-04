use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use zip::ZipArchive;
use crate::error;

pub struct Forge;

impl Forge {
    pub fn get_args() -> Vec<String> {
        let mut returner = Vec::new();
        let mut reader = ZipArchive::new(File::open(Path::new("libraries/net/minecraftforge/forge.jar")).unwrap_or_else(|_| {
            error!("Jar not found!");
            exit(1);
        })).unwrap();
        let mut contents = String::new();
        reader.by_name("data/unix_args.txt").unwrap().read_to_string(&mut contents).unwrap();
        contents.split('\n').for_each(|arg| {
            returner.push(arg.to_string())
        });
        returner
    }
}