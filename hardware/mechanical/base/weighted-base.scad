include <../config.scad>

base_width = 520;
base_depth = 420;
base_height = 120;

difference() {
    cube([base_width, base_depth, base_height], center=true);
    translate([0, 0, -20])
        cube([base_width - 80, base_depth - 80, base_height], center=true);
}
