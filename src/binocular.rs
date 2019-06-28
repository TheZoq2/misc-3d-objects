use scad::*;

fn flexible_strap(length: f32, height: f32, segments: i32, thickness: f32, extra_length: f32) -> ScadObject {
    let outer = {
        let shape = centered_cube(vec3(length + extra_length, height, thickness), (false, true, false));

        scad!(Translate(vec3(-extra_length/2., 0., 0.)); shape)
    };

    let mut inner = scad!(Union);

    let separation = length / segments as f32;

    let inner_width= 2.;

    for segment in 0..segments {
        let object = scad!(Union; {
            scad!(Cylinder(thickness, Diameter(inner_width))),
            centered_cube(vec3(inner_width, height, thickness), (true, false, false))
        });

        let x_offset = separation * segment as f32 + inner_width;
        let y_offset = - height/2. + inner_width;
        let translated = scad!(Translate(vec3(x_offset, y_offset, 0.)); {
            object
        });

        let mirrored = scad!(Mirror(vec3(0., (segment % 2) as f32, 0.)); translated);

        inner.add_child(mirrored);
    }

    scad!(Difference; {
        outer,
        inner
    })
}

// fn flexible_strap

fn cup(outer_diameter: f32, inner_diameter: f32, outer_height: f32, inner_height: f32) -> ScadObject {
    scad!(Difference; {
        scad!(Cylinder(outer_height, Diameter(outer_diameter))),
        scad!(Cylinder(inner_height, Diameter(inner_diameter)))
    })
}


pub fn assembly() -> ScadObject {
    let inter_distance = 19.;
    let flex_length = 16.;
    let extra_length = 23.;
    let outer_diameter = 41.;
    let inner_diameter = outer_diameter - 4.;
    let inner_height = 10.;
    let outer_height = inner_height + 2.;
    let connect_y_offset = 12.;

    let cup_ = cup(outer_diameter, inner_diameter, outer_height, inner_height);

    let full_distance = outer_diameter / 2. + inter_distance / 2.;

    let ltrans = |obj| scad!(Translate(vec3(-full_distance, 0., 0.)); obj);
    let rtrans = |obj| scad!(Translate(vec3(full_distance, 0., 0.)); obj);
    let l = ltrans(cup_.clone());
    let r = rtrans(cup_);

    let connect = {
        let strap_shape = flexible_strap(flex_length, 8., 5, 2., extra_length);

        scad!(Difference; {
            scad!(Translate(vec3(-flex_length / 2., connect_y_offset, 0.)); {
                strap_shape,
            }),
            ltrans(scad!(Cylinder(10., Diameter(outer_diameter)))),
            rtrans(scad!(Cylinder(10., Diameter(outer_diameter))))
        })
    };

    let screwholes = {
        let shape = scad!(Cylinder(10., Diameter(3.)));

        scad!(Union; {
            scad!(Translate(vec3(inter_distance / 2. + 1., connect_y_offset + 1., 0.)); shape.clone()),
            scad!(Translate(vec3(-(inter_distance / 2. + 1.), connect_y_offset + 1., 0.)); shape)
        })
    };

    scad!(Difference; {
        connect,
        // l,
        // r,
        screwholes
    })
}

