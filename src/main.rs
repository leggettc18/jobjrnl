fn main() {
    let app = jobjrnl::JobApplication::new(
        String::from("Test Job"),
        String::from("2022-11-10"),
        true,
        false,
        None,
        None,
    );
    println!("{}", app)
}
