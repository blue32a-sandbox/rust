fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {
        println!("add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("remove fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("move player {}", num_spaces);
    }
}
