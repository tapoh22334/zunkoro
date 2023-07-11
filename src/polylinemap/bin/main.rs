
use bevy::prelude::*;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process::exit;

use svg2polylines::{self, Polyline};

fn transform_coordinate(v: Vec2) -> Vec2 {
    let mut rv: Vec2 = Vec2::ZERO;
    rv.y = -v.y;
    rv.x = v.x;

    //return rv * 3.779528;
    return rv;
}

fn main() {
    // Argument parsing
    let args: Vec<_> = env::args().collect();
    match args.len() {
        3 => {}
        _ => {
            println!("Usage: {} <path/to/in_file.svg> <path/to/out_file.map>", args[0]);
            exit(1);
        }
    };

    // Load file
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Parse data
    let polylines: Vec<Polyline> = svg2polylines::parse(&s, 0.15, true).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);
    });

    // Print data
    println!("Found {} polylines.", polylines.len());
    let mut map = vec![];

    for lines in polylines {

        let mut v = vec![];
        for line in lines {
            v.push(transform_coordinate(Vec2::new(line.x as f32, line.y as f32)));
        }
        println!("- {:?}", v);

        map.push(v);
    }

    let serialized = serde_json::to_string(&map).unwrap();
    let mut file = fs::File::create(&args[2]).unwrap();
    file.write(serialized.as_bytes()).unwrap();
}
