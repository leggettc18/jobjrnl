fn main() {
    let app =
        jobjrnl::JobApplication::new(String::from("Test Job"), String::from("2022-11-06"), true);
    println!("{:?}", app)
}
