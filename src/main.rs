#[macro_use]
extern crate serde;
extern crate csv;
extern crate clap;
extern crate rayon;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use clap::Arg;
use clap::App;
use rayon::prelude::*;
use csv::StringRecord;

// u8 is bool
// Commented fields exist in the csv but aren't being used by the program

#[derive(Debug, Deserialize)]
struct System {
    pub id: u64,
    // pub edsm_id: Option<u64>,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // pub population: Option<u64>,
    pub is_populated: u8,
    // pub government_id: Option<u64>,
    // pub government: Option<String>,
    // pub allegiance_id: Option<u64>,
    // pub allegiance: Option<String>,
    // pub security_id: Option<u64>,
    // pub security: Option<String>,
    // pub primary_economy_id: Option<u64>,
    // pub primary_economy: Option<String>,
    // pub power: Option<String>,
    // pub power_state: Option<u8>,
    // pub power_state_id: Option<u64>,
    // pub needs_permit: u8,
    // pub updated_at: u64,
    // pub simbad_ref: String,
    // pub controlling_minor_faction_id: Option<u64>,
    // pub controlling_minor_faction: Option<String>,
    // pub reserve_type_id: Option<u64>,
    // pub reserve_type: Option<String>
}

impl System {
    pub fn distance_to(&self, s: &System) -> f32 {
        let x_part = (s.x - self.x).powf(2.0);
        let y_part = (s.y - self.y).powf(2.0);
        let z_part = (s.z - self.z).powf(2.0);

        (x_part + y_part + z_part).sqrt()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("ED Federation Ranking Systems")
        .version("1.0")
        .author("Noah Shuart <shuart.noah.s@gmail.com>")
        .about("Searches an EDDB systems dump for systems good for ranking fed")
        .arg(Arg::with_name("multithreaded")
            .help("Theoretical speed boost at the cost of a lot of memory")
            .short("m"))
        .arg(Arg::with_name("file")
            .help("The systems dump csv file path, defaults to ./systems.csv")
            .index(1))
        .get_matches();

    // Home system
    let home = System {
        id: 19049,
        name: "WISE 1800+0134".to_string(),
        is_populated: 1,
        x: -13.40625,
        y: 5.96875,
        z: 24.65625
    };
    let file_path_str = matches.value_of("file").unwrap_or("./systems.csv");
    let file = File::open(file_path_str)?;

    print!("Loading systems...");
    let _ = io::stdout().flush();
    let mut reader = csv::Reader::from_reader(file);

    if matches.is_present("multithreaded") {
        let headers = reader.headers()?.clone();

        // Move deserialization into the rayon parallel iterator
        let records = reader.records()
            .filter_map(|record| record.ok())
            .collect::<Vec<StringRecord>>();
        println!("Done.");

        let nearby: Vec<System> = records.par_iter()
            .map(|record| record.deserialize(Some(&headers)))
            .filter_map(|result: Result<System, csv::Error>| result.ok())
            .filter(|system| system.distance_to(&home) <= 8.0)
            .collect();

        for system in nearby {
            println!("{}", system.name);
        }
    } else {
        let systems = reader.deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<System>>();
        println!("Done.");

        let nearby: Vec<&System> = systems.iter()
            .filter(|system| system.distance_to(&home) <= 8.0)
            .collect();

        for system in nearby {
            println!("{}", system.name);
        }
    }

    Ok(())
}

