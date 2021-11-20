struct Conf;

impl tuid::config::Configuration for Conf {
    const __PERIOD__: u64 = 2;
}

// cargo run -p tuid-examples --example approx --features tuid-rs/approx
fn main() {
    let updater = tuid::time::updater(500);
    updater.start().expect("Failed to spawn updater thread");

    for _ in 0..3 {
        let id = tuid::gen::new::<Conf>(0, 0);
        println!("id: {}", id.as_uuid());
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
