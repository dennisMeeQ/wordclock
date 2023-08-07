use chrono::{DateTime, Duration, Local, Timelike};

fn round_to_nearest(number: u32, nearest: i32) -> u32 {
    let rounded = (number as f64 / nearest as f64).round();
    (rounded * nearest as f64) as u32
}

// TODO: Take seconds into account when rounding
pub fn round_time_to_5_min(time: &DateTime<Local>) -> DateTime<Local> {
    let rounded_time = time.clone();

    let minutes = rounded_time.minute();
    let rounded_minutes = round_to_nearest(minutes, 5);

    rounded_time
        .checked_add_signed(Duration::minutes(rounded_minutes as i64 - minutes as i64))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn rounds_time() {
        let time = Local.with_ymd_and_hms(2023, 12, 19, 16, 57, 44).unwrap();
        let expected = Local.with_ymd_and_hms(2023, 12, 19, 16, 55, 44).unwrap();
        let result = round_time_to_5_min(&time);
        assert_eq!(result, expected);
    }

    #[test]
    fn rounds_time_over_hour() {
        let time = Local.with_ymd_and_hms(2023, 12, 19, 16, 58, 44).unwrap();
        let expected = Local.with_ymd_and_hms(2023, 12, 19, 17, 00, 44).unwrap();
        let result = round_time_to_5_min(&time);
        assert_eq!(result, expected);
    }
    #[test]
    fn rounds_time_down() {
        let time = Local.with_ymd_and_hms(2023, 12, 19, 16, 51, 44).unwrap();
        let expected = Local.with_ymd_and_hms(2023, 12, 19, 16, 50, 44).unwrap();
        let result = round_time_to_5_min(&time);
        assert_eq!(result, expected);
    }

    #[test]
    fn rounds_down_to_0() {
        let time = Local.with_ymd_and_hms(2023, 12, 19, 16, 1, 44).unwrap();
        let expected = Local.with_ymd_and_hms(2023, 12, 19, 16, 00, 44).unwrap();
        let result = round_time_to_5_min(&time);
        assert_eq!(result, expected);
    }

    #[test]
    fn rounds_up_to_nearest_5() {
        let result = round_to_nearest(33, 5);
        assert_eq!(result, 35);
    }

    #[test]
    fn rounds_up2_to_nearest_5() {
        let result = round_to_nearest(78, 5);
        assert_eq!(result, 80);
    }

    #[test]
    fn rounds_down_to_nearest_5() {
        let result = round_to_nearest(37, 5);
        assert_eq!(result, 35);
    }

    #[test]
    fn rounds_down2_to_nearest_5() {
        let result = round_to_nearest(71, 5);
        assert_eq!(result, 70);
    }

    #[test]
    fn does_not_round_exact() {
        let result = round_to_nearest(40, 5);
        assert_eq!(result, 40);
    }

    #[test]
    fn does_not_round_exact2() {
        let result = round_to_nearest(75, 5);
        assert_eq!(result, 75);
    }

    #[test]
    fn does_not_round_0() {
        let result = round_to_nearest(0, 5);
        assert_eq!(result, 0);
    }
}
