use caltropper::Caltropper;

fn main() {
    let mut caltropper = Caltropper::new();
    let sequence = "789,636,789,414.5".to_string();

    // caltropper::wait_ms(5000);

    caltropper.generate_command_sequence(sequence);

    caltropper.run_sequence();
}
