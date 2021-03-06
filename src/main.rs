use rand::Rng;
use mouse_rs::{types::keys::Keys, Mouse};
use std::{thread, time};


static UPPER_X_BOUND: u64 = 1800;
static UPPER_Y_BOUND: u64 = 1000;
static UPPER_SLEEP_CLICK_BOUND: u64 = 5;

static LOWER_X_BOUND: u64 = 100;
static LOWER_Y_BOUND: u64 = 100;
static LOWER_SLEEP_CLICK_BOUND: u64 = 1;


// This method is responsible for moving a given mouse object to a random position
fn move_mouse_to(mouse: &mut Mouse, x: u64, y: u64) {
    match mouse.move_to(x.try_into().unwrap(), y.try_into().unwrap()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Moving mouse to given position has failed!");
            std::process::exit(1);
        } 
    };
}


// This method clicks with a given mouse object
fn click_with_mouse(mouse: &mut Mouse) {
    match mouse.click(&Keys::RIGHT) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to click with the mouse.");
            std::process::exit(1);
        }
    };
}


// This method sleeps for a random amount of time between two given values
fn sleep_mf(low: u64, high: u64) {
    let sleep_value: u64 = rand::thread_rng().gen_range(low..high);
    let sleep_secs = time::Duration::from_secs(sleep_value);
    thread::sleep(sleep_secs);
}


fn main() {

    let mut mouse = Mouse::new();

    loop {

        let number_of_clicks: i64 = rand::thread_rng().gen_range(2..15);
        for _ in 0..number_of_clicks {
            let x_number: u64 = rand::thread_rng().gen_range(LOWER_X_BOUND..UPPER_X_BOUND);
            let y_number: u64 = rand::thread_rng().gen_range(LOWER_Y_BOUND..UPPER_Y_BOUND);
            
            move_mouse_to(&mut mouse, x_number, y_number);
            click_with_mouse(&mut mouse);
            
            sleep_mf(LOWER_SLEEP_CLICK_BOUND, UPPER_SLEEP_CLICK_BOUND);
        }

    }

}
