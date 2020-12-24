//This example generates thumbnails for all .jpg files in the current directory then saves them in a new folder called thumbnails.
//glob::glob_with finds jpeg files in current directory. rayon resizes images in parallel using par_iter calling DynamicImage::resize.

#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate image;
extern crate rayon;

use std::path::Path;
use std::fs::create_dir_all;

use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};
use rayon::prelude::*;

error_chain! {
    foreign_links {
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", &options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}

/*
cargo run -p para
    Updating crates.io index
   Compiling glob v0.3.0
   Compiling num-rational v0.2.2
   Compiling image v0.23.0-preview.0
   Compiling para v0.1.0 (/home/jay/b/wk/rust/books/learningrustcookbook/code/concurrency/para)
error[E0432]: unresolved import `image::FilterType`
  --> para/src/main.rs:15:13
   |
15 | use image::{FilterType, ImageError};
   |             ^^^^^^^^^^ no `FilterType` in the root

error[E0308]: mismatched types
  --> para/src/main.rs:28:44
   |
28 |     let files: Vec<_> = glob_with("*.jpg", &options)?
   |                                            ^^^^^^^^
   |                                            |
   |                                            expected struct `glob::MatchOptions`, found reference
   |                                            help: consider removing the borrow: `options`
   |
   = note: expected type `glob::MatchOptions`
              found type `&glob::MatchOptions`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0432.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `para`.
*/
