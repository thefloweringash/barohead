use std::io;

use barohead_data::items::*;

fn main() {
    let data: ItemDB = serde_json::from_reader(io::stdin()).unwrap();
    bincode::serialize_into(io::stdout(), &data).unwrap();
}
