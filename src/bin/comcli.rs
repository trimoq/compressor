
extern crate compressor;
use compressor::{Scale, Quality, compress_specs, find_all_jpegs,get_spec};

extern crate clap;
use clap::{Arg, App, ArgGroup, ArgMatches};
use std::path::{Path, PathBuf};

fn main() {
    // get the arguments from the cli
    let matches = create_clap();

    // parse the output path
    let output_path_string = matches.value_of("output").unwrap_or("compressed");

    // parse the quality, default to fastest since nobody wants to wait
    let quality = match matches.value_of("quality") {
        Some(s) => {
            match s {
                "fastest" => Quality::Fastest,
                "best" => Quality::Best,
                _ => Quality::Fastest
            }
        }
        _ => Quality::Fastest
    };

    let scale = parse_scale(&matches);

    // get the files according to the user-desired method, defaults to using the pwd
    if matches.is_present("files"){
        let files = matches.values_of("files");
        let files = files.unwrap();
        let files = files.map(|v|Path::new(v).to_path_buf()).collect();
        compress_files(quality, scale, files, output_path_string);
    }
    else {
        let mut path_string = ".";
        if matches.is_present("input_dir") {
            path_string = matches.value_of("input_dir").unwrap_or(".");
            println!("Using input dir: {}", path_string);
        }
        let paths = find_all_jpegs(Path::new(&path_string));
        compress_files(quality, scale, paths,output_path_string);
    }
}

/**
Compress the passed files according to the parameters
*/
fn compress_files(quality: Quality, scale: Scale, files: Vec<PathBuf>,output_path_string: &str) {
    let specs = files
        .iter()
        .map(|v| get_spec(quality, scale, v.clone(), v.parent().unwrap().join(output_path_string)))
        .collect();
    let num_saved_images = compress_specs(specs);
    println!("Sucessfully saved {} images", num_saved_images);
}


fn create_clap() -> ArgMatches<'static> {
    App::new("Compressor")
        .version("0.1.0")
        .author("Marco A. <marco@amann.dev>")
        .about("Quickly compresses images")
        // the output directory location
        .arg(Arg::with_name("output")
            .short("o")
            .default_value("compressed")
            .value_name("DIRECTORY")
            .required(false)
            .help("Output directory")
        )

        // list of input files or a input dir
        .arg(Arg::with_name("input_dir")
            .short("i")
            .value_name("DIRECTORY")
            .required(false)
            .help("Input directory (overrides files)")
            .conflicts_with("files")
        )
        .arg(Arg::with_name("files")
            .multiple(true)
            .value_name("FILES")
            .required(false)
            .help("Input files")
            .conflicts_with("input_dir")
        )

        // compression quality related things
        .arg(Arg::with_name("best").short("bst"))
        .arg(Arg::with_name("fastest").short("fst"))
        .group(ArgGroup::with_name("quality")
            .args(&["best", "fastest"])
            .required(false)
        )

        // scale ratio or dimension of the putput
        .arg(Arg::with_name("ratio")
            .short("r")
            .value_name("RATIO")
            .required(false)
            .help("Scale ratio, e.g. 0.1")
            .conflicts_with("dim")
        )
        .arg(Arg::with_name("dim")
            .short("d")
            .value_name("DIMENSION")
            .required(false)
            .help("Target dimension e.g. 100x100")
        )
        .get_matches()
}

/**
Parse the scale from the matches
If a dimension is specified, use it, else default to a scale of 0.1
*/
fn parse_scale(matches: &ArgMatches) -> Scale {
    let scale = match matches.is_present("dim") {
        true => {
            let dim = matches.value_of("dim").unwrap_or("100x100");
            let dims: Vec<u32> = dim.split("x").map(|i| i.parse::<u32>().unwrap_or(1)).collect();
            if dims.len() != 2 {
                panic!("Abort: Did not specify valid dimension, use e.g. 100x100");
            }
            println!("Scale x: {},y: {}", dims[1], dims[1]);
            Scale::Dimension(dims[0], dims[1])
        }
        _ => {
            let ratio = matches.value_of("ratio").unwrap_or("0.1");
            let ratio = ratio.parse::<f32>().unwrap_or(0.1);
            println!("Scale ratio: {}", ratio);
            Scale::Ratio(ratio)
        }
    };
    scale
}