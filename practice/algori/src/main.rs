use std::fs::File;
use std::io::Read;

//call algorithm
fn main() {
    //reading date from file
    let mut f = File::open("list.txt").expect("could not open file list.txt");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("read from file error");
    println!("reading from file: {}", s);

    //init data
    let data: Vec<i32> = s
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x: &&str| {
            x.trim()
                .parse()
                .expect(format!("parse '{}' to int error", x).as_str())
        })
        .collect();
    println!("init data: {:?}", data);

    //sort data
    let sorted_data = algori::sort(data);
    println!("after sort: {:?}", sorted_data);
}
