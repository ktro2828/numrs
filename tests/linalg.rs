extern crate numrs;
use numrs::linalg;
use numrs::ndarray::NdArray;

#[test]
fn test_norm() {
    let arr = NdArray::new(vec![1, 1, 1]);
    let ret = linalg::norm(&arr);
    assert_eq!(ret, 3);
}
