// adding my dataset
use std::fs::File;

fn main() {
    let data_result = File::open("Crime_Data_from_LA.csv");
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);
}

#[derive(Debug)]
struct Crime_Data_from_LA {
    LAT: f64,
    LON: f64
}


