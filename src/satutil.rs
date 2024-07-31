use sgp4::{Error, MinutesSinceEpoch};
use libm::atan;

const kPI: f64 = 3.14159265358979323846264338327950288419716939937510582;

pub struct Geodetic { 
    pub geo_pos: [f64; 3],
}

pub fn to_geodetic(sat_prediction: sgp4::Prediction, sat_elements: sgp4::Elements) -> core::result::Result<Geodetic, sgp4::Error> { 

    // Prediction x,y,z? 
    let theta = ac_tan(sat_prediction.position[1], sat_prediction.position[0]);
    let date_time: chrono::naive::NaiveDateTime = sat_elements.datetime;

    let lon: f64 = wrap_neg_pos_pi(theta, 
        to_greenwich_sidereal_time(
            sat_elements.datetime_to_minutes_since_epoch(&sat_elements.datetime).unwrap().0));

    Ok(Geodetic {
        geo_pos: [
            0.0,
            0.0,
            0.0,
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
            return atan(sin_x / cos_x);
        }
        else { 
            return kPI + atan(sin_x / cos_x);
        }
    }
}

pub fn wrap_neg_pos_pi(theta: f64, greenwich_sr_time: f64) -> f64 { 

}

pub fn to_greenwich_sidereal_time(dt: f64) -> f64 { 

}