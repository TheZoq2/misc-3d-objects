
use scad_generator::*;


pub fn divider() -> ScadObject
{
    let width = 1.;

    let box_x = 87.5;
    let box_y = 62.5;
    let height = 19.;

    let center = centered_cube(vec3(box_x, width, height), (true, true, false));
    let divider = centered_cube(vec3(width, box_y, height), (true, true, false));

    let dividers = [-0.1, 0.2].iter()
            .map(|i| scad!(Translate(vec3(box_x*i, 0., 0.)); divider.clone()))
            .fold(scad!(Union), |mut acc, object|{acc.add_child(object); acc});

    scad!(Union;
    {
        center,
        dividers
    })
}
