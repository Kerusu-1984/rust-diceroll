mod diceroll;

fn main() {
    println!("STR: {}", diceroll::roll_3d6());
    println!("CON: {}", diceroll::roll_3d6());
    println!("POW: {}", diceroll::roll_3d6());
    println!("DEX: {}", diceroll::roll_3d6());
    println!("APP: {}", diceroll::roll_3d6());
    println!("SIZ: {}", diceroll::roll_2d6_plus_6());
    println!("INT: {}", diceroll::roll_2d6_plus_6());
    println!("EDU: {}", diceroll::roll_3d6_plus_3());
}
