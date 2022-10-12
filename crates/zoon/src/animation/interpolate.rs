use num_traits::cast::AsPrimitive;

pub fn linear_with_bounds<T: Into<f64> + Copy + 'static>(
    (x1, y1): (impl Into<f64>, impl Into<f64>),
    (x2, y2): (impl Into<f64>, impl Into<f64>),
) -> impl Fn(T) -> T
where
    f64: AsPrimitive<T>,
{
    let (x1, y1, x2, y2) = (x1.into(), y1.into(), x2.into(), y2.into());
    assert_ne!(x1, x2, "linear function expects different x values");
    // y = kx + q
    let k = (y2 - y1) / (x2 - x1);
    let q = y1 - k * x1;
    move |x| {
        let x = x.into().clamp(x1, x2);
        let y = k * x + q;
        y.as_()
    }
}

pub fn linear<T: Into<f64> + Copy + 'static>(
    when_zero: impl Into<f64>,
    when_one: impl Into<f64>,
) -> impl Fn(T) -> T
where
    f64: AsPrimitive<T>,
{
    linear_with_bounds((0., when_zero), (1., when_one))
}
