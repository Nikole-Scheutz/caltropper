use enigo::*;
use std::{thread, time};

struct Delays {
    search_ms: u64,
    move_ms: u64,
    generic_ms: u64,
    inter_key_ns: u64,
}

pub struct Caltropper {
    keyboard: Enigo,
    actions: Vec<String>,
    iterations: u32,
    delays: Delays,
}

impl Caltropper {
    pub fn new() -> Caltropper {
        let mut keyboard = Enigo::new();

        let actions = vec![];
        let iterations = 0;

        let delays = Delays {
            search_ms: 75,
            move_ms: 75,
            generic_ms: 25,
            inter_key_ns: 12500,
        };

        keyboard.set_delay(delays.inter_key_ns);

        Caltropper { keyboard, actions, iterations, delays }
    }

    pub fn generate_command_sequence(&mut self, input: &str) {
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

    fn search(&mut self, search_string: &str) {
        self.keyboard.key_sequence_parse("a/");
        self.wait_ms(self.delays.generic_ms);

        self.keyboard.key_sequence_parse(search_string);
        self.wait_ms(self.delays.search_ms);

        self.keyboard.key_click(Key::Return);
        self.wait_ms(self.delays.generic_ms);

        self.keyboard.key_click(Key::Return);
        self.wait_ms(self.delays.generic_ms);
    }

    fn place(&mut self, direction: &str) {
        self.search("caltrop");
        self.keyboard.key_sequence_parse(direction);
    }

    pub fn place_multiple(&mut self, directions: String) {
        for direction in directions.chars() {
            let direction = &direction.to_string();
            self.place(direction);
            self.wait_ms(self.delays.generic_ms);
        }
    }

    fn move_character(&mut self, direction: &str) {
        self.keyboard.key_sequence_parse(direction);
        self.wait_ms(self.delays.move_ms);
    }

    pub fn move_multiple(&mut self, directions: String) {
        for direction in directions.chars() {
            let direction = &direction.to_string();
            self.move_character(direction);
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

    pub fn set_delay(&mut self, desired_delay: u64) {
        self.keyboard.set_delay(desired_delay);
    }

    pub fn run_sequence(&mut self) {
        for _iteration in 0..self.iterations {
            self.run_sequence_chunk();
        }
    }

    pub fn wait_ms(&self, ms: u64) {
        thread::sleep(time::Duration::from_millis(ms));
    }
}

