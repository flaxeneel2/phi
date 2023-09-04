use std::time::Duration;
use tokio::time::sleep;
use crate::log;
use crate::servers::bukkit::main::Bukkit;
use crate::servers::bukkit::entity::player::Player;

pub struct Inactivity;

impl Inactivity {
    pub fn activate(timeout: i32) {
        log!("Starting checker");
        tokio::spawn(Self::start_inactivity_checker(timeout));
    }
    async fn start_inactivity_checker(timeout: i32) {
        let mut counter = timeout;
        loop {
            sleep(Duration::from_secs(1)).await;
            if Player::get_online_player_count() != 0 {
                counter = timeout;
            } else {
                counter -= 1;
                match counter {
                    300 => { log!("There are no players online! The server will shut down in 5 minutes.") },
                    180 => { log!("There are no players online! The server will shut down in 3 minutes.") },
                    120 => { log!("There are no players online! The server will shut down in 2 minutes.") },
                    60 => { log!("There are no players online! The server will shut down in 1 minute.") },
                    30 => { log!("There are no players online! The server will shut down in 30 seconds.") },
                    15 => { log!("There are no players online! The server will shut down in 15 seconds.") },
                    5 => { log!("There are no players online! The server will shut down in 5 seconds.") },
                    3 => { log!("There are no players online! The server will shut down in 3 seconds.") },
                    2 => { log!("There are no players online! The server will shut down in 2 seconds.") },
                    1 => { log!("There are no players online! The server will shut down in 1 second.") },
                    0 => {
                        log!("There has been no activity for a while! Shutting the server down...");
                        Bukkit::stop_server();
                    }
                    _ => {}
                }
            }
        }
    }
}