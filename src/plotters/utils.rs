pub fn get_scale_factors(
    figsize: (f64, f64),
    bbox: (f64, f64, f64, f64),
    aspect: Option<f64>,
) -> ((f64, f64), (f64, f64)) {
    let min_x = bbox.0;
    let max_x = bbox.1;
    let min_y = bbox.2;
    let max_y = bbox.3;

    let span_x = max_x - min_x;
    let span_y = max_y - min_y;

    let mut scale_x = figsize.0 / span_x;
    let mut scale_y = figsize.1 / span_y;

    if let Some(aspect) = aspect {
        if scale_x < scale_y {
            scale_x = scale_x * aspect;
        } else {
            scale_y = scale_y / aspect;
        }
    };

    let shift_x = -1.0 * scale_x * min_x;
    let shift_y = -1.0 * scale_y * min_y;

    ((scale_x, scale_y), (shift_x, shift_y))
}
