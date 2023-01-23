extern crate numrs;
use numrs::ndarray::NdArray;

#[test]
fn test_add() {
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let arr2 = NdArray::new(&vec![1, 2, 3]);
    let ret = arr1 + arr2;
    assert_eq!(ret[0], 2);
    assert_eq!(ret[1], 4);
    assert_eq!(ret[2], 6);
}

#[test]
fn test_sub() {
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let arr2 = NdArray::new(&vec![1, 2, 3]);
    let ret = arr1 - arr2;
    assert_eq!(ret[0], 0);
    assert_eq!(ret[1], 0);
    assert_eq!(ret[2], 0);
}

#[test]
fn test_mul() {
    // A * B
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let arr2 = NdArray::new(&vec![1, 2, 3]);
    let ret = arr1 * arr2;
    assert_eq!(ret[0], 1);
    assert_eq!(ret[1], 4);
    assert_eq!(ret[2], 9);

    // A * b
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let ret = arr1 * 2;
    assert_eq!(ret[0], 2);
    assert_eq!(ret[1], 4);
    assert_eq!(ret[2], 6);
}

#[test]
fn test_neg() {
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let ret = -arr1;
    assert_eq!(ret[0], -1);
    assert_eq!(ret[1], -2);
    assert_eq!(ret[2], -3);
}

#[test]
fn test_eq() {
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let arr2 = NdArray::new(&vec![1, 2, 3]);
    let arr3 = NdArray::new(&vec![2, 2, 2]);
    assert_eq!(arr1 == arr2, true);
    assert_eq!(arr1 != arr3, true);
}

#[test]
fn test_zeros() {
    let ret = NdArray::zeros(3);
    assert_eq!(ret[0], 0.0);
    assert_eq!(ret[1], 0.0);
    assert_eq!(ret[2], 0.0);
}

#[test]
fn test_ones() {
    let ret = NdArray::ones(3);
    assert_eq!(ret[0], 1.0);
    assert_eq!(ret[1], 1.0);
    assert_eq!(ret[2], 1.0);
}

#[test]
fn test_min() {
    let arr = NdArray::new(&vec![1, 2, 3]);
    let ret = arr.min();
    assert_eq!(ret, 1);
}

#[test]
fn test_max() {
    let arr = NdArray::new(&vec![1, 2, 3]);
    let ret = arr.max();
    assert_eq!(ret, 3);
}

#[test]
fn test_sum() {
    let arr = NdArray::new(&vec![1, 2, 3]);
    let ret = arr.sum();
    assert_eq!(ret, 6);
}

#[test]
fn test_dot() {
    let arr1 = NdArray::new(&vec![1, 2, 3]);
    let arr2 = NdArray::new(&vec![1, 2, 3]);
    let ret = arr1.dot(&arr2);
    assert_eq!(ret, 14);
}

#[test]
fn test_arange() {
    let ret = NdArray::arange(1, 4, 2);
    assert_eq!(ret[0], 1);
    assert_eq!(ret[1], 3);
}
