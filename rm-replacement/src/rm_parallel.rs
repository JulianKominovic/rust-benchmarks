use glob::glob;
use rayon::prelude::*;
pub fn rm_parallel() {
    // Args
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    let glob_pattern_opt = match args.get(1) {
        Some(pattern) => Some(pattern),
        _ => None,
    };
    if glob_pattern_opt.is_none() {
        println!("Please provide a glob pattern");
        std::process::exit(1);
    }
    let glob_pattern = glob_pattern_opt.unwrap();

    let glob_files_intent = glob(glob_pattern);
    if glob_files_intent.is_err() {
        println!("Error: {:?}", glob_files_intent.err());
        std::process::exit(1);
    }
    glob_files_intent
        .unwrap()
        .par_bridge()
        .into_par_iter()
        .for_each(|pathbuf_intent| {
            if pathbuf_intent.as_ref().ok().is_some() {
                // println!("Deleting: {:?}", pathbuf_intent);
                let pathbuf = pathbuf_intent.unwrap();
                let pathbuf_str = pathbuf.to_str().unwrap();
                let remove_intent = std::fs::remove_file(pathbuf_str);
                if remove_intent.is_err() {
                    println!("Error: {:?}", remove_intent.err());
                }
            } else {
                println!("Error: {:?}", pathbuf_intent.err());
            }
        });
}
