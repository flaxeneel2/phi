use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use graceful::SignalGuard;
use jni::{InitArgsBuilder, JavaVM, JNIEnv, JNIVersion};
use jni::objects::{JObject, JObjectArray, JString, JValue};
use jni::sys::jobjectArray;
use once_cell::sync::OnceCell;
use serde_yaml::Value;
use zip::ZipArchive;
use crate::modules::inactivity::Inactivity;
use crate::servers::bukkit::bukkit::Bukkit;
use crate::servers::bukkit::entity::player::Player;
use crate::servers::bukkit::plugin::plugin::Plugin;

mod util;
mod servers;
mod modules;

static JVM: OnceCell<JavaVM> = OnceCell::new();

#[tokio::main]
async fn main() {
    let main_class = get_main_class();
    run_jar(main_class).await;
}

async fn run_jar(main_class: String) {
    let signal_guard = SignalGuard::new();
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Xmx4G")
        .option("-Xms128M")
        .option("-Dterminal.jline=false")
        .option("-Dterminal.ansi=true")
        .option("-Djava.class.path=/home/flaxeneel2/Documents/projects/rust/phi/server/server.jar")
        .option("-javaagent:/home/flaxeneel2/Documents/projects/rust/phi/server/server.jar")
        .build()
        .unwrap();
    let jvm = JavaVM::new(jvm_args).unwrap();
    JVM.set(jvm).unwrap();
    let mut env = jni();
    let arg_str = env.new_string("nogui").unwrap();
    let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
    let jobj_arg = unsafe {
        JObject::from_raw(**arg)
    };
    env.call_static_method(main_class, "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg).into()]).expect("ERR");
    log!("Loaded!");
    Inactivity::activate(30);
    signal_guard.at_exit(move |sig| {
        log!("Shutting down...");
        Bukkit::stop_server();
    });
}

fn get_main_class() -> String {
    let mut reader = ZipArchive::new(File::open(Path::new("/home/flaxeneel2/Documents/projects/rust/phi/server/server.jar")).unwrap()).unwrap();
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
    let meta_parsed: Value = match serde_yaml::from_str(&*contents) {
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
            class.as_str().unwrap().to_string().replace(".", "/" )
        },
        None => {
            error!("The meta doesnt contain a main class? what");
            exit(1)
        }
    }
}

fn jni() -> JNIEnv<'static> {
    JVM.get().unwrap().attach_current_thread_permanently().unwrap()
}