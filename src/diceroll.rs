use rand::Rng;

pub fn roll_3d6() -> i32 {
    dice() + dice() + dice()
}

pub fn roll_2d6_plus_6() -> i32 {
    dice() + dice() + 6
}

pub fn roll_3d6_plus_3() -> i32 {
    dice() + dice() + dice() + 3
}

fn dice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
