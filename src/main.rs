use caltropper::Caltropper;

fn main() {
    let mut caltropper = Caltropper::new("789,636,789,414.5".to_string());

    caltropper::wait_ms(5000);

    caltropper.place_multiple("789".to_string());
}
