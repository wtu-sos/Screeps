use crate::{creep_actions};
use log::*;
use screeps::{find, prelude::*, ResourceType, ConstructionSite, objects::Attackable};

pub fn run_settler(creep: screeps::objects::Creep) {
    let name = creep.name();
    debug!("running settler creep {}", name);

    if creep.spawning() {
        return;
    }

    if creep.store_used_capacity(None) > 0 && !creep.memory().bool("harvesting") {
        spend_energy(creep)
    } else {
        creep_actions::harvest(&creep);
    }
}

fn spend_energy(creep: screeps::objects::Creep) {
    let room = creep.room().unwrap();

    let construction_sites = find_construction(&creep);
    let structures = room.find(find::STRUCTURES);
    let mut towers: std::vec::Vec<screeps::objects::Structure> = vec![];
    let mut extensions: std::vec::Vec<screeps::objects::Structure> = vec![];
    let mut walls: std::vec::Vec<screeps::objects::Structure> = vec![];
    for my_structure in structures {
        match my_structure {
            screeps::Structure::Tower(ref my_tower) => {
                if my_tower.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                    towers.push(my_structure);
                }
            }
            screeps::Structure::Extension(ref my_extension) => {
                if my_extension.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                    extensions.push(my_structure);
                }
            }
            screeps::Structure::Wall(ref my_wall) => {
                let hits = my_wall.hits() as f32;
                let hits_max = my_wall.hits_max() as f32;
                if hits  < hits_max * 0.9f32 {
                    walls.push(my_structure);
                }
            }
            _ => (),
        };
    }
    debug!("settler spend info： tower: {}, extension: {}, construction_sites: {}", towers.len(), extensions.len(), construction_sites.len());
    if construction_sites.len() > 0 {
        creep_actions::build(creep, &construction_sites[0]);
    } else if towers.len() > 0 {
        creep_actions::fill(&creep, &towers[0]);
    } else if walls.len() > 0 {
        let mut target = &walls[0];
        for r in walls.iter() {
            if r.as_attackable().unwrap().hits() < target.as_attackable().unwrap().hits() {
                target = r;
            }
        }
        creep_actions::repair(&creep, target);
    } else if extensions.len() > 0 {
        creep_actions::fill(&creep, &extensions[0]);
    } else {
        creep_actions::upgrade_controller(creep, &room.controller().unwrap());
    };
}

fn find_construction(creep: &screeps::objects::Creep) -> Vec<ConstructionSite>{
    let room = creep.room().unwrap();
    let mut construction_sites = vec![];
    for construction_site in room.find(find::MY_CONSTRUCTION_SITES) {
        construction_sites.push(construction_site)
    }

    return construction_sites;
}
