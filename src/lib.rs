use enigo::*;
use std::{thread, time};

pub fn send_key_presses(key_presses: &str) {
    let mut keyboard = Enigo::new();

    wait_ms(2500);

    keyboard.set_delay(250);

    keyboard.key_sequence_parse(key_presses);
}

pub enum Directions {
    DownLeft = 1,
    Down = 2,
    DownRight = 3,
    Left = 4,
    Right = 6,
    UpLeft = 7,
    Up = 8,
    UpRight = 9,
    Center = 5,
}

#[allow(dead_code)]
pub struct Caltropper {
    keyboard: Enigo,
    actions: Vec<String>,
    iterations: u32,
}

pub fn command_parser(input: String) -> (Vec<String>, u32) {
    let split_commands: Vec<&str> = input.split(".").collect();
    let actions: Vec<&str> = split_commands[0].split(",").collect();
    let iterations: u32 = split_commands[1].parse::<u32>().unwrap();
    let mut better_actions = vec![];

    for action in actions.iter() {
        better_actions.push(action.to_string());
    }


    println!("Actions: {:?}", actions);
    println!("Iterations: {}", iterations);

    (better_actions, iterations)
}

impl Caltropper {
    pub fn new(input: String) -> Caltropper {
        let keyboard = Enigo::new();
        let ( actions, iterations ) = command_parser(input);

        Caltropper { keyboard, actions, iterations }
    }

    fn place_single(&mut self, direction: &str) {
        self.keyboard.key_sequence_parse("a/caltrops");
        self.keyboard.key_click(Key::Return);
        self.keyboard.key_sequence_parse(direction);
    }

    pub fn place_multiple(&mut self, directions: String) {
        for direction in directions.chars() {
            let direction = &direction.to_string();
            self.place_single(direction);
            wait_ms(25);
        }
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
