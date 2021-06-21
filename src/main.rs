mod rest_client;
mod dice;

use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let mon = rest_client::fetch_monster("aboleth").unwrap();
    let name = if let serde_json::Value::String(name) = &mon["name"] {
        name
    } else {
        "error"
    };
    println!("Name: {}", name);
    let def_hp = if let serde_json::Value::Number(hp) = &mon["hit_points"] {
        hp.as_u64().unwrap()
    } else {
        0u64
    };
    println!("Default HP: {}", def_hp);

    let hit_dice = dice::DiceRoll::from_str(if let serde_json::Value::String(hd) = &mon["hit_dice"] {
        hd
    } else {
        panic!();
    }).unwrap();



    println!("Hit Dice: {}", hit_dice);
    println!("HP Roll Modifier: {}", def_hp - hit_dice.avg_roll());
    

    let mut roller = dice::DiceRoller::default();
    let roll = dice::DiceRoll::from_str("1d20 + 10").unwrap();
    println!("Test Roll: {}", roll);
    for _ in 0..10 {
        println!("{} => {:?}", roll, roller.roll(&roll));
    }

}
