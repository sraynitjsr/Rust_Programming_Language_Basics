enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction1 = Direction::Up;
    let player_direction2 = Direction::Down;
    let player_direction3 = Direction::Left;
    let player_direction4 = Direction::Right;

    print_direction(player_direction1);
    print_direction(player_direction2);
    print_direction(player_direction3);
    print_direction(player_direction4);
}

fn print_direction(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving Up!"),
        Direction::Down => println!("Moving Down!"),
        Direction::Left => println!("Moving Left!"),
        Direction::Right => println!("Moving Right!"),
    }
}
