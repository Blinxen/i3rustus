use serde::Serialize;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::NEUTRAL;
use crate::widgets::Widget;
use crate::widgets::WidgetError;

const SECONDS_IN_A_MINUTE: u64 = 60;
const SECONDS_IN_AN_HOUR: u64 = 60 * 60;
const SECONDS_IN_A_DAY: u64 = 24 * 60 * 60;

#[derive(Serialize)]
pub struct Time {
    // Name of the widget
    name: &'static str,
    // Text that will be shown in the status bar
    full_text: String,
    // Color of the text
    color: &'static str,
}

impl Time {
    pub fn new() -> Self {
        Self {
            name: "time",
            full_text: Self::now(),
            color: NEUTRAL,
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

    fn now() -> String {
        let mut epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
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
        self.full_text = Self::now();
    }

    fn display_text(&self) -> Result<Value, WidgetError> {
        Ok(serde_json::to_value(self)?)
    }
}
