use enigo::*;
use std::{thread, time};

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

    fn move_character(&mut self, direction: &str) {
        self.keyboard.key_sequence_parse(direction);
    }

    pub fn move_multiple(&mut self, directions: String) {
        for direction in directions.chars() {
            let direction = &direction.to_string();
            self.move_character(direction);
            wait_ms(25);
        }
    }

    fn run_sequence_chunk(&mut self) {
        let new_vec = self.actions.to_vec();
        for (i, chunk) in new_vec.iter().enumerate() {
            match i % 2 {
                1 => {
                    println!("moving: {:?}",chunk);
                    let chunk = new_vec[i].clone();
                    self.move_multiple(chunk);
                },
                0 => {
                    println!("placing: {:?}",chunk);
                    let chunk = new_vec[i].clone();
                    self.place_multiple(chunk);
                },
                _ => panic!("Iterator not an integer!"),
            }
        }
    }

    pub fn run_sequence(&mut self) {
        for _iteration in 0..self.iterations {
            self.run_sequence_chunk();
        }
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
