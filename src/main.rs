use std::borrow::Borrow;
use std::{thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use sgp4::parse_3les;
mod satutil;

fn main() -> anyhow::Result<()> {

    // let elements_vec: Vec<sgp4::Elements> = response.into_json()?;
    let sleep_dur = time::Duration::from_millis( 31 );
    let one_sec = time::Duration::from_secs( 1 );
    // let ten_sec = time::Duration::from_secs( 10 );
    let one_day = time::Duration::from_secs( 60 * 60 * 24 );
    let now = time::Instant::now(); 
    let mut m_sec: f64 = 32.0 / 60000.0;

    // println!("{}", message);

    let file = File::open("src/tls.txt")?;
    let reader = BufReader::new(file);
    let mut tle_string: String = String::from("");

    for line in reader.lines() {
        let temp_string = line;

        tle_string.push_str(temp_string.unwrap().as_str());
        tle_string.push_str("\n");
    }

    let elements_vec = parse_3les(&tle_string)?;

    loop {
        // timestamp to verify speed
        let poll_timer = time::Instant::now();
        
        for element in &elements_vec {
            let constants = sgp4::Constants::from_elements(element)?;
            let prediction = constants.propagate(sgp4::MinutesSinceEpoch(( m_sec ) as f64))?;
            println!("        r = {:?} km", prediction.position);
            
            // custom function for converting TEME to ECEF
            // use this: https://mycoordinates.org/tracking-satellite-footprints-on-earth%E2%80%99s-surface/
            // satutil::to_ecef(&mut prediction.position)
        }
        assert!(poll_timer.elapsed() < sleep_dur);

        // Sleep 31ms to simulate 32Hz polling time. 
        let sleep_check = time::Instant::now();
        thread::sleep(sleep_dur);
        assert!(sleep_check.elapsed() >= sleep_dur);
        
        // Hey we slept for 32ms... increment 
        m_sec += 32.0 / 60000.0;

        // Stop case timestamp
        let timestamp = now.elapsed();
        //println!("        elapsed time: {timestamp:?}");
    
        if timestamp >= one_day {
            return Ok(())
        }
    }
}