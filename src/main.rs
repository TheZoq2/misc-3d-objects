use scad::*;

use std::string::String;

//mod mic_mount;
mod emax_box_divider;
mod pi_ceiling_mount;
mod pitaya_mount;

fn main() 
{
    let mut sfile = ScadFile::new();
    sfile.set_detail(25);

    //sfile.add_object(mic_mount::mic_headphone_mount());
    //sfile.add_object(emax_box_divider::divider());
    sfile.add_object(pi_ceiling_mount::mount());
    sfile.add_object(pitaya_mount::pitaya_mount());
    sfile.write_to_file(String::from("output.scad"));
}
