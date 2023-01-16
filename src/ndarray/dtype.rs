/// A generic Dtype trait for primitive integers and floating point Dtype
pub mod dtype {
    use num;
    use num::traits::Num;
    use std::ops::AddAssign;
    pub trait Dtype: Num + Clone + Copy + PartialEq + PartialOrd + AddAssign {}

    impl Dtype for f64 {}
    impl Dtype for f32 {}
    impl Dtype for i64 {}
    impl Dtype for i32 {}
    impl Dtype for i16 {}
    impl Dtype for i8 {}
    impl Dtype for u64 {}
    impl Dtype for u32 {}
    impl Dtype for u16 {}
    impl Dtype for u8 {}
    impl Dtype for usize {}
}
