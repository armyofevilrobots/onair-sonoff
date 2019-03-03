#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate dns_lookup;
extern crate rand;
extern crate url;
extern crate cpal;

// use reqwest;
//use rand::seq::SliceRandom;
use cpal::EventLoop;
use rand::{seq::SliceRandom, thread_rng};
use std::env;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use url::{Host, Url};

mod config;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


fn bonjour_lookup_host(hostname: &str) -> Result<String> {
    let mut ips: Vec<std::net::IpAddr> = dns_lookup::lookup_host(hostname)?;
    // Oh crap, this order may be stable, but we want to randomly try them
    // in case one IP is broken/unroutable, or whatevs.
    // let _ips: &[std::net::IpAddr] = ips.as_mut_slice();
    let mut rng = thread_rng();
    ips.shuffle(&mut rng);
    for ip in ips {
        debug!("An IP: {:?}", ip);
        return Ok(ip.to_string());
    }
    Ok(hostname.into())
}

fn rewrite_url_with_mdns(url: &str) -> Result<String> {
    let mut url_obj = Url::parse(url)?;
    match url_obj.host() {
        Some(Host::Domain(host_name)) => {
            debug!("Rewriting URL host {}", &host_name);
            let host = bonjour_lookup_host(&host_name)?;
            url_obj.set_host(Some(&host))?;
            info!("Rewrote URL to {}", url_obj.as_str());
            return Ok(url_obj.to_string());
        }
        Some(Host::Ipv4(addr)) => {
            debug!("Received an IPV4 host {}, no rewrite.", &addr);
        }
        Some(Host::Ipv6(addr)) => {
            debug!("Received an IPV6 host {}, fancy!", &addr);
        }
        None => {}
    }
    warn!(
        "Could not lookup host for {}, so using raw, may be flaky!",
        url
    );
    Ok(url.into())
}

// Changes the state of the Tasmota to the one defined
fn tasmota_set_state(url: &str, state: bool) -> Result<bool>{
    let result = match state {
        true => reqwest::get(&format!(
            "{tasmota}cm?cmnd=Power%20On",
            tasmota = url
        )),
        false => reqwest::get(&format!(
            "{tasmota}cm?cmnd=Power%20Off",
            tasmota = url
        ))
    };
    match result {
        Ok(_) => Ok(state),
        Err(err) => Err(err.into()),
    }
}

fn iterate_devices() -> Vec<String>{
    let mut out:Vec<String> = Vec::new();
    for device in cpal::devices(){
        out.push(device.name());
    }
    out
}

fn main() {
    // Set the default env var the easy way...
    match env::var_os("RUST_LOG") {
        Some(_) => {}
        None => {
            env::set_var("RUST_LOG", "info");
        }
    }

    // Initialize logging
    pretty_env_logger::init();
    let delay_time = Duration::from_secs(1);

    loop {
        let _ = config::CONFIG; // Force immediate resolution, in case we need to print help...
        if config::CONFIG.is_present("list_devices"){
            for item in iterate_devices(){
                info!("Found device {}", &item);
            }
        }

        let devices = iterate_devices();




        let tasmota = match rewrite_url_with_mdns(
            config::CONFIG
                .value_of("tasmota")
                .unwrap_or(config::DEFAULT_TAS),
        ) {
            Ok(tasmota) => tasmota,
            Err(e) => {
                error!("Unable to lookup host :( {}", e);
                warn!("Sleeping for a couple seconds and retrying...");
                sleep(Duration::from_secs(2));
                continue;
            }
        };

        let event_loop = EventLoop::new();
        let device = cpal::default_input_device().expect("Nope!");
        info!("device: {}", device.name());
        let in_format = cpal::Format {
            channels: 1,
            sample_rate: cpal::SampleRate(2048),
            data_type: cpal::SampleFormat::I16,
        }; 

        let in_listen = event_loop.build_input_stream(&device, &in_format).unwrap();
        let mut muted_count: u32 = 0;
        let mut sign_state = true;
        tasmota_set_state(&tasmota, sign_state);
        event_loop.run(move |_stream_id, mut stream_data| {
            let now = SystemTime::now();
            muted_count = (muted_count+1);
            if muted_count > 1024 { muted_count = 512 }
            match(stream_data){
                cpal::StreamData::Input{ buffer: cpal::UnknownTypeInputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter(){
                        if cpal::Sample::to_i16(elem) != 0 {
                            muted_count = 0;
                        }
                    }
                }
                _ => {
                    info!("Unmuted.")
                }
            }
            if muted_count>16 && (muted_count % 32) == 0{
                debug!("Muted_count is {}", muted_count);
            }

            if muted_count > 32 && sign_state{
                match tasmota_set_state(&tasmota, false){
                    Ok(out_state) => {
                        sign_state = out_state;
                        info!("Turned off sign.")
                    },
                    Err(_) =>()
                }
                
            }else if muted_count < 32 &&!sign_state{
                match tasmota_set_state(&tasmota, true){
                    Ok(out_state) => {
                        sign_state = out_state;
                        info!("Turned on sign.")
                    },
                    Err(_) =>()
                }
                
            }
            //sleep(delay_time);
            match now.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() > 30 {
                        error!(
                            "Unexpectedly long delay of {} seconds. Restarting.",
                            elapsed.as_secs()
                        );
                        return;
                    }
                }
                Err(e) => {
                    info!("Error occured {:?}", e);
                    return;
                }
            }
        });
        warn!("We exited the polling loop. Restarting...")
    }
}
