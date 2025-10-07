pub fn parse_time(seconds: &u64) -> String {
    match seconds {
        seconds if *seconds < 60 => format!("{seconds} sec"),
        seconds if *seconds < 7200 => {
            let min = seconds / 60;
            format!("{min} min")
        }
        seconds if *seconds < 86400 => {
            let hours = seconds / 3600;
            format!("{hours} h")
        }
        seconds if *seconds < 604800 => {
            let days = seconds / 86400;
            format!("{days} d")
        }
        seconds if *seconds < 2592000 => {
            let weeks = seconds / 604800;
            format!("{weeks} w")
        }
        _ => {
            let month = seconds / 2592000;
            format!("{month} m")
        }
    }
}

#[allow(warnings)]
mod test {
    use crate::util::parse_time;

    #[test]
    fn it_returns_duration_in_seconds() {
        assert_eq!(parse_time(&10), "10 sec")
    }

    #[test]
    fn it_returns_duration_in_minutes() {
        assert_eq!(parse_time(&61), "1 min");
        assert_eq!(parse_time(&301), "5 min");
        assert_eq!(parse_time(&360), "6 min");
        assert_eq!(parse_time(&3600), "60 min");
        assert_eq!(parse_time(&5460), "91 min");
    }

    #[test]
    fn it_returns_duration_in_hours() {
        assert_eq!(parse_time(&10800), "3 h");
        assert_eq!(parse_time(&43200), "12 h");
    }

    #[test]
    fn it_returns_duration_in_days() {
        assert_eq!(parse_time(&86400), "1 d");
        assert_eq!(parse_time(&433000), "5 d");
    }

    #[test]
    fn it_returns_duration_in_weeks() {
        assert_eq!(parse_time(&691200), "1 w");
        assert_eq!(parse_time(&1815400), "3 w");
    }

    #[test]
    fn it_returns_duration_in_month() {
        assert_eq!(parse_time(&2592000), "1 m");
        assert_eq!(parse_time(&7776000), "3 m");
        assert_eq!(parse_time(&15555000), "6 m");
    }
}
