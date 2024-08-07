use sgp4::Prediction;
use chrono::NaiveDateTime;
use crate::satellite::Satellite;
use crate::satutil::{degrees_to_radians, dot_prod, get_ecef, get_teme, radians_to_degrees, sub_vector, to_local_sidereal_time, to_sidereal
                };
use crate::coord_systems::{Geodetic, TEME, ECEF};

#[derive(Copy, Clone)]
pub struct Observer {
    pub geodetic_coords: Geodetic,
    pub teme_coords: TEME,
    pub ecef_coords: ECEF,
}

pub struct LookAngle { 
    pub azimuth: f64, 
    pub elevation: f64,
    pub range_mag: f64, 
    pub rate: f64,
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
        }
    }

    pub fn calculate_look_angle(&mut self, sat_coords: &Prediction, dt: &NaiveDateTime) -> LookAngle {
        self.teme_coords.sidereal = to_sidereal(dt);

        // ECI coords for ranges. Geodetic coords for everything else... hence the degrees to radians.
        let range_rate: [f64; 3] = sub_vector(&sat_coords.velocity, &self.teme_coords.velo_vector);
        let range: [f64; 3] = sub_vector(&sat_coords.position, &self.teme_coords.pos_vector);
        let range_magnitude = (range[0] * range[0] + range[1] * range[1] + range[2] * range[2]).sqrt();
        let theta = to_local_sidereal_time(degrees_to_radians(&self.geodetic_coords.longitude), &self.teme_coords.sidereal);
        let sin_lat = degrees_to_radians(&self.geodetic_coords.latitude).sin();
        let cos_lat = degrees_to_radians(&self.geodetic_coords.latitude).cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let top_s = sin_lat * cos_theta * range[0] 
            + sin_lat * sin_theta * range[1] - cos_lat * range[2];
        let top_e = -sin_theta * range[0]
            + cos_theta * range[1];
        let top_z = cos_lat * cos_theta * range[0]
            + cos_lat * sin_theta * range[1] + sin_lat * range[2];
        let mut az = (-top_e / top_s).atan();

        if top_s > 0.0 { 
            az += core::f64::consts::PI;
        }

        if az < 0.0 { 
            az += 2.0 * core::f64::consts::PI;
        }

        let el = (top_z / range_magnitude).asin();
        let rate = dot_prod(range, range_rate) / range_magnitude;
        
        LookAngle { 
            azimuth: radians_to_degrees(&az),
            elevation: radians_to_degrees(&el),
            range_mag: range_magnitude,
            rate: rate,
        }
    } 

    pub fn update_state(&mut self, new_epoch: &NaiveDateTime) {
        self.teme_coords = get_teme(&self.geodetic_coords, &new_epoch);
    }

    pub fn get_distance_to_sat(self, sat_state: &Satellite) -> f64 { 
        let mut d = (sat_state.ecef_coordinates.x - self.ecef_coords.x).powf(2.0) + (sat_state.ecef_coordinates.y - self.ecef_coords.y).powf(2.0) + (sat_state.ecef_coordinates.z - self.ecef_coords.z).powf(2.0);
        d = d.sqrt();
        d
    }
}