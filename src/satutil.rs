use libm::floor;
use libm::pow;
use libm::atan;

const kAE: f64  = 1.0;
const kQ0: f64  = 120.0;
const kS0: f64  = 78.0;
const kMU: f64  = 398600.8;
const kXKMPER : f64 = 6378.135;
const kXJ2: f64  = 1.082616e-3;
const kXJ3: f64  = -2.53881e-6;
const kXJ4: f64  = -1.65597e-6;
const kPI: f64 = 3.14159265358979323846264338327950288419716939937510582;
const kTWOPI: f64 = 2.0 * kPI;

// Need to use non-const values for this... dumb that rust can't do
// non-const globals. Should I create a separate file for globals?
pub fn get_kQOMS2T() -> f64 {
    let non_const_kQ = kQ0;
    let non_const_kS = kS0;
    let non_const_kXKMPER = kXKMPER;

    return pow((non_const_kQ - non_const_kS) / non_const_kXKMPER, 4.0);
}

pub fn to_ecef(teme_coords: &mut [f64; 3]) { 
    println!("coords: {:?}", teme_coords);

    // Calc Greenwich Mean Sidereal Tiem (GMST)

    // Define earth rotation matrix

    // Dot product teme and earth rotation matrix

    // profit???
}

// Do this later 
pub fn geodetic_to_ecef(geodet_coords: [f64; 3]) -> [f64; 3] {
    let mut ecef_coords: [f64; 3] = [0.0, 0.0, 0.0];


    ecef_coords
}

pub fn check_look_angle(r_lat: f64, r_lon: f64, s_lat: f64, s_lon: f64) -> f64 {
    let mut look_angle = 0.0;

    look_angle
}

pub fn to_geodetic(pos_y: f64, pos_x: f64) {
    // This is going to suck
}

pub fn Mod(x: f64, y: f64) -> f64 { 
    if y == 0.0 { 
        return x;
    }
    x - y * floor(x / y)
}

pub fn wrap_neg_pos_pi(a: f64) -> f64 { 
    Mod(a + kPI, kTWOPI) - kPI
}

pub fn wrap_two_pi(a: f64) -> f64 { 
    Mod(a, kTWOPI)
}

pub fn wrap_neg_pos_180(a: f64) -> f64 {
    Mod(a + 180.0, 360.0) - 180.0
}

pub fn wrap_360(a: f64) -> f64 { 
    Mod(a, 360.0)
}

pub fn degrees_to_radians(degrees: f64) -> f64 { 
    degrees * kPI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 { 
    radians * 180.0 / kPI
}

pub fn ac_tan(sinx: f64, cosx: f64) -> f64 { 
    if cosx == 0.0 { 
        if sinx > 0.0 {
            return kPI / 2.0;
        }
        else { 
            return 3.0 * kPI / 2.0;
        }
    }
    else { 
        if cosx > 0.0 { 
            return atan(sinx / cosx);
        }
        else { 
            return kPI + atan(sinx / cosx);
        }
    }
}