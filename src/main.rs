use caltropper::send_key_presses;
use caltropper::command_parser;

fn main() {
    println!("Hello, world!");

    //command_parser("789,636,789,414.5".to_string());
    
    let mut keyboard = caltropper::Command::new("789,636,789,414.5".to_string());

    caltropper::wait_ms(5000);
    keyboard.place_multiple("789".to_string());
}
