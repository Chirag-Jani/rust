use anyhow::{ Context, Result };
use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path.clone()),
    // };

    let args = Cli::parse();
    // println!("pattern: \n{:?}, \npath: \n{:?}", args.pattern, args.path);

    let file_content = std::fs
        ::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // println!("\nFile Content: \n{:?}", file_content);

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("Pattern found in line, \n{:?}", line);
        }
    }

    Ok(())
}

// demo for progressbar
// fn main() {
//     let pb = indicatif::ProgressBar::new(99999);
//     for i in 0..99999 {
//         do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");
// }
// fn do_hard_work() {}
