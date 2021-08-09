pub fn is_image(str: &str) -> bool {
    match str {
        "png" => true,
        _ => false,
    }
}

pub fn is_font(str: &str) -> bool {
    match str {
        "ttf" => true,
        _ => false,
    }
}

pub fn is_common(str: &str) -> bool {
    if is_image(str) || is_font(str) {
        false
    } else {
        true
    }
}
