use clap::{AppSettings, Arg, Command};
mod requester;
mod scrapper;
use regex::Regex;
use requester::req::Urlhandler;

fn cli() -> Command<'static> {
    Command::new("DnsDump")
        .version("OWL101")
        .author("Ali™")
        .about("DNS Record Scraper")
        .arg_required_else_help(true)
        .allow_hyphen_values(true)
        .setting(AppSettings::DeriveDisplayOrder)
        .bin_name("dnsdump")
        .subcommand(
            Command::new("dump")
                .alias("-s")
                .about("Dumps a dns list")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("url")
                        .short('u')
                        .long("url")
                        .takes_value(true)
                        .help("Dns Dump specified domain"),
                ),
        )
}

fn main() {
    // https://bobeespot.com/en/
    // https://github.com/clap-rs/clap/blob/v3.1.9/examples/git.rs

    let matches = cli().get_matches();
    let urlregex = Regex::new(
        r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
    ).unwrap();

    match matches.subcommand() {
        Some(("dump", submatch)) => {
            let arg = submatch.value_of("url").unwrap_or_else(|| "SunShine");
            let urlcaps = urlregex.captures(arg).is_none();
            if urlcaps == true{panic!("Please provide url like: google.com facebook.com etc")}

            let handler = Urlhandler::new(
                String::from("https://dnsdumpster.com/"),
                arg.to_owned()
            );
            let _ = handler.request();
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
