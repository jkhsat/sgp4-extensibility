use std::num::Wrapping;

use chrono::DateTime;
use serde::de;
use sgp4::iau_epoch_to_sidereal_time;
use sgp4::{Elements, Error, MinutesSinceEpoch};
use libm::fabs;

const kPI: f64 = 3.14159265358979323846264338327950288419716939937510582;
const kTwoPi: f64 = kPI * 2.0;
const TicksPerDay: i128         =  86400000000;
const TicksPerHour: i128        =  3600000000;
const TicksPerMinute: i128      =  60000000;
const TicksPerSecond: i128      =  1000000;
const TicksPerMillisecond: i128 =  1000;
const TicksPerMicrosecond: i128 =  1;
const kF: f64 = 1.0 / 298.26;
const kXKMPER: f64 = 6378.135;

pub struct Geodetic { 
    pub geo_pos: [f64; 3],
}

pub fn to_geodetic(sat_prediction: sgp4::Prediction, sat_elements: &sgp4::Elements) -> core::result::Result<Geodetic, sgp4::Error> { 

    // Prediction x,y,z? degree
    let theta = ac_tan(sat_prediction.position[1], sat_prediction.position[0]);

    // maybe change timestamp to be microseconds instead of seconds. 
    let mut lon: f64 = wrap_neg_pos_pi(theta - iau_epoch_to_sidereal_time(sat_elements.epoch()));


    let r: f64 = ((sat_prediction.position[0] * sat_prediction.position[0]) + (sat_prediction.position[1] * sat_prediction.position[1])).sqrt();
    
    let e2: f64 = kF * (2.0 - kF);

    let mut lat = ac_tan(sat_prediction.position[2], r);
    let mut phi = 0.0;
    let mut c = 0.0;
    let mut cnt = 0;
        
    loop { 
        phi = lat;
        let sinphi: f64 = phi.sin();
        c = 1.0 / (1.0 - e2 * sinphi * sinphi).sqrt();
        lat = ac_tan(sat_prediction.position[2] + kXKMPER * c * e2 * sinphi, r);

        if fabs(lat - phi) < 1e-10 || cnt >= 10 { 
            break;
        }

        cnt += 1;
    }

    let alt: f64 = r / lat.cos() - kXKMPER * c;

    lat = radians_to_degrees(lat);
    // TODO: this lon does not match the cpp port by Daniel Warner.
    //       double check lon calculations. From TLE parsing to here.
    //       Lat and Alt is pretty close, I believe the propagation is different
    //       between the neuromorphic implementation and the DWarner implementation.
    //       Need to find a known TLE / propagation to compare accuracy between the two.
    lon = radians_to_degrees(lon);

    Ok(Geodetic {
        geo_pos: [
            lat,
            lon,
            alt,
        ],
    })
}

pub fn ac_tan(sin_x: f64, cos_x: f64) -> f64 { 
    if cos_x == 0.0 { 
        if sin_x > 0.0 { 
            return kPI / 2.0;
        }
        else { 
            return 3.0 * kPI / 2.0;
        }
    }
    else { 
        if cos_x > 0.0 { 
            return (sin_x / cos_x).atan();
        }
        else { 
            return kPI + (sin_x / cos_x).atan();
        }
    }
}

pub fn wrap_neg_pos_pi(a: f64) -> f64 { 
    return mod_helper(a + kPI, kTwoPi) - kPI
}

pub fn mod_helper(x: f64, y: f64) -> f64 { 
    if y == 0.0 { 
        return x;
    }
    return x - y * (x / y).floor();
}

pub fn degrees_to_radians(degree: f64) -> f64 { 
    return degree * kPI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 { 
    return radians * 180.0 / kPI;
}