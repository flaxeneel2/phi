use jni::{InitArgsBuilder, JavaVM, JNIVersion};
use jni::objects::JObject;

mod util;

fn main() {
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Xmx4G")
        .option("-Xms4G")
        .option("-Dterminal.jline=false")
        .option("-Dterminal.ansi=true")
        .option("-Djava.class.path=/home/flaxeneel2/Documents/projects/rust/phi/server/server.jar")
        .build()
        .unwrap();
    let jvm = JavaVM::new(jvm_args).unwrap();
    let env = jvm.attach_current_thread().unwrap();
    let arg_str = env.new_string("nogui").unwrap();
    let arg = env.new_object_array(1, "java/lang/String", arg_str).unwrap();
    unsafe { env.call_static_method("io/papermc/paperclip/Paperclip", "main", "([Ljava/lang/String;)V", &[JObject::from_raw(arg).into()]).expect("ERR"); }
    log!("Loaded!");
}