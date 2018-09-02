extern crate chrono;
extern crate chrono_tz;
extern crate timeago;
extern crate fs_extra;

use std::fmt;
use chrono::prelude::*;
use fs_extra::file;
use std::path::Path;
use fs_extra::dir;

pub struct Hrt {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub length: String,
}

impl Hrt {
    pub fn new(args: Vec<String>) -> Result<Hrt, &'static str> {
        let stored_file = Path::new("/Users/bea/.hrt/hrt.txt");

        if args.len() == 2 {
            dir::create_all("/Users/bea/.hrt/", false).unwrap();
            file::write_all(stored_file, &args[1]).unwrap();
            Hrt::since(&args[1])
        } else {
            let date = file::read_to_string(stored_file).unwrap();
            Hrt::since(&date)
        }
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
