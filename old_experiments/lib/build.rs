extern crate cheddar;

fn main() {
    // This generates the header for our library.
    cheddar::Cheddar::new().expect("could not read manifest")
        .run_build("include/rps_header.h");
}
