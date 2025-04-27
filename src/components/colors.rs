/// Creates a linear interpolation between 2 color strings
pub fn color_lerp(left: &str, right: &str, value: f32) -> String {
    let lerp_val = value.clamp(0.0, 1.0);

    let rl = &left[0..2];
    let rr = &right[0..2];

    let gl = &left[2..4];
    let gr = &right[2..4];

    let bl = &left[4..6];
    let br = &right[4..6];

    let red_value = hex_lerp(rl, rr, lerp_val);
    let green_value = hex_lerp(gl, gr, lerp_val);
    let blue_value = hex_lerp(bl, br, lerp_val);
    format!("#{:02X}{:02X}{:02X}", red_value, green_value, blue_value)

    // let lerped = left_value as f32 + ((right_value as f32 - left_value as f32) * lerp_val);
    // let lerpedu = lerped as u32;
    // let lerpedcol = format!("#{:06X}", lerped as u32);
    // dbg!(&lerpedcol);
    // lerpedcol
}
pub fn hex_lerp(left: &str, right: &str, value: f32) -> u8 {
    let lerp_val = value.clamp(0.0, 1.0);
    let left_value = u8::from_str_radix(left, 16).unwrap();
    let right_value = u8::from_str_radix(right, 16).unwrap();
    let lerped = left_value as f32 + ((right_value as f32 - left_value as f32) * lerp_val);
    lerped as u8
    // let lerpedu = lerped as u8;
    // lerpedu
}
