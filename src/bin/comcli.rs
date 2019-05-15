
extern crate compressor;
use compressor::{CompressionSpec, Scale, Quality, compress_specs, find_all_jpegs};

extern crate clap;
use clap::{Arg, App, SubCommand, ArgGroup, ArgMatches};
use std::path::Path;

fn main() {
    let matches = create_clap();

    let output_path_string = matches.value_of("output").unwrap_or("compressed");
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


    if matches.is_present("files"){
        let files = matches.values_of("files");
        let files = files.unwrap();
        let specs = files.map(|v| {
            dbg!(format!("Using file: {:?}",v));
            let target_path = Path::new(output_path_string).to_path_buf();
            println!("Saving to {:?}",target_path);
            let path = Path::new(v).to_path_buf();
            let scale = scale.clone();
            let quality = quality.clone();
            CompressionSpec{
                path,
                target_path,
                scale,quality}

        }).collect();
        let num_saved_images = compress_specs(specs);
        println!("Sucessfully saved {} images",num_saved_images);
    }
    else {
        let mut pathString = ".";
        if matches.is_present("input_dir") {
            pathString = matches.value_of("input_dir").unwrap_or(".");
            println!("Using input dir: {}", pathString);
        }
        let paths = find_all_jpegs(Path::new(&pathString));

    }
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

fn parse_scale(matches: &ArgMatches) -> Scale {
    let scale = match matches.is_present("ratio") {
        true => {
            let ratio = matches.value_of("ratio").unwrap_or("0.1");
            let ratio = ratio.parse::<f32>().unwrap_or(0.1);
            println!("Scale ratio: {}", ratio);

            Scale::Ratio(ratio)
        }
        _ => {
            let dim = matches.value_of("dim").unwrap_or("100x100");
            let dims: Vec<u32> = dim.split("x").map(|i| i.parse::<u32>().unwrap_or(1)).collect();
            if dims.len() != 2 {
                panic!("Abort: Did not specify valid dimension, use e.g. 100x100");
            }
            println!("Scale x: {},y: {}", dims[1], dims[1]);
            Scale::Dimension(dims[0], dims[1])
        }
    };
    scale
}