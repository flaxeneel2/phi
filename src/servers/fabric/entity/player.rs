use jni::objects::JObject;
use crate::jni;

pub struct Player;

impl Player {
    pub fn get_online_player_count() -> i32 {
        //let server_metadata = JObject::try_from(jni().call_static_method("net/flaxeneel2/test", "method_3765", "()Lnet/minecraft/class_2926;", &[]).unwrap()).unwrap();
        //let server_instance = JObject::try_from(jni().call_static_method("net/flaxeneel2/testarea/Testarea", "getMinecraftServer", "()Lnet/minecraft/server/MinecraftServer;", &[]).unwrap()).unwrap();
        //println!("{:?}", server_instance);
        //let player_data = JObject::try_from(jni().call_st)
        //jni().call_method(server_metadata_players, "getOnlinePlayerCount", "()I", &[]).unwrap().i().unwrap()
        0
    }
}