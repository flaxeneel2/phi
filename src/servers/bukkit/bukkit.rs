use jni::objects::JObject;
use crate::jni;

pub struct Bukkit;

impl Bukkit {
    pub fn stop_server() {
        jni().call_static_method("org/bukkit/Bukkit", "shutdown", "()V", &[]).unwrap();
    }
    pub fn get_plugin_manager() -> JObject<'static> {
        let mut jni_instance = jni();
        let server = jni_instance.call_static_method("org/bukkit/Bukkit", "getServer", "()Lorg/bukkit/Server;", &[]).unwrap();
        let bukkit = JObject::try_from(server).unwrap();
        JObject::try_from(jni_instance.call_method(bukkit, "getPluginManager", "()Lorg/bukkit/plugin/PluginManager;", &[]).unwrap()).unwrap()
    }
}