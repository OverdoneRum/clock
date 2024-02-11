use chrono::{TimeZone, Utc};
use chrono_tz::Australia::{Adelaide, Brisbane, Darwin, Perth, Sydney};
use chrono_tz::Europe::London;
use chrono_tz::{NZ};

// get CLI argument for timezone and return as string
fn get_arguments() -> String {
    let args: Vec<_> = std::env::args().collect(); // get all arguments passed to app
    let input: String = args[1].to_string();
    return input;
}

fn main() {

    // CLI config - receive arguments from handler
    let input = get_arguments();

    // Timezone config - calculate timezone
    let naive_dt_utc = Utc::now().naive_utc();

    // Australia timezones
    let bne = Brisbane.from_utc_datetime(&naive_dt_utc);
    let syd = Sydney.from_utc_datetime(&naive_dt_utc);
    let per = Perth.from_utc_datetime(&naive_dt_utc);
    let adl = Adelaide.from_utc_datetime(&naive_dt_utc);
    let dar = Darwin.from_utc_datetime(&naive_dt_utc);

    // NZ timezones
    let nzl = NZ.from_utc_datetime(&naive_dt_utc);

    // UK timezones
    let ldn = London.from_utc_datetime(&naive_dt_utc);


    // let mut result: String = "unk".to_string();
    let result: String = input.to_string();


    let output = match result.as_str() {
        "bne" => bne.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "syd" => syd.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "per" => per.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "adl" => adl.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "dar" => dar.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "nzl" => nzl.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        "ldn" => ldn.format("%Y-%m-%d %H:%M:%S.%f").to_string().chars().skip(0).take(16).collect(),
        _ => "unknown timezone".to_string()
    };

    println!("{}", output);
}