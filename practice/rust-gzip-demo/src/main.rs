use clap::{App, Arg};
use std::time::SystemTime;

fn main() {
    let matches = App::new("rust gzip demo")
        .arg(
            Arg::with_name("num")
                .short("n")
                .help("loop num")
                .takes_value(true),
        )
        .get_matches();

    let num = matches
        .value_of("num")
        .unwrap_or("100")
        .parse::<u32>()
        .unwrap_or(100);

    let start = SystemTime::now();

    rust_gzip_demo::test_gzip(num);

    let since_the_epoch = SystemTime::now()
        .duration_since(start)
        .expect("Time went backwards");

    println!("{}", since_the_epoch.as_millis());
}
