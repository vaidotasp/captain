use std::path::Path;

use chrono::Datelike;
use image::{imageops, GenericImageView};

// Captain image is 500x425

// "static" means static lifetime which is to say that the value will live for the duration of the program. it is the longest possible lifetime.
fn get_path() -> &'static str {
    // weekly images
    const MONDAY_PATH: &str = "src/monday.png";
    const TUESDAY_PATH: &str = "src/tuesday.png";
    const WEDNESDAY_PATH: &str = "src/wednesday.png";
    const THURSDAY_PATH: &str = "src/thursday.png";
    const FRIDAY_PATH: &str = "src/friday.png";
    const SATURDAY_PATH: &str = "src/saturday.png";
    const SUNDAY_PATH: &str = "src/sunday.png";

    let current_time = chrono::offset::Local::now();
    let weekday = current_time.date_naive().weekday();
    println!("{}", weekday);
    match weekday {
        chrono::Weekday::Mon => return MONDAY_PATH,
        chrono::Weekday::Tue => return TUESDAY_PATH,
        chrono::Weekday::Wed => return WEDNESDAY_PATH,
        chrono::Weekday::Thu => return THURSDAY_PATH,
        chrono::Weekday::Fri => return FRIDAY_PATH,
        chrono::Weekday::Sat => return SATURDAY_PATH,
        chrono::Weekday::Sun => return SUNDAY_PATH,
    }
}

fn main() {
    // load png in memory and see how we can read it?
    println!("Hello, world!!!");

    // `open` returns a `DynamicImage` on success.
    let mut cap = image::open("src/captain.png").unwrap();
    let img = image::open(get_path()).unwrap();
    let img2 = img.resize(120, 30, image::imageops::FilterType::Gaussian);
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    println!("dimensions {:?}", img2.dimensions());

    imageops::overlay(&mut cap, &img2, 195, 135);

    let save_path = Path::new("src/new.png");
    cap.save(save_path).unwrap();

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}
