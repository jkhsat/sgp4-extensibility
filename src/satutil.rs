use sgp4::iau_epoch_to_sidereal_time;

const K_PI: f64 = 3.14159265358979323846264338327950288419716939937510582;
const K_TWO_PI: f64 = K_PI * 2.0;
const K_F: f64 = 1.0 / 298.26;
const K_XKMPER: f64 = 6378.135;

pub struct Geodetic { 
    pub s_lat: f64,
    pub s_lon: f64, 
    pub s_alt: f64,
}

pub fn to_geodetic(sat_prediction: sgp4::Prediction, sat_elements: &sgp4::Elements) -> core::result::Result<Geodetic, sgp4::Error> { 

    // Prediction x,y,z? degree
    let theta = ac_tan(sat_prediction.position[1], sat_prediction.position[0]);
    let r: f64 = ((sat_prediction.position[0] * sat_prediction.position[0]) + (sat_prediction.position[1] * sat_prediction.position[1])).sqrt();
    let e2: f64 = K_F * (2.0 - K_F);
    let mut c;
    let mut phi: f64;
    let mut cnt = 0;

    let mut lon: f64 = wrap_neg_pos_pi(theta - iau_epoch_to_sidereal_time(sat_elements.epoch()));
    let mut lat = ac_tan(sat_prediction.position[2], r);

    // phi = geocentric latitude
    // account for angle between equatorial plane and the point on the surface of the ellipse.
    loop { 
        phi = lat;
        let sinphi: f64 = phi.sin();
        c = 1.0 / (1.0 - e2 * sinphi * sinphi).sqrt();
        lat = ac_tan(sat_prediction.position[2] + K_XKMPER * c * e2 * sinphi, r);

        if (lat - phi).abs() < 1e-10 || cnt >= 10 { 
            break;
        }

        cnt += 1;
    }

    let alt: f64 = r / lat.cos() - K_XKMPER * c;

    lat = radians_to_degrees(lat);
    // TODO: this lon does not match the cpp port by Daniel Warner.
    //       double check lon calculations. From TLE parsing to here.
    //       Lat and Alt is pretty close, I believe the propagation is different
    //       between the neuromorphic implementation and the DWarner implementation.
    //       Need to find a known TLE / propagation to compare accuracy between the two.
    lon = radians_to_degrees(lon);

    let geo = Geodetic {
        s_lat: lat,
        s_lon: lon,
        s_alt: alt,
    };

    Ok(geo)

}

pub fn ac_tan(sin_x: f64, cos_x: f64) -> f64 { 
    if cos_x == 0.0 { 
        if sin_x > 0.0 { 
            return K_PI / 2.0;
        }
        else { 
            return 3.0 * K_PI / 2.0;
        }
    }
    else { 
        if cos_x > 0.0 { 
            return (sin_x / cos_x).atan();
        }
        else { 
            return K_PI + (sin_x / cos_x).atan();
        }
    }
}

pub fn wrap_neg_pos_pi(a: f64) -> f64 { 
    return mod_helper(a + K_PI, K_TWO_PI) - K_PI
}

pub fn mod_helper(x: f64, y: f64) -> f64 { 
    if y == 0.0 { 
        return x;
    }
    return x - y * (x / y).floor();
}

pub fn radians_to_degrees(radians: f64) -> f64 { 
    return radians * 180.0 / K_PI;
}