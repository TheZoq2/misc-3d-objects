#![allow(non_upper_case_globals)]

use scad_generator::*;


const connector_diameter: f32 = 8.5;
const connector_screw_diameter: f32 = 7.4;
const connector_screw_height: f32 = 2.5;
const connector_height: f32 = 6.;
const connector_outer_diameter: f32 = 12.;

pub fn mic_headphone_mount() -> ScadObject
{
    scad!(Union;
    {
        scad!(Translate(vec3(-connector_height / 2., -(headset_mount_outer_diameter / 2. + connector_diameter / 2.), connector_outer_diameter / 2.));
        {
            scad!(Rotate(90., vec3(0., 1., 0.));
            {
                connector_mount(),
            }),
        }),
        headset_mount()
    })
}

fn connector_mount() -> ScadObject
{
    let total_height = connector_screw_height + connector_height;
    let outer_shell = scad!(Cylinder(total_height, Diameter(connector_outer_diameter)));

    let screw_cutout = {
        let base = scad!(Cylinder(connector_screw_height, Diameter(connector_screw_diameter)));
        scad!(Translate(vec3(0., 0., connector_height));base)
    };

    let main_cutout = scad!(Cylinder(connector_height, Diameter(connector_diameter)));

    scad!(Difference;
    {
        outer_shell,
        screw_cutout,
        main_cutout
    })
}

const headset_diameter: f32 = 11.;
const headset_mount_outer_diameter: f32 = 16.;
const headset_mount_height: f32 = 11.;

fn headset_mount() -> ScadObject
{
    let main = scad!(Cylinder(headset_mount_height, Diameter(headset_mount_outer_diameter)));

    let hole_cutout = scad!(Cylinder(headset_mount_height, Diameter(headset_diameter)));
    let triangle_coutout = scad!(Cube(vec3(headset_diameter, headset_diameter, headset_mount_height)));

    scad!(Difference;
    {
        main,
        hole_cutout,
        triangle_coutout
    })
}

