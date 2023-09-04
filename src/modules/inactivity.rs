use std::time::Duration;
use tokio::time::sleep;
use crate::log;
use crate::servers::bukkit::main::Bukkit;
use crate::servers::bukkit::entity::player::Player;
use crate::util::misc::time_to_string;

pub struct Inactivity;

impl Inactivity {
    pub fn activate(timeout: i32) {
        log!("Starting checker");
        tokio::spawn(Self::start_inactivity_checker(timeout));
    }
    async fn start_inactivity_checker(timeout: i32) {
        let timestamps_to_warn: Vec<i32> = vec![300, 240, 180, 120, 60, 30, 15, 5, 4, 3, 2, 1];
        let mut counter = timeout;
        loop {
            sleep(Duration::from_secs(1)).await;
            if Player::get_online_player_count() != 0 {
                counter = timeout;
            } else {
                counter -= 1;
                match counter {
                    0 => {
                        log!("There has been no activity for a while! Shutting the server down...");
                        Bukkit::stop_server();
                    }
                    _ => {
                        if timestamps_to_warn.contains(&counter) {
                            log!("There are no players online! The server will shut down in {}", time_to_string(counter))
                        }
                    }
                }
            }
        }
    }
}