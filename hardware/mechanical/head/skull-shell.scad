include <../config.scad>

difference() {
    scale([scale_major, scale_major, scale_major])
        minkowski() {
            cube([head_width_mm - 20, head_depth_mm - 20, head_height_mm - 20], center=true);
            sphere(r=10);
        }

    // Placeholder internal opening for the mechanical gimbal.
    cube([head_width_mm - 35, head_depth_mm - 35, head_height_mm - 15], center=true);

    // Optical window placeholders.
    for (x = [-40, 40])
        translate([x, -head_depth_mm/2 - 1, 25])
            cube([eye_display_size_mm, 12, eye_display_size_mm], center=true);
}
