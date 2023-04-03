use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use jni::{InitArgsBuilder, JavaVM, JNIEnv, JNIVersion};
use jni::objects::{JObject, JObjectArray, JString, JValue};
use jni::sys::jobjectArray;
use once_cell::sync::OnceCell;
use serde_yaml::Value;
use zip::ZipArchive;
use crate::plugins::bukkit::plugin::Plugin;

mod util;
mod plugins;

static JVM: OnceCell<JavaVM> = OnceCell::new();

#[tokio::main]
async fn main() {
    let main_class = get_main_class();
    run_jar(main_class).await;
}

async fn run_jar(main_class: String) {
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
    jvm.attach_current_thread_permanently().expect("Failed to bond with my own child!");
    JVM.set(jvm).unwrap();
    let mut env = jni();
    let arg_str = env.new_string("nogui").unwrap();
    let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
    let jobj_arg = unsafe {
        JObject::from_raw(**arg)
    };
    env.call_static_method(main_class, "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg).into()]).expect("ERR");
    log!("Loaded!");
    make_server_live_longer_than_half_a_second().await;
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

fn get_plugin_list() -> Vec<Plugin> {
    let mut jni_instance = jni();
    let mut plugins_vec: Vec<Plugin> = Vec::new();
    let server = jni_instance.call_static_method("org/bukkit/Bukkit", "getServer", "()Lorg/bukkit/Server;", &[]).unwrap();
    let b = JObject::try_from(server).unwrap();
    let a = JObject::try_from(jni_instance.call_method(b, "getPluginManager", "()Lorg/bukkit/plugin/PluginManager;", &[]).unwrap()).unwrap();
    let plugin_manager = jni_instance.call_method(a, "getPlugins", "()[Lorg/bukkit/plugin/Plugin;", &[]).unwrap();
    let plugins = JObjectArray::from(JObject::try_from(plugin_manager).unwrap());
    let plugin_count = jni_instance.get_array_length(&plugins).unwrap();
    log!("Number of plugins loaded: {:?}", &plugin_count);
    if plugin_count != 0 {
        for i in 0..plugin_count {
            let plugin = jni_instance.get_object_array_element(&plugins, i);
            match plugin {
                Ok(plugin) => {
                    let plugin_data = JObject::try_from(jni_instance.call_method(plugin, "getDescription", "()Lorg/bukkit/plugin/PluginDescriptionFile;", &[]).unwrap()).unwrap();
                    let plugin_name: String = jni_instance.get_string(&JString::from(JObject::try_from(jni().get_field(&plugin_data, "name", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
                    let plugin_version: String = jni_instance.get_string(&JString::from(JObject::try_from(jni().get_field(&plugin_data, "version", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
                    let plugin_description: String = jni_instance.get_string(&JString::from(JObject::try_from(jni().get_field(plugin_data, "description", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
                    plugins_vec.push(Plugin {
                        name: plugin_name,
                        version: plugin_version,
                        description: plugin_description,
                    })
                },
                Err(err) => {
                    error!("Error: {:?}", err)
                }
            }
        }
    } else {
        log!("There are no plugins loaded!");
    }
    plugins_vec
}

fn jni() -> JNIEnv<'static> {
    JVM.get().unwrap().get_env().unwrap()
}

async fn make_server_live_longer_than_half_a_second() {
    tokio::time::sleep(Duration::from_secs(5)).await;
    log!("{:?}", get_plugin_list());
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}