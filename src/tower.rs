use log::*;
use screeps::{find, prelude::*};

pub fn run_tower(tower: screeps::objects::StructureTower) {
    debug!("running tower {:?}", tower.id());

    let room = tower.room().unwrap();
    let targets = room.find(find::HOSTILE_CREEPS);
    if targets.len() > 0 {
        tower.attack(&targets[0]);
    }

    //info!("tower energy: {}, max energy: {}", tower.energy(), tower.store_capacity(Some(screeps::ResourceType::Energy)));
    if tower.energy() * 2 > tower.store_capacity(Some(screeps::ResourceType::Energy)) {
        let my_structures = room.find(find::STRUCTURES);
        let mut repair_targets: std::vec::Vec<screeps::objects::Structure> = vec![];
        for structure in my_structures {
            if structure.as_attackable().is_some() {
                let hits = structure.as_attackable().unwrap().hits();
                let hits_max = structure.as_attackable().unwrap().hits_max();
                if structure.structure_type() == screeps::constants::StructureType::Wall {
                    if hits < 500000u32 {
                        repair_targets.push(structure);
                    }
                } else {
                    if hits + 800 < hits_max {
                        repair_targets.push(structure);
                    }
                }
            }
        }
        if repair_targets.len() > 0 {
            let mut target = &repair_targets[0];
            for r in repair_targets.iter() {
                if r.as_attackable().unwrap().hits() < target.as_attackable().unwrap().hits() {
                    target = r;
                }
            }
            //debug!("target hits: {}", target.as_attackable().unwrap().hits());
            tower.repair(target);
        }
    }
}
