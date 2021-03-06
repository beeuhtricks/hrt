use chrono::prelude::*;
use std::fmt;

pub struct Hrt {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub length: String,
}

impl Hrt {
    pub fn new(args: Vec<String>) -> Result<Hrt, &'static str> {
        Hrt::since(&args[1])
    }

    fn since(start: &str) -> Result<Hrt, &'static str> {
        let start_fixed = chrono::DateTime::parse_from_rfc2822(start).unwrap();
        let start = start_fixed.with_timezone(&Utc);
        let end = Local::now().with_timezone(&Utc);
        let length = Hrt::time_since(start, end);

        Ok(Hrt { start, end, length })
    }

    fn time_since(from: DateTime<Utc>, to: DateTime<Utc>) -> String {
        let mut f = timeago::Formatter::new();
        f.num_items(4);

        f.convert_chrono(from, to)
    }
}

impl fmt::Display for Hrt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.length)
    }
}
