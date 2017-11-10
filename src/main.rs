
#[macro_use]
extern crate clap;
extern crate image;

use std::path::Path;

use clap::App; 
 
fn main() {

    let config = load_yaml!("args.yml");
    let matches = App::from_yaml(config).get_matches();

    // This should never not be present, but it doesn't hurt to check.
    if matches.is_present("INPUT") {

        let file_path = matches.value_of("INPUT").unwrap().to_string();
        let safe_path = Path::new(&file_path);

        // Removes the extension from the file path if there is one.
        // I swear there's probably a better way of doing this
        // but after an hour or so of searching docs this is all I can
        // come up with. *shrugs*
        let ext_index = file_path.rfind(".");
        let mut path_no_ext = file_path.to_string();

        if ext_index.is_some() { path_no_ext.truncate(ext_index.unwrap()) }

        let default_output = &format!("{}_COMPROMISE.png", path_no_ext);
        let output_path = matches.value_of("OUTPUT").unwrap_or(default_output);

        // TODO: Probably a better way of silencing prints but I'm lazy right now.
        // Possibly improve in the future.
        let not_silent = !matches.is_present("silent");

        // Make sure the file actually exists so we don't crash.
        if safe_path.exists() {

            let mut img = image::open(safe_path).unwrap().to_rgba();

            let pixel = img.get_pixel(1, 1).data;

            // This of course doesn't check the *entire* image for transparency,
            // but it's a quick way of telling. We don't want setting a pixel's
            // alpha to 254 if it's anything but 255. Maybe improve in the future?
            if pixel[3] < 255 {
                if not_silent { println!("This image already has transparency, nothing needs to be done.") }
                return
            }

            if not_silent { println!("Fixing image ...") }
 
            img.get_pixel_mut(1, 1).data = [pixel[0], pixel[1], pixel[2], 254];
            img.save(output_path).unwrap();

            if not_silent { println!("Done! Saved to {}", output_path) }

        } else {
            if not_silent { eprintln!("Could not open file {}", file_path) }
        }

    }

}
