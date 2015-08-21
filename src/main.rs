// Custom derive for Serde's Serialize/Deserialize
// Compiler plugins for Serde
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]


extern crate rustty;
extern crate simplemad;
extern crate cpal;
extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io::Read;

use hyper::Url;
use hyper::Client as HttpClient;


mod responses {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {
        pub status: String,
        pub version: String
    }

}


#[derive(Debug)]
struct Config {
    base_url: Url,
}

impl Config {
    // TODO: report result
    pub fn new(username: &str, password: &str, host: &str, port: Option<u16>) -> Option<Config> {
        let url_string = format!("https://{}:{}/rest?u={}&p={}&v=1.11.0&c=roadkill-{}&f=json", 
                                 host,
                                 port.unwrap_or(8443),
                                 username,
                                 password,
                                 "0.1.0");
        Url::parse(&url_string).ok().map(|base_url| Config { base_url: base_url })
    }
}

struct SubsonicClient {
    config: Config,
    client: HttpClient,
}

#[derive(Serialize, Deserialize, Debug)]
enum Foo {
    Bar {
        x: i32,
        y: i32,
    },
    Qux(String, String)
}

impl SubsonicClient {
    pub fn new(config: Config) -> SubsonicClient {
        SubsonicClient { config: config, client: HttpClient::new() }
    }

    pub fn ping(&self) -> bool {
        let mut base_url = self.config.base_url.clone();
        base_url.path_mut().unwrap().push("getLicense.view".to_string());
        let request = self.client.get(base_url);
        if let Ok(mut response) = request.send() {
            use responses::PingResponse;
            let mut body = String::new();
            if let Ok(_) = response.read_to_string(&mut body) { // .ok().and_then(|_| serde_json::from_str(&body).ok()).map(|response: PingResponse| response.status == "ok").unwrap_or(false)
                println!("{}", body);
                match serde_json::from_str(&body) {
                    Ok(PingResponse { status, .. }) => {
                        println!("Status {}", status);
                        true
                    },
                    Err(e) => { println!("err: {}", e); false }
                }
            } else {
                false
            }
        }
        else {
            false
        }
    }
}



fn main() {
    println!("{:?}", serde_json::to_string(&Foo::Bar { x: 1, y: 2 }));
    println!("{:?}", serde_json::to_string(&Foo::Qux("foo".to_string(), "bar".to_string()))); return;
    let u_p  = std::env::args().skip(1).take(2).collect::<Vec<_>>();
    let u = u_p[0].clone();
    let p = u_p[1].clone();
    let config = Config::new(&u, &p, "neferty.me", None).unwrap();
    let client = SubsonicClient::new(config);
    println!("Ping: {}", client.ping());
}


/* Playing stuff
use std::fs::File;
use std::path::Path;

use simplemad::Decoder;
use cpal::{Voice, SamplesRate, Buffer};


fn main() {




    let path = Path::new("foo.mp3");
    let file = File::open(&path).expect("can't open file");
    let mut decoder = Decoder::new(file);
    let mut voice = Voice::new();

    for frame in decoder {
        match frame {
            Ok(frame) => {
                let ref left  = frame.samples[0];
                let ref right = frame.samples[1];
                let total_frame_samples = left.len();
                let mut played_samples = 0;
                assert!(left.len() == right.len());
                // println!("XXX: {}", total_frame_samples);

                while played_samples < total_frame_samples {
                    // println!("PlayedSamples: {}", played_samples);
                    let max_buffer = total_frame_samples - played_samples;
                    let mut buffer: cpal::Buffer<f32> = voice.append_data(1, SamplesRate(frame.sample_rate as u32), max_buffer);
                    let mapped = left.iter().map(|x| (*x as f32) / (std::i32::MAX as f32)).collect::<Vec<_>>();

                    played_samples += buffer.clone_from_slice(&mapped);
                }
                voice.play();

                // println!("Frame sample rate: {}", frame.sample_rate);
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    println!("Hello, world!");
}
*/
