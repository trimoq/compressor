use std::path::{Path,PathBuf};
use std::fs;

extern crate image;
use image::{GenericImageView,DynamicImage,FilterType};
use std::ffi::OsStr;
use std::iter::Filter;


pub mod compressor{
    pub use crate::compress_path;
}

pub fn compress_path(pathString: String){
    /*
    let paths = find_all_files(Path::new(&pathString));
    let target_path = Path::new(&pathString).join(Path::new("compressed"));


    if paths.len()>1{
        compress_path_buf_vec(paths,target_path)
    }
    else {
        println!("Could not find any files in {}",pathString)
    }
    */
}

pub fn compress_path_buf_vec(paths: Vec<PathBuf>, target_path:PathBuf){
    let num_elems = paths.len();
    println!("Compressing {} elemets to {} ",num_elems,target_path.to_str().unwrap_or("Unknown Destrination"));
     let spec_vec = paths
        .iter()
        .map(|path|{
            CompressionSpec{
                path: path.clone(),
                target_path: target_path.clone(),
                scale: Scale::Ratio(0.1f32),
                quality: Quality::Fastest
            }
        })
        .collect();
    let num_saved_images = compress_specs(spec_vec);
    println!("Sucessfully saved {} images",num_saved_images);
}

pub fn compress_specs(specs : Vec<CompressionSpec>)->u32{
    specs.iter().map(|spec|compress(spec)).sum()
}

fn compress( spec: &CompressionSpec)->u32{

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

    let target = spec.target_path.join(spec.path.file_name().unwrap_or(OsStr::new("compressor_default.jpg")));
    println!("Compressing {} to {}",spec.path.to_str().unwrap_or("Unknown file"),target.to_str().unwrap_or("Unknown target"));
    let img = image::open(&spec.path);
    let filter_type = match spec.quality {
        Quality::Best => FilterType::Gaussian,
        Quality::Fastest => FilterType::Nearest,
    };

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
        println!("Could not read image: {:?}",target);
        return 0;
    }

}

fn save(target: &PathBuf, result: DynamicImage) -> u32{
    let save_res = result.save(target);
    if let Err(e) = save_res{
        println!("Error saving image to {}: {:?}",target.to_str().unwrap_or("Unknown file"),e);
        return 0;
    }
    return 1;
}

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


pub struct CompressionSpec{
    pub path: PathBuf,
    pub target_path: PathBuf,
    pub scale: Scale,
    pub quality: Quality
}

#[derive(Copy, Clone)]
pub enum Scale{
    Ratio(f32),
    Dimension(u32,u32)
}

#[derive(Copy, Clone)]
pub enum Quality{
    Fastest,
    Best,
}