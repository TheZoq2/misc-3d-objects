#[macro_use]
extern crate scad_generator;
extern crate scad_util;

use scad_generator::*;

use std::string::String;

//mod mic_mount;
mod emax_box_divider;

fn main() 
{
    let mut sfile = ScadFile::new();

    //sfile.add_object(mic_mount::mic_headphone_mount());
    sfile.add_object(emax_box_divider::divider());
    sfile.write_to_file(String::from("output.scad"));
}
