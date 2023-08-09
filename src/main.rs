use std::path::Path;

use chrono::Datelike;
use image::{imageops, DynamicImage};

fn get_file_image() -> DynamicImage {
    // we basically jam all pngs into memory so that we can access at runtime.
    let monday_bytes = include_bytes!("monday.png");
    let tuesday_bytes = include_bytes!("tuesday.png");
    let wednesday_bytes = include_bytes!("wednesday.png");
    let thursday_bytes = include_bytes!("thursday.png");
    let friday_bytes = include_bytes!("friday.png");
    let saturday_bytes = include_bytes!("saturday.png");
    let sunday_bytes = include_bytes!("sunday.png");
    let monday = image::load_from_memory(monday_bytes).unwrap();
    let tuesday = image::load_from_memory(tuesday_bytes).unwrap();
    let wednesday = image::load_from_memory(wednesday_bytes).unwrap();
    let thursday = image::load_from_memory(thursday_bytes).unwrap();
    let friday = image::load_from_memory(friday_bytes).unwrap();
    let saturday = image::load_from_memory(saturday_bytes).unwrap();
    let sunday = image::load_from_memory(sunday_bytes).unwrap();

    let current_time = chrono::offset::Local::now();
    let weekday = current_time.date_naive().weekday();

    match weekday {
        chrono::Weekday::Mon => return monday,
        chrono::Weekday::Tue => return tuesday,
        chrono::Weekday::Wed => return wednesday,
        chrono::Weekday::Thu => return thursday,
        chrono::Weekday::Fri => return friday,
        chrono::Weekday::Sat => return saturday,
        chrono::Weekday::Sun => return sunday,
    }
}

fn main() {
    let cap_bytes = include_bytes!("captain.png");
    let mut cap = image::load_from_memory(cap_bytes).unwrap();

    let img = get_file_image();
    // The dimensions method returns the images width and height.
    let img2 = img.resize(120, 30, image::imageops::FilterType::Gaussian);

    //approximate location of speech bubble where we overlay the day of the week
    imageops::overlay(&mut cap, &img2, 195, 135);
    let save_path = Path::new("new.png");
    cap.save(save_path).unwrap();
    println!("done");
    return;
}
