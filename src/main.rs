use std::env;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;
use better_graceful::SignalGuard;
use jni::{InitArgsBuilder, JavaVM, JNIEnv, JNIVersion};
use jni::objects::{JObject, JValue};
use once_cell::sync::OnceCell;
use crate::modules::inactivity::Inactivity;
use crate::servers::bukkit::main::Bukkit;
use crate::servers::forge::main::Forge;
use crate::util::jar_util::get_main_class;
use crate::util::server_type::ServerType;

mod util;
mod servers;
mod modules;


struct Args {
    
}

static JVM: OnceCell<JavaVM> = OnceCell::new();

#[tokio::main]
async fn main() {
    //let main_class = get_main_class();
    //run_jar(main_class).await;
    run_jar().await;
}

async fn run_jar() {
    let signal_guard = SignalGuard::new();
    let server_type = ServerType::detect();
    let jar_loc = env::var("JAR_LOCATION").unwrap_or_else(|_e| {
        error!("JAR_LOCATION env variable needs to be reset!");
        exit(1);
    });
    let mut args_builder = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Xmx4G")
        .option("-Xms128M");
    match server_type {
        ServerType::Forge => {
            for arg in Forge::get_args() {
                if ! arg.starts_with('-') {
                    break
                }
                // if ! arg.contains(' ') {
                //     args_builder = args_builder.option(arg);
                // }
                let arg = arg.replace(' ', "=").replace("-p", "--module-path").replace("libraries/", "/home/flaxeneel2/Documents/projects/rust/phi/server/forge/libraries/");
                log!("arg: {}", arg);
                args_builder = args_builder.option(arg);
            }
        },
        ServerType::Fabric => {
            args_builder = args_builder
                .option("-Dterminal.jline=false")
                .option("-Dterminal.ansi=true")
                .option(format!("-Djava.class.path={}", jar_loc))
        },
        ServerType::Bukkit => {
            args_builder = args_builder
                .option("-Dterminal.jline=false")
                .option("-Dterminal.ansi=true")
                .option(format!("-Djava.class.path={}", jar_loc))
                .option(format!("-javaagent:{}", jar_loc))
        }
    }
    log!("args: {:?}", args_builder);
    let jvm_args = args_builder.build().unwrap();
    log!("hi");
    let jvm = JavaVM::new(jvm_args).unwrap();
    JVM.set(jvm).unwrap();
    let mut env = jni();
    match server_type {
        ServerType::Bukkit => {
            let arg_str = env.new_string("nogui").unwrap();
            let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
            let jobj_arg = unsafe {
                JObject::from_raw(**arg)
            };
            let main_class = get_main_class();
            env.call_static_method(main_class, "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg)]).expect("ERR");
            log!("Loaded!");
            if env::var("INACTIVITY_MODE").unwrap_or("false".to_string()).eq_ignore_ascii_case("true") {
                Inactivity::activate(i32::from_str(env::var("INACTIVITY_DURATION").unwrap_or("310".to_string()).as_str()).unwrap_or_else(|_| {
                    warn!("Invalid inactivity timeout duration! Defaulting to 310 seconds");
                    310
                }));
            }
        },
        ServerType::Forge => {
            let mut args = String::new();
            let mut reversed_args = Forge::get_args();
            let mut class_path = String::new();
            reversed_args.reverse();
            for arg in reversed_args  {
                if ! arg.starts_with('-') {
                    class_path = arg;
                    break
                }
                args.push_str(&format!(" {}", arg));
            }
            let arg_str = env.new_string(args).unwrap();
            let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
            let jobj_arg = unsafe {
                JObject::from_raw(**arg)
            };
            env.call_static_method(class_path, "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg)]).expect("ERR");
            log!("Loaded!");
        },
        ServerType::Fabric => {
            tokio::spawn(run_fabric()).await.unwrap();
        }
    }
    loop {
        tokio::time::sleep(Duration::from_secs(7)).await;
        println!("Online player count: {}", servers::bukkit::entity::player::Player::get_online_player_count());
    }
    signal_guard.at_exit(move |_sig| {
        log!("Shutting down...");
        match server_type {
            ServerType::Bukkit => {
                Bukkit::stop_server();
            },
            ServerType::Fabric => {

            },
            ServerType::Forge => {

            }
        }
    });
}

async fn run_fabric() {
    let mut env = jni();
    let arg_str = env.new_string("nogui").unwrap();
    let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
    let jobj_arg = unsafe {
        JObject::from_raw(**arg)
    };
    log!("Loaded!");
    env.call_static_method(get_main_class(), "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg)]).expect("ERR");
}


fn jni() -> JNIEnv<'static> {
    JVM.get().unwrap().attach_current_thread_permanently().unwrap()
}