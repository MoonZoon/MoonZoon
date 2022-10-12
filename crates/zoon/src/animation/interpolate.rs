use easy_cast::*;

pub fn linear_with_bounds<T: Into<f64> + Conv<f64>>(
    (x1, y1): (impl Into<f64>, impl Into<f64>),
    (x2, y2): (impl Into<f64>, impl Into<f64>),
) -> impl Fn(T) -> T {
    let (x1, y1, x2, y2) = (x1.into(), y1.into(), x2.into(), y2.into());
    assert_ne!(x1, x2, "linear function expects different x values");
    // y = kx + q
    let k = (y2 - y1) / (x2 - x1); // @TODO 0
    let q = y1 - k * x1;
    move |x| {
        let x = x.into().clamp(x1, x2);
        let y = k * x + q;
        y.cast()
    }
}

pub fn linear<T: Into<f64> + Conv<f64>>(
    when_zero: impl Into<f64>,
    when_one: impl Into<f64>,
) -> impl Fn(T) -> T {
    linear_with_bounds((0., when_zero), (1., when_one))
}
