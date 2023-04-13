use jni::objects::{JObject, JObjectArray, JString};
use crate::jni;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub uuid: String,
    java_object: JObject<'static>
}

impl Player {
    pub fn get_online_players() {
        let players = JObject::try_from(jni().call_static_method("org/bukkit/Bukkit", "getOnlinePlayers", "()Ljava/util/Collection;", &[]).unwrap()).unwrap();
        let players_array = JObjectArray::try_from(JObject::try_from(jni().call_method(&players, "toArray", "()[Ljava/lang/Object;", &[]).unwrap()).unwrap()).unwrap();
        for i in 0..jni().call_method(&players, "size", "()I", &[]).unwrap().i().unwrap() {
            let player = jni().get_object_array_element(&players_array, i);
            match player {
                Ok(player) =>  {
                    let player_name =  JString::try_from(jni().call_method(&player, "getDisplayName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap()).unwrap();
                    let player_name: String = jni().get_string(&player_name).unwrap().into();
                },
                Err(e) => {}
            }
        }
    }
    pub fn get_online_player_count() -> i32 {
        let players = JObject::try_from(jni().call_static_method("org/bukkit/Bukkit", "getOnlinePlayers", "()Ljava/util/Collection;", &[]).unwrap()).unwrap();
        jni().call_method(&players, "size", "()I", &[]).unwrap().i().unwrap()
    }
    pub fn kick(&self) {
        jni().call_method(&self.java_object, "kickPlayer", "()V", &[]).unwrap();
    }
}