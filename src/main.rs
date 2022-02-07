use std::collections::HashSet;

use log::*;
use screeps::{find, prelude::*};
use stdweb::js;

mod logging;
mod goals;
mod spawner;
mod settler;
mod creep_actions;
mod tower;
mod harvester;
mod upgrader;

fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    debug!("running spawns");
    goals::set_goals();
    for spawn in screeps::game::spawns::values() {
        spawner::run_spawn(spawn);
    }

    debug!("running towers");
    let mut towers: std::vec::Vec<screeps::objects::StructureTower> = vec![];
    for room in screeps::game::rooms::values() {
        let structures = room.find(find::STRUCTURES);
        for my_structure in structures {
            match my_structure {
                screeps::Structure::Tower(my_tower) => {
                    towers.push(my_tower);
                }
                _ => (),
            };
        }
    }
    for my_tower in towers {
        tower::run_tower(my_tower);
    }

    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }
        let creep_type = creep.memory().string("type").unwrap_or(None);
        match creep_type {
            Some(t) => {
                match t.as_str() {
                    "settler" => {
                        settler::run_settler(creep);
                    },
                    "harvester" => {
                        harvester::run_harvest(creep);
                    }
                    "upgrader" => {
                        upgrader::run_upgrader(creep);    
                    }
                    _ => {
                        warn!("creep {} not in control", t);
                    }
                }
            }
            None => {
            }
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}

fn cleanup_memory() -> Result<(), Box<dyn std::error::Error>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            warn!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}
