use caltropper::Caltropper;

fn main() {
    let mut caltropper = Caltropper::new();
    caltropper.wait_ms(5000);
    let sequence = "789,636,789,414.2";

    caltropper.generate_command_sequence(sequence);

    caltropper.run_sequence();
}
