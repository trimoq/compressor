extern crate image;
use image::{GenericImageView,DynamicImage,FilterType};

use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use std::fs;

/**
Struct to hold the information necessary to independently compress an image
*/
pub struct CompressionSpec{
    pub path: PathBuf,
    pub target_path: PathBuf,
    pub scale: Scale,
    pub quality: Quality
}

/**
Enum to specify the scale type and factor(s)
Either scale the image with a ratio (e.g. 0.2) or scale to a fixed dimension (e.g. 100x100)
*/
#[derive(Copy, Clone)]
pub enum Scale{
    Ratio(f32),
    Dimension(u32,u32)
}

/**
Enum to chose the compression quality (and thereby the speed)
*/
#[derive(Copy, Clone)]
pub enum Quality{
    Fastest,
    Best,
}

/**
Compress all passed specs and return the number of successfully saved images
*/
pub fn compress_specs(specs : Vec<CompressionSpec>)->u32{
    specs.iter().map(|spec|compress(spec)).sum()
}

/**
Generate a CompressionSpec based on the provided attributes.
This clones the Quality and Scale attributes but only borrows the path string
*/
pub fn get_spec(quality: Quality, scale: Scale, source: PathBuf, target_path: PathBuf) -> CompressionSpec {
    dbg!(format!("Using file: {:?}", source));
    println!("Saving to {:?}", target_path);
    let scale = scale.clone();
    let quality = quality.clone();
    CompressionSpec {
        path:source,
        target_path,
        scale,
        quality
    }
}

/**
Compress an image according to its compression spec
*/
fn compress( spec: &CompressionSpec)->u32{
    //TODO return a result type and map OK/ERR to numbers on the caller

    // Create the target dir if it does not exist yet
    if !spec.target_path.exists(){
        let dir_res = fs::create_dir(spec.target_path.clone());
        if dir_res.is_err(){
            println!("Error: {:?}",dir_res);
        }
    }
    else{
        if !spec.target_path.is_dir(){
            println!("Error: target path is not a directory");
        }
    }

    // create the final file name
    let target = spec.target_path.join(spec.path.file_name().unwrap_or(OsStr::new("compressor_default.jpg")));
    //read input image
    println!("Compressing {} to {}",spec.path.to_str().unwrap_or("Unknown file"),target.to_str().unwrap_or("Unknown target"));
    let img = image::open(&spec.path);
    let filter_type = match spec.quality {
        Quality::Best => FilterType::Gaussian,
        Quality::Fastest => FilterType::Nearest,
    };

    // if the image was loaded correctly compress depending on the type of scale and save
    if let Ok(img) = img {
        match spec.scale{
            Scale::Ratio(ratio) => {
                let mut dims =  img.dimensions();
                dims.0 = (dims.0 as f32 * ratio) as u32;
                dims.1 = (dims.1 as f32 * ratio) as u32;
                let result = img.resize(dims.0 ,dims.1 ,filter_type);
                return save(&target, result);
            },
            Scale::Dimension(x,y) => {
                let result = img.resize(x ,y ,filter_type);
                return save(&target, result)
            }
        }
    }
    else{
        // in the case of an error, do not panic but notify the user about the problem and return a zero for easy counting of results
        println!("Could not read image: {:?}",target);
        return 0;
    }

}

/**
Save an image to the specified location and hanlde errors
*/
fn save(target: &PathBuf, result: DynamicImage) -> u32{
    //TODO return a result type and map OK/ERR to numbers on the caller
    let save_res = result.save(target);
    if let Err(e) = save_res{
        println!("Error saving image to {}: {:?}",target.to_str().unwrap_or("Unknown file"),e);
        return 0;
    }
    return 1;
}


/**
Find all jpegs in the provided path without descending into child directories.
Found jpegs are returned as a vector of their PathBufs.
This Vector may be empty.
*/
pub fn find_all_jpegs(dir: &Path) -> Vec<PathBuf> {

    let mut result_vec = vec![];

    // we only traverse directories
    if !dir.is_dir() {
        println!("Directory {:?} is no directory, abort.",dir);
        return result_vec;
    }

    // only process valid directories
    if let Ok(files) = fs::read_dir(dir) {

        // process all files in directory, not recursive
        for file in files {

            //only process readabe files
            if let Ok(file) = file {

                // only add files with valid, reaqdable names
                if let Ok(name) = file.file_name().into_string(){

                    // only add supported jpeg files
                    if name.ends_with("JPG")
                        || name.ends_with("jpg")
                        || name.ends_with("jpeg"){

                        // add the file to the result vector
                        result_vec.push(file.path());
                        println!("Found {}",file.path().to_str().unwrap_or("Missing filename"));
                    }
                    else{
                        println!("Skipping {} due to wrong file ending.",name);
                    }
                }
                else{
                    println!("Illegal characters in file strin: {}",file.path().to_str().unwrap_or("Missing filename"));
                }
            }
            else{
                println!("Cannot access file: {:?}",file.unwrap_err());
            }
        }
    }
    else{
        println!("Cannot read dir: {:?}",dir);
    }
    return result_vec;
}
