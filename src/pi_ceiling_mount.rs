use scad_generator::*;

pub fn mount() -> ScadObject {
    let x_size = 64.;
    let y_size = 10.;
    let z_size = 3.5;
    
    let hole_x_distance = 58.;
    let hole_y_edge_distance = 2.5;
    let hole_diameter = 3.2;

    let holes = [-0.5 as f32,0.5 as f32].iter()
        .map(|x_pos| scad!(Translate(vec3(hole_x_distance * x_pos, hole_y_edge_distance, 0.)); {
            scad!(Cylinder(z_size, Diameter(hole_diameter)))
        }))
        .fold(scad!(Union), |mut union, object| {
            union.add_child(object);
            union
        });

    let main_body = centered_cube(vec3(x_size, y_size, z_size), (true,false,false));

    let slit_width = 1.7;
    let padding = 3.;
    let mount_y_size = padding * 2. + slit_width;

    let ceiling_mount = {
        let slit_length = 12.5;

        let mount_z_size = slit_length + z_size;

        let body = centered_cube(vec3(x_size, mount_y_size, mount_z_size), (true,true,false));
        let slit = centered_cube(vec3(x_size, slit_width, mount_z_size), (true,true,false));

        scad!(Difference; {
            body,
            scad!(Translate(vec3(0., 0., z_size)); slit)
        })
    };

    scad!(Difference; {
        scad!(Union; {
            main_body,
            scad!(Translate(vec3(0., y_size + mount_y_size / 2., 0.)); ceiling_mount)
        }),
        holes
    })
}
