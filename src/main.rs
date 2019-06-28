use scad::*;

use std::string::String;

//mod mic_mount;
mod emax_box_divider;
mod pi_ceiling_mount;
mod pitaya_mount;
mod bike_pipe;
mod ssd_mount;
mod binocular;



fn main() 
{
    let mut sfile = ScadFile::new();
    sfile.set_detail(100);

    //sfile.add_object(mic_mount::mic_headphone_mount());
    //sfile.add_object(emax_box_divider::divider());
    // sfile.add_object(pi_ceiling_mount::mount());
    // sfile.add_object(pitaya_mount::pitaya_mount());
    // sfile.add_object(bike_pipe::bike_pipe());
    // sfile.add_object(ssd_mount::ssd_mount());
    sfile.add_object(binocular::assembly());
    sfile.write_to_file(String::from("output.scad"));
}
