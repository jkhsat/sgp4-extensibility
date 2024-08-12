use std::fs::set_permissions;

use libm::cos;
use sgp4::{Prediction, WGS84};
use chrono::NaiveDateTime;
use crate::satellite::{self, Satellite};
use crate::satutil::{self, degrees_to_radians, dot_prod, get_ecef, get_geodetic, get_teme, radians_to_degrees, sub_vector, to_local_sidereal_time, to_sidereal
                };
use crate::coord_systems::{Geodetic, TEME, ECEF, LookAngle};

#[derive(Copy, Clone)]
pub struct Observer {
    pub geodetic_coords: Geodetic,
    pub teme_coords: TEME,
    pub ecef_coords: ECEF,
    pub look_angle: LookAngle,
}

impl Observer {
    pub fn new(coordinates: [f64; 3]) -> Observer { 
        Observer { 
            geodetic_coords: Geodetic { 
                latitude: coordinates[0],
                longitude: coordinates[1],
                altitude: coordinates[2],
            }, 
            teme_coords: TEME {
                ..Default::default()
            }, 
            ecef_coords: get_ecef( &Geodetic {
                latitude: coordinates[0],
                longitude: coordinates[1],
                altitude: coordinates[2],
            }),
            look_angle: LookAngle { 
                ..Default::default()
            }
        }
    }

    pub fn calculate_look_angle(&mut self, sat_coords: &Prediction, dt: &NaiveDateTime) {
        self.teme_coords.sidereal = to_sidereal(dt);
        let rad_long = degrees_to_radians(&self.geodetic_coords.longitude);
        let rad_lat = degrees_to_radians(&self.geodetic_coords.latitude);
        let theta = to_local_sidereal_time(rad_long, &self.teme_coords.sidereal);
            
        let distance_vector: [f64; 3] = [sat_coords.position[0] - self.teme_coords.pos_vector[0],
                                         sat_coords.position[1] - self.teme_coords.pos_vector[1],
                                         sat_coords.position[2] - self.teme_coords.pos_vector[2]];

        let distance = (distance_vector[0].powf(2.0) + distance_vector[1].powf(2.0) + distance_vector[2].powf(2.0)).sqrt();

        let sin_lat = rad_lat.sin();
        let cos_lat = rad_lat.cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();        

        let top_s = sin_lat * cos_theta * distance_vector[0] + sin_lat * sin_theta * distance_vector[1] - cos_lat * distance_vector[2];
        let top_e = -sin_theta * distance_vector[0] + cos_theta * distance_vector[1];
        let top_z = cos_lat * cos_theta * distance_vector[0] + cos_lat * sin_theta * distance_vector[1] + sin_lat * distance_vector[2];
        let mut az = (-top_e / top_s).atan();
        
        if top_s > 0.0 { 
            az += core::f64::consts::PI;
        }

        if az < 0.0 { 
            az += core::f64::consts::PI * 2.0;
        }

        let el = (top_z / distance).asin(); 
        
        self.look_angle = LookAngle { 
            azimuth: az,
            elevation: el,
            distance: distance,
        }
    } 


    // pub fn calculate_look_angle(&self, sat_location: &Satellite) -> f64 { 
    //     // central angle = cos(Le) * Cos(Ls) * cos(ls - le) + sin Le * sine Ls
    //     // Le = observer latitiude
    //     // le = observer longitude
    //     // Ls = latitude of satellite 
    //     // ls = longitude of satellite

    //     let sat_lat:f64 = -105.0;
    //     let sat_lon:f64 = 0.0; 
    //     let sat_alt = 42164.0 - WGS84.ae;

    //     let theta = (self.geodetic_coords.latitude.cos() * sat_lat.cos() * 
    //         (sat_lon - self.geodetic_coords.longitude).cos() + self.geodetic_coords.latitude.sin() * 
    //         sat_lat.sin()).acos();
        
    //     println!("theta: {}", radians_to_degrees(&theta));

    //     let elevation = (theta.sin() / (1.0 + ((WGS84.ae) / (WGS84.ae + sat_alt)).powf(2.0) - 
    //                                                                 2.0*(WGS84.ae / (WGS84.ae + sat_alt))*theta.cos()).sqrt()).acos();
                                                                    
    //     println!("elevation {}", elevation);

    //     elevation
    // }

    pub fn update_state(&mut self, new_epoch: &NaiveDateTime) {
        self.teme_coords = get_teme(&self.geodetic_coords, &new_epoch);
    }

    pub fn get_obs_status(&self) -> Observer { 
        Observer { 
            geodetic_coords: self.geodetic_coords,
            teme_coords: self.teme_coords,
            ecef_coords: self.ecef_coords,
            look_angle: self.look_angle,
        }
    }
}