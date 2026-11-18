// Shared units: millimeters
$fn = 36;

scale_major = 1.0;
build_plate_mm = 220;
head_width_mm = 160;
head_height_mm = 210;
head_depth_mm = 165;
bearing_clearance_mm = 0.25;
bowden_outer_diameter_mm = 4.0;
eye_display_size_mm = 36;

// Safety and thermal values are documentation gates, not fabrication values.
neck_continuous_torque_nm = 5.0;
neck_factor_of_safety_dynamic = 3.0;
neck_factor_of_safety_static = 5.0;

echo(str("Robot design scale: ", scale_major));
