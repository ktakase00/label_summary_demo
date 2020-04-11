use label_summary_generic::run;
use label_summary_generic::LshClient;

fn main() {
    let lsh_client = LshClient::new();
    match run(&lsh_client) {
        Err(err) => { println!("{:#?}", err) },
        _ => ()
    }
}
