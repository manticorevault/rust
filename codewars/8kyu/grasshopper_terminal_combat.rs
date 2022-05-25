fn combat(health: f32, damage: f32) -> f32 {
    if health - damage < 0.0 {
        println!("Health cannot go below 0");
        return 0.0
    } else {
        return health - damage;
    }
}

// Good practice
// fn combat(health: f32, damage: f32) -> f32 {
//     (health - damage).max(0.0)
// }