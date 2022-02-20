use crate::math::{f32x4, Transform, Vec2, Vec3, Wec2, Wec3};

/// A generic object which contains a property of type T which is sequenced over time.
pub trait Sequenced<T>: Send + Sync {
    fn sample_at(&self, t: f32) -> T;
}

pub trait WSequenced<T>: Send + Sync {
    fn sample_at(&self, t: f32x4) -> T;
}

macro_rules! impl_inherent_sequenced {
    ($($type:ty,)*) => {
        $(impl Sequenced<$type> for $type {
            #[inline]
            fn sample_at(&self, _t: f32) -> Self {
                self.clone()
            }
        })*
    }
}

macro_rules! impl_inherent_wsequenced {
    ($($type:ty,)*) => {
        $(impl WSequenced<$type> for $type {
            #[inline]
            fn sample_at(&self, _t: f32x4) -> Self {
                self.clone()
            }
        })*
    }
}

macro_rules! impl_wsequenced_for_sequenced {
    ($($type:ty => $wtype:ty),*) => {
        $(impl WSequenced<$wtype> for $type {
            #[inline]
            fn sample_at(&self, t: f32x4) -> $wtype {
                let ts = t.as_ref();
                <$wtype>::from([
                    Sequenced::sample_at(self, ts[0]),
                    Sequenced::sample_at(self, ts[1]),
                    Sequenced::sample_at(self, ts[2]),
                    Sequenced::sample_at(self, ts[3]),
                ])
            }
        })*
    }
}

impl_inherent_sequenced!(f32, usize, u32, i32, isize, Vec2, Vec3, Transform,);
impl_inherent_wsequenced!(f32x4, Wec3, Wec2,);
impl_wsequenced_for_sequenced!(f32 => f32x4, Vec2 => Wec2, Vec3 => Wec3);

impl<T, F: Fn(f32) -> T + Send + Sync> Sequenced<T> for F {
    #[inline]
    fn sample_at(&self, t: f32) -> T {
        self(t)
    }
}

impl<F: Fn(f32) -> Vec3 + Send + Sync> WSequenced<Wec3> for F {
    #[inline]
    fn sample_at(&self, t: f32x4) -> Wec3 {
        let ts = t.as_ref();
        [self(ts[0]), self(ts[0]), self(ts[0]), self(ts[0])].into()
    }
}

#[cfg(feature = "minterpolate")]
pub use minterpolate_integration::*;
#[cfg(feature = "minterpolate")]
mod minterpolate_integration {
    use super::*;
    use minterpolate::{InterpolationFunction, InterpolationPrimitive};

    /// A concrete struct which holds a sequence of interpolated values of type T. Basically,
    /// a keyframed animation.
    pub struct Sequence<T: InterpolationPrimitive + Clone + Send + Sync> {
        /// The time at which the corresponding output should be reached.
        inputs: Vec<f32>,
        /// The sampled value at the corresponding input time. Depending on the interpolation function,
        /// there may be multiple outputs required for a single input (for example tangents of a spline).
        outputs: Vec<T>,
        /// How to interpolate between keys
        interpolation: InterpolationFunction<T>,
        /// If the output should be normalized after being interpolated
        /// (useful when interpolating between rotations stored as Quaternions)
        normalize: bool,
    }

    impl<T: InterpolationPrimitive + Clone + Send + Sync> Sequence<T> {
        #[allow(dead_code)]
        pub fn new(
            inputs: Vec<f32>,
            outputs: Vec<T>,
            interpolation: InterpolationFunction<T>,
            normalize: bool,
        ) -> Self {
            Sequence {
                inputs,
                outputs,
                interpolation,
                normalize,
            }
        }

        pub fn sample(&self, t: f32) -> T {
            self.interpolation
                .interpolate(t, &self.inputs, &self.outputs, self.normalize)
        }
    }

    impl<T: InterpolationPrimitive + Clone + Send + Sync> Sequenced<T> for Sequence<T> {
        #[inline]
        fn sample_at(&self, t: f32) -> T {
            self.sample(t)
        }
    }

    impl Sequenced<Vec3> for Sequence<[f32; 3]> {
        #[inline]
        fn sample_at(&self, t: f32) -> Vec3 {
            Vec3::from(self.sample(t))
        }
    }
}

// /// A convenient struct to hold the animation of a single Transform
// pub struct TransformSequence<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> {
//     pos_seq: PS,
//     ori_seq: OS,
// }

// impl<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> TransformSequence<PS, OS> {
//     pub fn new(pos_seq: PS, ori_seq: OS) -> Self {
//         TransformSequence { pos_seq, ori_seq }
//     }
// }

// impl<PS: Sequenced<Vec3>, OS: Sequenced<Quat>> Sequenced<Transform> for TransformSequence<PS, OS> {
//     fn sample_at(&self, t: f32) -> Transform {
//         Transform {
//             position: self.pos_seq.sample_at(t),
//             orientation: self.ori_seq.sample_at(t),
//         }
//     }
// }
