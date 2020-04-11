use reqwest::blocking::Client;
use label_summary::run;

fn main() {
    let client = Client::new();
    match run(&client) {
        Err(err) => { println!("{:#?}", err) },
        _ => ()
    }
}
