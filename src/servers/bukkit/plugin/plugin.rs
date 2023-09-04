use jni::objects::{JObject, JObjectArray, JString};
use crate::{error, jni, log};
use crate::servers::bukkit::main::Bukkit;

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String
}

impl Plugin {
    pub fn get_plugin_list() -> Vec<Plugin> {
        let mut plugins_vec = Vec::new();
        let plugin_manager = Bukkit::get_plugin_manager();
        let plugins_jobject = jni().call_method(plugin_manager, "getPlugins", "()[Lorg/bukkit/plugin/Plugin;", &[]).unwrap();
        let plugins = JObjectArray::from(JObject::try_from(plugins_jobject).unwrap());
        let plugin_count = jni().get_array_length(&plugins).unwrap();
        log!("Number of plugins loaded: {:?}", &plugin_count);
        for i in 0..plugin_count {
            let plugin = jni().get_object_array_element(&plugins, i);
            match plugin {
                Ok(plugin) => {
                    let plugin_data = JObject::try_from(jni().call_method(plugin, "getDescription", "()Lorg/bukkit/plugin/PluginDescriptionFile;", &[]).unwrap()).unwrap();
                    let plugin_name: String = jni().get_string(&JString::from(JObject::try_from(jni().get_field(&plugin_data, "name", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
                    let plugin_version: String = jni().get_string(&JString::from(JObject::try_from(jni().get_field(&plugin_data, "version", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
                    let plugin_description: String = jni().get_string(&JString::from(JObject::try_from(jni().get_field(plugin_data, "description", "Ljava/lang/String;").unwrap()).unwrap())).unwrap().into();
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
        plugins_vec
    }
}