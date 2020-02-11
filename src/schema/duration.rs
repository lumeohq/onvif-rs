use crate::utils;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug)]
pub struct Duration {
    pub is_negative: bool,

    pub years: u64,
    pub months: u64,
    pub days: u64,

    pub hours: u64,
    pub minutes: u64,
    pub seconds: f64,
}

impl Duration {
    pub fn to_lexical_representation(&self) -> String {
        let mut s = if self.is_negative {
            "-P".to_string()
        } else {
            "P".to_string()
        };

        let mut date_str = String::new();
        if self.years > 0 {
            date_str.push_str(&format!("{}Y", self.years));
        }
        if self.months > 0 {
            date_str.push_str(&format!("{}M", self.months));
        }
        if self.days > 0 {
            date_str.push_str(&format!("{}D", self.days));
        }

        let mut time_str = String::new();
        if self.hours > 0 {
            time_str.push_str(&format!("{}H", self.hours));
        }
        if self.minutes > 0 {
            time_str.push_str(&format!("{}M", self.minutes));
        }
        if self.seconds > 0.0 {
            time_str.push_str(&format!("{}S", self.seconds));
        }

        if time_str.is_empty() {
            if date_str.is_empty() {
                s.push_str("0Y");
            } else {
                s.push_str(&date_str);
            }
        } else {
            s.push_str(&date_str);
            s.push_str("T");
            s.push_str(&time_str);
        }

        s
    }

    pub fn to_std_duration(&self) -> Result<std::time::Duration, String> {
        if self.years > 0 || self.months > 0 {
            Err("Duration with months or years require a starting date to be converted".into())
        } else {
            let secs = self.seconds as u64;

            let nanos = ((self.seconds - secs as f64) * 1e9) as u32;
            let secs = secs + 60 * (self.minutes + 60 * (self.hours + 24 * self.days));

            Ok(std::time::Duration::new(secs, nanos))
        }
    }

    // TODO: Add a version of to_std_duration that takes a moment at time to start from.

    // TODO: Implement normalization function that takes a moment at time to start from and
    // converts months & years to days.

    pub fn from_lexical_representation(s: &str) -> Result<Duration, String> {
        fn fill_component(
            context: &mut ParsingContext,
            component: &mut u64,
            idx: i32,
            name: &str,
            symbol: char,
        ) -> Result<(), String> {
            if context.is_number_empty {
                return Err(format!(
                    "No value is specified for {}, so '{}' must not be present",
                    name, symbol
                ));
            }

            if context.is_dot_found {
                return Err("Only the seconds can be expressed as a decimal".into());
            }

            if context.last_filled_component >= idx {
                return Err("Bad order of duration components".into());
            }

            *component = context.number;
            context.last_filled_component = idx;
            context.number = 0;
            context.is_number_empty = true;

            Ok(())
        }

        fn fill_seconds(
            context: &mut ParsingContext,
            seconds: &mut f64,
        ) -> Result<(), &'static str> {
            if context.is_number_empty {
                return Err("No value is specified for seconds, so 'S' must not be present");
            }

            if context.is_dot_found && context.denom == 1 {
                return Err("At least one digit must follow the decimal point if it appears");
            }

            if context.last_filled_component >= 6 {
                return Err("Bad order of duration components");
            }

            *seconds = context.number as f64 + context.numer as f64 / context.denom as f64;
            context.last_filled_component = 6;
            context.number = 0;
            context.is_number_empty = true;

            Ok(())
        }

        let mut dur: Duration = Default::default();
        let mut context = ParsingContext::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '-' => {
                    if i == 0 {
                        dur.is_negative = true;
                    } else {
                        return Err("The minus sign must appear first".into());
                    }
                }
                'P' => {
                    if i == 0 || i == 1 && dur.is_negative {
                        context.is_p_found = true;
                    } else {
                        return Err("Symbol 'P' occurred at the wrong position".into());
                    }
                }
                'T' => {
                    if context.is_t_found {
                        return Err("Symbol 'T' occurred twice".into());
                    }

                    if context.number > 0 {
                        return Err("Symbol 'T' occurred after a number".into());
                    }

                    context.is_t_found = true;
                    context.last_filled_component = 3;
                }

                // Duration components:
                'Y' => {
                    fill_component(&mut context, &mut dur.years, 1, "years", 'Y')?;
                }
                'M' => {
                    if context.is_t_found {
                        fill_component(&mut context, &mut dur.minutes, 5, "minutes", 'M')?;
                    } else {
                        fill_component(&mut context, &mut dur.months, 2, "months", 'M')?;
                    }
                }
                'D' => {
                    fill_component(&mut context, &mut dur.days, 3, "days", 'D')?;
                }
                'H' => {
                    if !context.is_t_found {
                        return Err("No symbol 'T' found before hours components".into());
                    }
                    fill_component(&mut context, &mut dur.hours, 4, "hours", 'H')?;
                }
                'S' => {
                    if !context.is_t_found {
                        return Err("No symbol 'T' found before seconds components".into());
                    }
                    fill_seconds(&mut context, &mut dur.seconds)?;
                }

                // Number:
                '.' => {
                    if context.is_dot_found {
                        return Err("Dot occurred twice".into());
                    }

                    if context.is_number_empty {
                        return Err("No digits before dot".into());
                    }

                    context.is_dot_found = true;
                }
                digit => {
                    if !digit.is_digit(10) {
                        return Err("Incorrect character occurred".into());
                    }

                    if context.is_dot_found {
                        context.numer *= 10;
                        context.numer +=
                            digit.to_digit(10).expect("error converting a digit") as u64;
                        context.denom *= 10;
                    } else {
                        context.number *= 10;
                        context.number +=
                            digit.to_digit(10).expect("error converting a digit") as u64;
                        context.is_number_empty = false;
                    }
                }
            }
        }

        if context.number > 0 {
            return Err("Number at the end of the string".into());
        }

        if !context.is_p_found {
            return Err("'P' must always be present".into());
        }

        if context.last_filled_component == 0 {
            return Err("At least one number and designator are required".into());
        }

        if context.last_filled_component <= 3 && context.is_t_found {
            return Err("No time items are present, so 'T' must not be present".into());
        }

        Ok(dur)
    }
}

struct ParsingContext {
    is_p_found: bool,           // Is 'P' found in the string.
    is_t_found: bool,           // Is 'T' delimiter occurred.
    last_filled_component: i32, // 1 to 6 for Year to Minute.

    number: u64,
    is_number_empty: bool,

    is_dot_found: bool,
    // Numerator and denominator of seconds fraction part.
    numer: u64,
    denom: u64,
}

impl ParsingContext {
    pub fn new() -> ParsingContext {
        ParsingContext {
            is_p_found: false,
            is_t_found: false,
            last_filled_component: 0,

            number: 0,
            is_number_empty: true,

            is_dot_found: false,
            numer: 0,
            denom: 1,
        }
    }
}

impl YaDeserialize for Duration {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| Duration::from_lexical_representation(s))
    }
}

impl YaSerialize for Duration {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Duration", writer, |s| {
            Ok(s.to_lexical_representation())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(s: &str) {
        if let Err(err) = Duration::from_lexical_representation(s) {
            panic!("{} should be valid, but an error occurred: {}", s, err)
        }
    }

    fn check_invalid(s: &str) {
        if let Ok(_) = Duration::from_lexical_representation(s) {
            panic!("{} should be invalid", s)
        }
    }

    #[test]
    fn duration_parsing_test() {
        check_valid("P2Y6M5DT12H35M30S");
        check_valid("P1DT2H");
        check_valid("P20M");
        check_valid("PT20M");
        check_valid("P0Y20M0D");
        check_valid("P0Y");
        check_valid("-P60D");
        check_valid("PT1M30.5S");

        check_invalid("P-20M");
        check_invalid("P20MT");
        check_invalid("P1YM5D");
        check_invalid("P15.5Y");
        check_invalid("P1D2H");
        check_invalid("1Y2M");
        check_invalid("P2M1Y");
        check_invalid("P");
        check_invalid("PT15.S");
    }
}
