use enigo::*;
use std::{thread, time};

#[allow(dead_code)]
pub struct Caltropper {
    keyboard: Enigo,
    actions: Vec<String>,
    iterations: u32,
}


impl Caltropper {
    pub fn new() -> Caltropper {
        let keyboard = Enigo::new();

        let actions = vec![];
        let iterations = 0;

        Caltropper { keyboard, actions, iterations }
    }

    pub fn generate_command_sequence(&mut self, input: String) {
        let split_commands: Vec<&str> = input.split(".").collect();
        let actions: Vec<&str> = split_commands[0].split(",").collect();
        let iterations: u32 = split_commands[1].parse::<u32>().unwrap();

        let mut better_actions = vec![];
        for action in actions.iter() {
            better_actions.push(action.to_string());
        }

        self.actions = better_actions;
        self.iterations = iterations;

        println!("Actions: {:?}", actions);
        println!("Iterations: {}", iterations);
    }

    fn place(&mut self, direction: &str) {
        self.keyboard.key_sequence_parse("a/caltrops");
        self.keyboard.key_click(Key::Return);
        self.keyboard.key_sequence_parse(direction);
    }

    pub fn place_multiple(&mut self, directions: String) {
        for direction in directions.chars() {
            let direction = &direction.to_string();
            self.place(direction);
            wait_ms(25);
        }
    }

    pub fn run_sequence(&mut self) {
        for command_chunk in 0..self.actions.len() {
            match command_chunk % 2 {
                0 => println!("even pattern"),
                1 => println!("odd pattern"),
                _ => panic!("Iterator not an integer!"),
            }
        }
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
