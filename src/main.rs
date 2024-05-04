use clap::{Arg, Command as ClapCommand};
use std::process::Command;
use std::process::Stdio;

fn main() {
    let matches = ClapCommand::new("rust_cli_tool")
        .version("0.1.0")
        .author("Callaghan <hattinghcallaghan@gmail.com>")
        .about("Processes a variable length input string into piper.")
        .arg(
            Arg::new("input")
                .help("Input string to process")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Retrieve the input string
    let input = matches.value_of("input").unwrap();
    // println!("input: {}", input);
    // println!(
    //     "{}",
    //     format!(
    //         "echo \"{}\" |\
    //          .piper/piper --model ./.piper/en_GB-jenny_dioco-medium.onnx --output-raw |\
    //           aplay -r 22050 -f S16_LE -t raw -",
    //         input
    //     )
    // );

    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo \"{}\" |\
             .piper/piper --model ./.piper/en_GB-jenny_dioco-medium.onnx --output-raw |\
              aplay -r 22050 -f S16_LE -t raw -",
            input
        ))
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to execute command");

    let _ = cmd.wait().expect("failed to wait on child");
}
