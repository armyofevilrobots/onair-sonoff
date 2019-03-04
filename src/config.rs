extern crate clap;

use clap::{App, Arg, ArgMatches};

pub static DEFAULT_TAS: &str = &"http://sonoff-on-air.local/";

lazy_static! {
    pub static ref CONFIG: ArgMatches<'static> = App::new("rust-plantronics")
        .version("0.0.2")
        .author("Derek Anderson <derek@armyofevilrobots.com>")
        .about("Monitors state of a plantronics headset and sends events to various endpoints.")
        .arg(
            Arg::with_name("list_devices")
                .short("D")
                .long("list_devices")
                .help("List all available devices")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .help("ALSA device to monitor, ie: default, or dsnoop:CARD=BT600,DEV=0")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("rate")
                .short("r")
                .long("rate")
                .help("The sample rate to use. Lower numbers use less CPU.")
                .default_value("4000")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("tasmota")
                .short("T")
                .long("tasmota")
                .takes_value(true)
                .required(true)
                .conflicts_with("list_devices")
                .help("The destination url for the tasmota rest api (http://sonoff-on-air.local/)"),
        )
        .get_matches();
}
 
