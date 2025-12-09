#![allow(dead_code)]
use chrono::{Duration, TimeZone, Utc};

pub fn ntp_to_unix_time(ntp_time: u64) -> Duration {
    let int_part: i64 = (ntp_time >> 32).try_into().unwrap();
    let seconds = Duration::seconds(int_part);

    let frac_part: i64 = (ntp_time & u32::MAX as u64).try_into().unwrap();
    let milliseconds =
        Duration::milliseconds(((frac_part as f64 / u32::MAX as f64) * 1000.0) as i64);

    seconds + milliseconds
}

fn ntp_to_datetime(ntp_time: u64) -> chrono::DateTime<Utc> {
    let ntp_base = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();

    ntp_base + ntp_to_unix_time(ntp_time)
}

pub fn ntp_to_string(ntp_time: u64) -> String {
    let time = ntp_to_datetime(ntp_time);

    time.format("%Y-%m-%d %H:%M:%S%.f").to_string()
}

pub fn ntp_to_time_string(ntp_time: u64) -> String {
    let time = ntp_to_datetime(ntp_time);

    time.format("%H:%M:%S%.f").to_string()
}

pub fn ntp_to_f64(ntp_time: u64) -> f64 {
    let seconds: i64 = (ntp_time >> 32).try_into().unwrap();
    let frac_part: i64 = (ntp_time & u32::MAX as u64).try_into().unwrap();
    let fraction = frac_part as f64 / u32::MAX as f64;
    seconds as f64 + fraction
}

pub fn f64_to_ntp(time: f64) -> u64 {
    let secs = time.trunc() as u64;
    let frac_f = time.fract();
    let frac = (frac_f * (u32::MAX as f64)).round() as u32;

    (secs << 32) | (frac as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ntp_to_datetime_test() {
        let ntp_timestamp: u64 = 0xc59286ee1ba5e354;
        let datetime = ntp_to_datetime(ntp_timestamp);

        let valid_datetime =
            Utc.with_ymd_and_hms(2005, 1, 14, 17, 59, 10).unwrap() + Duration::milliseconds(108);

        assert_eq!(datetime, valid_datetime);
    }

    #[test]
    fn ntp_to_string_test() {
        let ntp_timestamp: u64 = 0xc59286ee1ba5e354;
        let datetime = ntp_to_string(ntp_timestamp);

        let valid_datetime = "2005-01-14 17:59:10.108".to_string();

        assert_eq!(datetime, valid_datetime);
    }

    const TEST_TIMESTAMP: u64 = 0xc59286ee1ba5e354;
    const EXPECTED_FLOAT: f64 = {
        let secs = (TEST_TIMESTAMP >> 32) as f64;
        let frac = (TEST_TIMESTAMP & 0xFFFF_FFFF) as u32 as f64 / (u32::MAX as f64);
        secs + frac
    };
    #[test]
    fn ntp_to_f64_test() {
        let float_time = ntp_to_f64(TEST_TIMESTAMP);
        let diff = (float_time - EXPECTED_FLOAT).abs();
        assert!(
            diff < 1.0e-9,
            "ntp_to_f64 returned `{}`, expected `{}` (diff `{}`)",
            float_time,
            EXPECTED_FLOAT,
            diff
        );
    }

    #[test]
    fn f64_to_ntp_test() {
        let back = f64_to_ntp(EXPECTED_FLOAT);
        assert_eq!(
            back, TEST_TIMESTAMP,
            "f64_to_ntp({}) returned {:#x}, expected {:#x}",
            EXPECTED_FLOAT, back, TEST_TIMESTAMP
        );
    }
}
