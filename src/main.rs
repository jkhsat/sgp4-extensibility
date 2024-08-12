use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::slice_from_raw_parts_mut;
use satutil::{get_ecef, get_geodetic, radians_to_degrees};
use sgp4::parse_3les;
mod satutil;
pub mod observer;
pub mod satellite;
use crate::satellite::Satellite;
pub mod coord_systems;

fn main() -> anyhow::Result<()>{
    let file = File::open("src/tle3.txt").unwrap();
    let reader = BufReader::new(file); 
    let mut tle_string = String::from("");

    for line in reader.lines() { 
        tle_string.push_str(line.unwrap().as_str());
        tle_string.push_str("\n");
    }

    let mut gateway: observer::Observer = observer::Observer::new([0.00, 0.0, 0.00]);
    let mut terminal: observer::Observer = observer::Observer::new([45.00, 0.0, 0.00]);

    let mut sat_vec: Vec<Satellite> = Vec::new();
    let sat_elements = parse_3les(&tle_string).unwrap();

    for element in sat_elements { 
        let sat_state: satellite::Satellite = Satellite::new(element);
        sat_vec.push(sat_state);
    }

    for n in 0..=7200 {
        for satellite in 0..=sat_vec.len()-1 {
            let sat_constants = sgp4::Constants::from_elements(&sat_vec[satellite].sat_elements).unwrap();
            let elapsed_time = n;
            let time_delta = chrono::TimeDelta::minutes(elapsed_time);
            // let time_delta = chrono::TimeDelta::minutes(elapsed_time);
            let new_epoch = sat_vec[satellite].sat_elements.datetime.checked_add_signed(time_delta).unwrap();

            // The propagate function returns position as TEME reference frame coordinates in km and
            // returns velocity in each dimension in terms of km/s
            let prediction: sgp4::Prediction = sat_constants.propagate(sgp4::MinutesSinceEpoch((elapsed_time) as f64))?;
            // Sets satellite coordinates for all reference frames
            sat_vec[satellite].update_sat_state(&prediction, &new_epoch);
    
            // Update observer state to pull in new teme coords with respect to new epoch
            gateway.update_state(&new_epoch);
            terminal.update_state(&new_epoch);

            // Set the look angle values
            // gateway.calculate_look_angle(&prediction, &new_epoch);
            gateway.calculate_look_angle(&prediction, &new_epoch);
            terminal.calculate_look_angle(&prediction, &new_epoch);
            
            if  sat_vec[satellite].sat_elements.norad_id == 54755 && 
                radians_to_degrees(&terminal.look_angle.elevation) >= 15.0 &&
                radians_to_degrees(&gateway.look_angle.elevation) >= 15.0
                { 

                println!("Datetime: {}", new_epoch);

                println!("Gateway  ---- Satellite: {}\t Elevation Angle: {:.6}", sat_vec[satellite].sat_elements.norad_id, 
                                satutil::radians_to_degrees(&gateway.look_angle.elevation));

                println!("Terminal ---- Satellite: {}\t Elevation Angle: {:.6}", sat_vec[satellite].sat_elements.norad_id, 
                                satutil::radians_to_degrees(&terminal.look_angle.elevation));

                println!("Satellite --- Lat: {}\tLon: {}\tAlt: {}\t\n", sat_vec[satellite].geodetic_coordinates.latitude, 
                                sat_vec[satellite].geodetic_coordinates.longitude,
                                sat_vec[satellite].geodetic_coordinates.altitude);


            }

        }
        // println!("\n");
    }

    Ok(())
}
