use std::fs::File;
use std::io::{BufRead, BufReader};
use sgp4::parse_3les;
mod satutil;
pub mod observer;
use crate::observer::Observer;
pub mod satellite;
use crate::satellite::Satellite;
pub mod coord_systems;

fn main() -> anyhow::Result<()>{
    let file = File::open("src/tle2.txt").unwrap();
    let reader = BufReader::new(file); 
    let mut tle_string = String::from("");

    for line in reader.lines() { 
        tle_string.push_str(line.unwrap().as_str());
        tle_string.push_str("\n");
    }
    
    let sat_elements = parse_3les(&tle_string).unwrap();
    
    // Only 1 sat element in the TLE
    let sat_constants = sgp4::Constants::from_elements(&sat_elements[0]).unwrap();
    
    let mut obs: Observer = Observer::new([51.507406923983446, -0.12773752212524414, 0.05]);
    let mut sat_state: satellite::Satellite = Satellite::new();

    for n in 0..=9 { 
        let elapsed_time = n * 10;
        let time_delta = chrono::TimeDelta::minutes(elapsed_time);
        let new_epoch = &sat_elements[0].datetime.checked_add_signed(time_delta).unwrap();
        // The propagate function returns position as TEME reference frame coordinates in km and
        // returns velocity in each dimension in terms of km/s
        let prediction: sgp4::Prediction = sat_constants.propagate(sgp4::MinutesSinceEpoch((elapsed_time) as f64))?;
        // println!("        ṙ = {:?} km.s⁻¹", prediction.velocity);

        // Sets satellite coordinates for all reference frames
        sat_state.update_sat_state(&prediction, &new_epoch);

        // Update observer state to pull in new teme coords with respect to new epoch
        obs.update_state(new_epoch);

    }

    Ok(())
}
