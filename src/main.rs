// Imports
use glicko2::*;
use serde::{Serialize,Deserialize};
use serde_json;
use chrono::{DateTime, TimeZone, Utc, Local};
use chrono::offset::LocalResult;
use chrono::offset::LocalResult::{Single, Ambiguous, None};
use std::fs;
use std::error::Error;
use std::collections::HashMap;


struct G2PlayerUpdate {
    before: G2Player,
    after: G2Player,
    outcomes: Vec<Outcome>,
}


#[derive(Copy,Clone)]
struct G2Player {
    mu: f64,
    phi: f64,
    vol: f64,
}


#[derive(Serialize,Deserialize,Debug)]
struct Outcome {
    #[serde(default)]
    ignore: bool,
    p1_name: String,
    p2_name: String,
    p1_gc: i8,
    p2_gc: i8,
    date_time: chrono::DateTime<Utc>,
}


impl Default for Outcome {
    fn default() -> Outcome {
        Outcome {
            ignore: false,
            p1_name: String::from("p1_name"),
            p2_name: String::from("p2_name"),
            p1_gc: 0,
            p2_gc: 0,
            date_time: Utc::now(),
        }
    }
}


/// Takes a path to a rating file `fp` and if it is a valid rating period file, returns a vector of
/// all the outcomes specified in the file. If either reading the file to a string or parsing the
/// contents as a vector of `Outcome`s fail, the function returns the error it encountered.
///
/// #### Parameters:
/// * `fp` a file path to a rating period file which this function will attempt to read and parse.
/// #### Return:
/// * An `Result<Vec<Outcome>, Box<dyn Error>` containing the vector of `Outcome`s specified in
///     the file if it was able to be read and parsed successfully, otherwise a `Box` containing
///     either an `std::io::Error` if reading the file failed or a `serde_json::Error` if parsing
///     the file failed.
fn read_rating_period_file(fp: &str) -> Result<Vec<Outcome>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(fp)?;
    let parsed_outcomes = serde_json::from_str::<Vec<Outcome>>(&file_contents)?;

    Ok(parsed_outcomes)
}

fn assign_outcomes_to_players(outcomes: &mut Vec<Outcome>, ) {
}


/// Converts a year, month, day integer pair from the local time zone to a full, complete, Utc
/// DateTime.
fn convert_ymd_to_utc_ymd_hms(year: i32, month: u32, day: u32) -> Result<DateTime<Utc>, String> {
    // A possible instance of the year, month, day, converted to a full DateTime<Local>
    let dt_res: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);

    // Return an error if the given year, month, day could NOT be converted to a single, valid
    // DateTime. Return the DateTime converted to UTC if the year, month, day could be converted
    // to a single, valid DateTime
    match dt_res {
        Single(s) => Ok(s.with_timezone(&Utc)),
        Ambiguous(_, _) => Err(String::from("Given date was ambiguous")),
        None => Err(String::from("Given date was invalid")),
    }
}


fn main() {
    let rating_period_ht: HashMap<&str, G2PlayerUpdate> = HashMap::new();

    println!("");

    let rating_period_file_fp = "TSE14PRO.json";

    let rating_period_outcomes = read_rating_period_file(rating_period_file_fp);
    match rating_period_outcomes {
        Ok(outcomes) => {
            for o in outcomes {
                println!("{o:#?}");
            }
        },
        Err(e) => {
            println!("ERROR: Could not read rating period file.\n \
                      \tINPUT: {rating_period_file_fp}\n \
                      \tERROR MESSAGE: {e:#?}");
            return ();
        }
    }
}
