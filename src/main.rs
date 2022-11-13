use chrono::Local;
use chrono::TimeZone;

fn main() {
    let app = jobjrnl::JobApplication::new(
        String::from("Test Job"),
        Local
            .datetime_from_str("2022-11-10 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        true,
        false,
        None,
        None,
    );
    println!("{}", app)
}
