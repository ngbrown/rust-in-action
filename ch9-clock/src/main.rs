mod clock;
mod ntp;

use crate::ntp::check_time;
use chrono::DateTime;
use clap::{App, Arg};
use clock::Clock;

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and sets the time.")
        .after_help(
            "Note: UNIX timestamps are paresd as whole \
            seconds since 1st January 1970 0:00:00 UTC. \
            For more accuracy, use another format.",
        )
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set", "check-ntp"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(Arg::with_name("datetime").help(
            "When <action> is 'set', apply <datetime>. \
                Otherwise, ignore.",
        ));

    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        Clock::set(t);

        let maybe_error = std::io::Error::last_os_error();
        let os_error_code = &maybe_error.raw_os_error();

        match os_error_code {
            Some(0) => (),
            Some(_) => eprintln!("Unable to set the time: {:?}", maybe_error),
            None => (),
        }
    } else if action == "check-ntp" {
        let offset = check_time().unwrap() as isize;

        println!("All servers combined (weighted) =>");
        println!(" {}ms away from local system time", offset);
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
