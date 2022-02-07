use crate::creep_actions;
use log::*;
use screeps::prelude::*;

pub fn run_upgrader(creep: screeps::objects::Creep) {
    let name = creep.name();
    debug!("running upgrader creep {}", name);

    if creep.spawning() {
        return;
    }

    if creep.store_used_capacity(None) > 0 && !creep.memory().bool("harvesting") {
        let room = creep.room().unwrap();
        creep_actions::upgrade_controller(creep, &room.controller().unwrap());
    } else {
        creep_actions::harvest(&creep);
    }
}
