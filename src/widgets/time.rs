use serde::Serialize;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::NEUTRAL;
use crate::i3_status::CONFIG;
use crate::utils::file;
use crate::utils::macros::walk_to_number;
use crate::utils::macros::walk_until_with_condition;
use crate::utils::walking_vec::WalkingVec;
use crate::widgets::Widget;
use crate::widgets::WidgetError;

const SECONDS_IN_A_MINUTE: u64 = 60;
const SECONDS_IN_AN_HOUR: u64 = 60 * 60;
const SECONDS_IN_A_DAY: u64 = 24 * 60 * 60;

/*
    +---------------+---+
    |  magic    (4) |ver|
    +---------------+---+---------------------------------------+
    |           [unused - reserved for future use] (15)         |
    +---------------+---------------+---------------+-----------+
    |  isutcnt  (4) |  isstdcnt (4) |  leapcnt  (4) |
    +---------------+---------------+---------------+
    |  timecnt  (4) |  typecnt  (4) |  charcnt  (4) |
    +---------------+---------------+---------------+
*/
#[derive(Debug)]
struct TzHeader {
    pub version: u8,
    pub isutcnt: u32,
    pub isstdcnt: u32,
    pub leapcnt: u32,
    pub timecnt: u32,
    pub typecnt: u32,
    pub charcnt: u32,
}

/*
    +---------------------------------------------------------+
    |  transition times          (timecnt x TIME_SIZE)        |
    +---------------------------------------------------------+
    |  transition types          (timecnt)                    |
    +---------------------------------------------------------+
    |  local time type records   (typecnt x 6)                |
    +---------------------------------------------------------+
    |  time zone designations    (charcnt)                    |
    +---------------------------------------------------------+
    |  leap-second records       (leapcnt x (TIME_SIZE + 4))  |
    +---------------------------------------------------------+
    |  standard/wall indicators  (isstdcnt)                   |
    +---------------------------------------------------------+
    |  UT/local indicators       (isutcnt)                    |
    +---------------------------------------------------------+
*/
struct TzDataBlock {
    pub transition_times: Vec<i64>,
    pub transition_types: Vec<u8>,
    pub local_time_type_records: Vec<(i32, u8, u8)>,
    pub time_zone_designations: Vec<u8>,
    pub leap_second_records: Vec<(u64, i32)>,
    pub standard_wall_indicators: Vec<u8>,
    pub ut_local_indicators: Vec<u8>,
}

#[derive(Serialize)]
pub struct Time {
    // Name of the widget
    name: &'static str,
    // Text that will be shown in the status bar
    full_text: String,
    // Color of the text
    color: &'static str,
    // Timezone offset
    tz_offset: u64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            name: "time",
            full_text: String::new(),
            color: NEUTRAL,
            tz_offset: Self::timezone_offset(),
        }
    }

    fn is_leap_year(year: u64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn days_in_year(year: u64) -> u64 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn days_in_month(month: u64, year: u64) -> u64 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Enreachable Error: Invalid month: {month}"),
        }
    }

    fn read_tz_info(tz_info_bytes: &mut WalkingVec) -> Option<(TzHeader, TzDataBlock)> {
        // The hex values are ASCII values
        if tz_info_bytes.walk(4) != [0x54, 0x5A, 0x69, 0x66] {
            log::error!("Missing magic number in TZ file");
            return None;
        }

        // The hex values are ASCII values
        let version = match walk_to_number!(tz_info_bytes, u8) {
            0x00 => 1,
            0x32 => 2,
            0x33 => 3,
            0x34 => 4,
            _ => {
                log::error!("Invalid version number found");
                return None;
            }
        };

        // The next 15 bytes are reserved for future use
        tz_info_bytes.walk(15);

        // See https://datatracker.ietf.org/doc/html/rfc8536#section-3.1
        let header = TzHeader {
            version,
            isutcnt: walk_to_number!(tz_info_bytes, u32),
            isstdcnt: walk_to_number!(tz_info_bytes, u32),
            leapcnt: walk_to_number!(tz_info_bytes, u32),
            timecnt: walk_to_number!(tz_info_bytes, u32),
            typecnt: walk_to_number!(tz_info_bytes, u32),
            charcnt: walk_to_number!(tz_info_bytes, u32),
        };
        // https://datatracker.ietf.org/doc/html/rfc8536#section-3.2
        let mut data = TzDataBlock {
            transition_times: Vec::new(),
            transition_types: Vec::new(),
            local_time_type_records: Vec::new(),
            time_zone_designations: Vec::new(),
            leap_second_records: Vec::new(),
            standard_wall_indicators: Vec::new(),
            ut_local_indicators: Vec::new(),
        };
        let time_size = if header.version > 1 { 8 } else { 4 };

        println!("{:?}", header);
        if header.version > 1 {
            for _ in 0..(header.timecnt * time_size) {
                let num = walk_to_number!(tz_info_bytes, i64);
                if num > -2_i64.pow(59) {
                    data.transition_times.push(num);
                }
            }
        } else {
            for _ in 0..(header.timecnt * time_size) {
                let num = walk_to_number!(tz_info_bytes, i32) as i64;
                if num > -2_i64.pow(59) {
                    data.transition_times.push(num);
                }
            }
        };

        data.transition_types = walk_until_with_condition!(
            tz_info_bytes,
            0..header.timecnt,
            std::mem::size_of::<u8>(),
            |value: u8| (value as u32) < (header.typecnt - 1),
            u8,
            u8
        );

        for _ in 0..header.typecnt {
            let utoff = walk_to_number!(tz_info_bytes, i32);
            let dst = walk_to_number!(tz_info_bytes, u8);
            let idx = walk_to_number!(tz_info_bytes, u8);
            if (utoff != -2_i32.pow(32) && utoff > -89999 && utoff < 93599)
                && (dst == 0 || dst == 1)
            {
                data.local_time_type_records.push((utoff, dst, idx));
            }
        }

        data.time_zone_designations = walk_until_with_condition!(
            tz_info_bytes,
            0..header.charcnt,
            std::mem::size_of::<u8>(),
            |value: u8| (value as u32) < (header.typecnt - 1),
            u8,
            u8
        );

        for index in 0..header.typecnt {
            let occur = if header.version > 1 {
                walk_to_number!(tz_info_bytes, u64)
            } else {
                walk_to_number!(tz_info_bytes, u32) as u64
            };
            let corr = walk_to_number!(tz_info_bytes, i32);
            if index == 0 || occur - data.leap_second_records[index as usize].0 >= 2419199 {
                data.leap_second_records.push((occur, corr));
            }
        }

        data.standard_wall_indicators = walk_until_with_condition!(
            tz_info_bytes,
            0..header.isstdcnt,
            std::mem::size_of::<u8>(),
            |value: u8| value == 0 || value == 1,
            u8,
            u8
        );
        data.ut_local_indicators = walk_until_with_condition!(
            tz_info_bytes,
            0..header.isutcnt,
            std::mem::size_of::<u8>(),
            |value: u8| value == 0 || value == 1,
            u8,
            u8
        );

        Some((header, data))
    }

    fn timezone_offset() -> u64 {
        let offset = 0;
        let mut tz_info_bytes = file::read_file_to_vec(CONFIG.timezone()).unwrap();
        let tz_info = Self::read_tz_info(&mut tz_info_bytes);
        if let Some((header, data)) = tz_info {
            // Skip first header and data block if version 2 and above is used
            if header.version > 1 {
                let tz_info = Self::read_tz_info(&mut tz_info_bytes);
                if let Some((header_v2, data_v2)) = tz_info {}
            }
        }

        offset
    }

    fn now(&self) -> String {
        let mut epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + self.tz_offset;
        // Calculate the amount of days that is in the EPOCH by dividing EPOCH with the amount of seconds in a day
        let mut total_days = epoch / SECONDS_IN_A_DAY;
        // Calculate the remaining seconds by getting the remainder of the previous calculation
        epoch %= SECONDS_IN_A_DAY;
        // Calculate hours by dividing the remaining seconds by the amount of seconds in a hour
        let hours = epoch / SECONDS_IN_AN_HOUR;
        // Calculate the remaining seconds by getting the remainder of the previous calculation
        epoch %= SECONDS_IN_AN_HOUR;
        // Calculate minutes by dividing the remaining seconds by the amount of seconds in a minute
        let minutes = epoch / SECONDS_IN_A_MINUTE;
        // Calculate the remaining seconds by getting the remainder of the previous calculation
        let seconds = epoch % SECONDS_IN_A_MINUTE;

        // Calculate the current year with respect to leap years
        let mut current_year = 1970;
        while total_days >= Self::days_in_year(current_year) {
            total_days -= Self::days_in_year(current_year);
            current_year += 1;
        }

        // Calculate the current month with respect to leap years
        let mut current_month = 1;
        while total_days >= Self::days_in_month(current_month, current_year) {
            total_days -= Self::days_in_month(current_month, current_year);
            current_month += 1;
        }

        let current_day = total_days + 1;

        format!(
            "{:02}.{:02}.{current_year} {:02}:{:02}:{:02}",
            current_day, current_month, hours, minutes, seconds
        )
    }
}

impl Widget for Time {
    fn name(&self) -> &str {
        self.name
    }

    fn update(&mut self) {
        self.full_text = self.now();
    }

    fn display_text(&self) -> Result<Value, WidgetError> {
        Ok(serde_json::to_value(self)?)
    }
}
