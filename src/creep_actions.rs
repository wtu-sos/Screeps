use log::*;
use screeps::{prelude::*, ResourceType, ReturnCode};
use std::cmp::min;

pub fn upgrade_controller(
    creep: screeps::objects::Creep,
    controller: &screeps::objects::StructureController,
) {
    let r = creep.upgrade_controller(controller);
    if r == ReturnCode::NotInRange {
        creep.move_to(controller);
    } else if r != ReturnCode::Ok {
        warn!("couldn't upgrade: {:?}", r);
    }
}

pub fn build(creep: screeps::objects::Creep, target_site: &screeps::objects::ConstructionSite) {
    let r = creep.build(target_site);
    if r == ReturnCode::NotInRange {
        //info!("creep: {}, move to :{}", creep.name(), target_site.structure_type());
        creep.move_to(target_site);
    } else if r != ReturnCode::Ok {
        warn!("couldn't build: {:?}", r);
    }
}

pub fn fill(creep: &screeps::objects::Creep, fill_target: &screeps::objects::Structure) {
    let transferable = fill_target.as_transferable().unwrap();
    let has_store = fill_target.as_has_store().unwrap();

    let empty_space = has_store.store_free_capacity(Some(ResourceType::Energy)) as u32;
    let creep_energy = creep.energy();
    let amount = min(creep_energy, empty_space);

    let r = creep.transfer_amount(transferable, ResourceType::Energy, amount);
    if r == ReturnCode::NotInRange {
        creep.move_to(fill_target);
    } else if r == ReturnCode::Full {
        creep.memory().del("fill_target");
    } else if r != ReturnCode::Ok {
        warn!("couldn't transfer: {:?}", r);
    }
}

pub fn harvest(creep: &screeps::objects::Creep) {
    if creep.memory().bool("harvesting") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("harvesting", false);
        }
    } else {
        if creep.store_used_capacity(None) == 0 {
            creep.memory().set("harvesting", true);
        }
    }

    if creep.memory().bool("harvesting") {
        //if let Some(source) = &creep
        //    .pos()
        //    .find_closest_by_range(screeps::constants::find::SOURCES)
        if let Some(source) = find_source(creep) {
            if creep.pos().is_near_to(&source) {
                let r = creep.harvest(&source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(&source);
            }
        } else {
            warn!("can't find source ! creep: {}", creep.name());
        }
    }
}

pub fn repair(creep: &screeps::objects::Creep, target: &screeps::objects::Structure) {
    //info!("creep: {}, repair :{} id: {}, pos: {}", creep.name(), target.structure_type(), target.id(), target.pos());
    let r = creep.repair(target);
    if r == ReturnCode::NotInRange {
        //info!("creep: {}, move to :{}", creep.name(), target_site.structure_type());
        creep.move_to(target);
    } else if r != ReturnCode::Ok {
        warn!("couldn't repair: {:?}", r);
    }
}

fn find_source(creep: &screeps::objects::Creep) -> Option<screeps::Source> {
    let sources = creep.room().unwrap().find(screeps::constants::find::SOURCES);
    if sources.len() == 0 {
        return None;
    }
    if let Ok(creep_type) = creep.memory().string("type") {
        if creep_type.unwrap_or("harvester".to_string()) != "harvester" {
            return Some(sources[0].clone());
        } else {
            return sources.last().map(|s|s.clone());
        }
    } else {
        return creep.pos().find_closest_by_range(screeps::constants::find::SOURCES);
    }
}
