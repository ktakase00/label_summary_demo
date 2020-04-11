use label_summary_mock::run;
use label_summary_mock::LshClient;

fn main() {
    let lsh_client = LshClient::new();
    match run(&lsh_client) {
        Err(err) => { println!("{:#?}", err) },
        _ => ()
    }
}
