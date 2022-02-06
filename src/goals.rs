pub fn set_goals() {
    let mem = screeps::memory::root();
    mem.set("home_room", "E3S24");
    mem.set("work_room", vec!["E3S24"]);

    mem.path_set("spawn_goals.E3S24.harvester", 6);
    mem.path_set("spawn_goals.E3S24.filler", 0);
    mem.path_set("spawn_goals.E3S24.upgrader", 0);
    mem.path_set("spawn_goals.E3S24.settler", 2);
    mem.path_set("spawn_goals.E3S24.reserver", 0);
}