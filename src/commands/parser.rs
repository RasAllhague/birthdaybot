use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

pub struct DateInputParser;

#[derive(Debug)]
pub struct ParserError;

impl DateInputParser {
    pub fn parse(&self, options: &[CommandDataOption]) -> Result<NaiveDateTime, ParserError>{
        let date_parts: Result<Vec<i64>, String> = (0..3)
            .map(|x| Self::get_int_option(options, x))
            .collect();

        let date_parts = date_parts.expect("User input expected.");

        if let Some(date) = NaiveDate::from_ymd_opt(date_parts[2] as i32, date_parts[1] as u32,date_parts[0] as u32) {
            if let Some(date) = date.and_hms_opt(0, 0, 0) {
                return Ok(date);
            }
        }

        Err(ParserError)
    }

    fn get_int_option(options: &[CommandDataOption], index: usize) -> Result<i64, String> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::Integer(data) = value {
                    return Ok(*data);
                }
            }
        }
    
        Err(format!("Option {} not found!", index))
    }
}

