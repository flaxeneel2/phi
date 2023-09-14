use std::path::PathBuf;
use better_graceful::SignalGuard;
use clap::Parser;
use jni::{InitArgsBuilder, JavaVM, JNIEnv, JNIVersion};
use jni::objects::{JObject, JValue};
use once_cell::sync::OnceCell;
use crate::modules::inactivity::Inactivity;
use crate::servers::bukkit::main::Bukkit;
use crate::servers::forge::main::Forge;
use crate::util::jar_util::get_main_class;
use crate::util::misc::run_startup_checks;
use crate::util::server_type::ServerType;

mod util;
mod servers;
mod modules;

#[derive(Parser,Debug)]
#[command(author, version, about)]
struct Args {
    /// The path of the server jar file
    #[arg(short,long,env="JAR_LOCATION",default_value="server.jar")]
    jar_location: PathBuf,
    /// If set, enables the inactivity module and sets the duration to the provided time in seconds.
    #[arg(short,long,env="INACTIVITY_DURATION")]
    inactivity_duration: Option<u32>,
}

static JVM: OnceCell<JavaVM> = OnceCell::new();
static CONFIG: OnceCell<Args> = OnceCell::new();
#[tokio::main]
async fn main() {
    let args_parsed = Args::parse();
    CONFIG.set(args_parsed).unwrap();
    run_startup_checks();
    run_jar().await;
}

async fn run_jar() {
    let signal_guard = SignalGuard::new();
    let server_type = ServerType::detect();
    let jar_loc = config().jar_location.to_str().unwrap();
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
                let mut arg = arg.replace("-p ", "--module-path=").replace(' ', "=")/*.replace("libraries/", "C:\\Users\\siddh\\Documents\\projects\\rust\\phi\\server\\forge\\libraries")*/;
                log!("arg: {}", arg);
                args_builder = args_builder.option(arg);
            }
        },
        /*
        ServerType::Forge => {
            for arg in Forge::get_args() {
                if ! arg.starts_with('-') {
                    break
                }
                if arg.starts_with("-p ") {
                    let modules = arg[2..].split(':').clone();
                    for module in modules {
                        args_builder = args_builder.option(format!("--module-path={}", module.trim()))
                    }
                    continue;
                }
                let arg = arg.replace(' ', "=");
                args_builder = args_builder.option(arg);
            }
            for args_built in args_builder.options().unwrap() {
                log!("Arg: {:?}", args_built)
            }
        },
        */
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
    log!("{:?}", args_builder);
    let jvm_args = args_builder.build().unwrap();
    let jvm = JavaVM::new(jvm_args).unwrap();
    JVM.set(jvm).unwrap();
    let mut env = jni();
    match server_type {
        ServerType::Bukkit => {
            let main_class = get_main_class();
            let arg_str = env.new_string("nogui").unwrap();
            let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
            let jobj_arg = unsafe {
                JObject::from_raw(**arg)
            };
            env.call_static_method(main_class, "main", "([Ljava/lang/String;)V", &[JValue::from(&jobj_arg)]).expect("ERR");
            log!("Loaded!");
            if config().inactivity_duration.is_some() {
                Inactivity::activate(config().inactivity_duration.unwrap());
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

fn config() -> &'static Args {
    CONFIG.get().unwrap()
}