include <../config.scad>

module gimbal_ring(inner=25, outer=32, thickness=4) {
    difference() {
        cylinder(h=thickness, d=outer * 2, center=true);
        cylinder(h=thickness + 1, d=inner * 2, center=true);
    }
}

gimbal_ring();
