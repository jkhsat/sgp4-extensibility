use sgp4::iau_epoch_to_sidereal_time;

const K_PI: f64 = 3.14159265358979323846264338327950288419716939937510582;
const K_TWO_PI: f64 = K_PI * 2.0;
const K_F: f64 = 1.0 / 298.26;
const K_XKMPER: f64 = 6378.135;

// maybe sgp4 has these constants already 
const SM_AXIS: f64 = 6378.137; // Earth's semi-major axis.
const E2: f64 = 6.69437999014e-3; // Square of first eccentricity.

pub struct Geodetic { 
    pub geo_lat: f64,
    pub geo_lon: f64,
    pub geo_alt: f64,
}

pub struct ECEF {
    pub ecef_x: f64,
    pub ecef_y: f64,
    pub ecef_z: f64,
}

/// Convert TEME coordinates from sgp4::propagate to geodetic coordinates lat/long/alt
/// At time of writing this comment I am a rust noob... I am sure there is a better way to write
/// this function. I will figure that out later. 
pub fn to_geodetic(sat_prediction: sgp4::Prediction, sat_elements: &sgp4::Elements, t_elapsed: i64) -> core::result::Result<Geodetic, sgp4::Error> { 

    // Update a temp epoch with the elapsed time in order to calculate updated sidreal.
    // this fixes the mismatched longitude calculation between Daniel Warner and neuromorphic implementations
    // Think about refactoring this function now that it works?
    let t_delta: chrono::TimeDelta = chrono::TimeDelta::minutes(t_elapsed);
    let updated_time = &sat_elements.datetime.checked_add_signed(t_delta).unwrap();
    let theta = ac_tan(sat_prediction.position[1], sat_prediction.position[0]);
    let r: f64 = ((sat_prediction.position[0] * sat_prediction.position[0]) + (sat_prediction.position[1] * sat_prediction.position[1])).sqrt();
    let e2: f64 = K_F * (2.0 - K_F);
    let mut c;
    let mut phi: f64;
    let mut cnt = 0;
    let mut lon: f64 = wrap_neg_pos_pi(theta - iau_epoch_to_sidereal_time(sgp4::julian_years_since_j2000(updated_time)));
    let mut lat = ac_tan(sat_prediction.position[2], r);

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
    lon = radians_to_degrees(lon);

    let geo = Geodetic {
        geo_lat: lat,
        geo_lon: lon,
        geo_alt: alt,
    };

    Ok(geo)

}

// Observer?
/// 15 degree look angle max for sat tx
pub fn get_look_angle() {

}

// Observer?
/// Return the Satellite sub-points
/// I think this is basically just lat/long with altitude at 0... need to double check on celestrak
pub fn get_ssp() { 

}

// Observer?
// Also I keep using sgp4::Error... Maybe I should add a few error cases to that enum.
pub fn get_dist_to_satellite(observer: ECEF, satellite: ECEF) -> core::result::Result<ECEF, sgp4::Error> { 
    
    // Placeholders
    // Stick the 3d distance formula in here when I'm not so lazy.
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;

    let distance = ECEF  {
        ecef_x: x,
        ecef_y: y,
        ecef_z: z,
    };

    Ok(distance)
}

/// Takes in geodetic coordinates and converts them to ecef coordinates. 
/// 
/// Useful for computing the distance between observer and satellite or observer to SSP.
/// 
/// Return a struct of ECEF coordinates.
pub fn geodetic_to_ecef(geo_latitude: &f64, geo_longitude: &f64, geo_altitude: &f64) -> core::result::Result<ECEF, sgp4::Error> { 

    let lat_rad = degrees_to_radians(geo_latitude);
    let lon_rad: f64 = degrees_to_radians(geo_longitude);
    let alt_rad: f64 = degrees_to_radians(geo_altitude);

    let N = SM_AXIS / (1.0 - E2 * (lat_rad*lat_rad).sin());
    let x = (N + alt_rad) * lat_rad.cos() * lon_rad.cos();
    let y = (N + alt_rad) * lat_rad.cos() * lon_rad.sin();
    let z = ((1.0 - E2) * N + alt_rad) * lat_rad.sin();

    let ecef = ECEF { 
        ecef_x: x,
        ecef_y: y,
        ecef_z: z,
    };

    Ok(ecef)
}

fn ac_tan(sin_x: f64, cos_x: f64) -> f64 { 
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

fn wrap_neg_pos_pi(a: f64) -> f64 { 
    return mod_helper(a + K_PI, K_TWO_PI) - K_PI
}

fn mod_helper(x: f64, y: f64) -> f64 { 
    if y == 0.0 { 
        return x;
    }
    return x - y * (x / y).floor();
}

fn radians_to_degrees(radians: f64) -> f64 { 
    return radians * 180.0 / K_PI;
}

fn degrees_to_radians(degrees: &f64) -> f64 { 
    return degrees * K_PI / 180.0
}