use crate::ndarray::dtype::dtype::Dtype;
use crate::ndarray::NdArray;

pub fn norm<T: Dtype>(arr: &NdArray<T>) -> T {
    let mut ret = arr[0] * arr[0];
    for i in 1..arr.len() {
        ret += arr[i] * arr[i];
    }
    return ret;
}
