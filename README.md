# numrs

A toy project of numerical computation library for Rust inspired by Numpy.

## Support

- [x] : 1D array

```rust
let arr = NdArray::new(&vec![1, 2, 3, 4]); // [1, 2, 3, 4]
```

- [x] : `+`, `-`, `*`, `==`, `!=` operators

```rust
let arr1 = NdArray::new(&vec![1, 2, 3, 4]);
let arr2 = NdArray::new(&vec![1, 2, 3, 4]);
let arr3 = NdArray::new(&vec![0, 0, 0, 0]);

let add = arr1 + arr2; // [2, 4, 6, 8]
let sub = arr1 - arr2; // [0, 0, 0, 0]
let mul = arr1 * arr2; // [1, 4, 6, 8]
let neg = -arr1; // [-1, -2, -3, -4]
let eq = arr1 == arr2; // true
let ne = arr1 != arr3; // true
```

- [x] : `min`, `max`, `sum` methods

```rust
let arr = NdArray::new(&vec![1, 2, 3, 4]);
let min = arr.min(); // 1
let max = arr.max(); // 4
let sum = arr.sum(); // 10
```

- [x] : `dot` method

```rust
let arr1 = NdArray::new(&vec![1, 2, 3]);
let arr2 = NdArray::new(&vec![1, 2, 3]);
let dot = arr1.dot(&arr2); // 14
```

- [x] : `zeros`, `ones`, `arange`, `random`, functions

```rust
let zeros = NdArray::zeros(4); // [0.0, 0.0, 0.0, 0.0]
let ones = NdArray::ones(4); // [1.0, 1.0, 1.0, 1.0]
let arange = NdArray::arange(1, 4, 2); // [1, 3]
let random = NdArray::random(4);
```

- [x] : `linalg::norm` function

```rust
let arr = NdArray::new(&vec![1, 1, 1]);
let norm = linalg::norm(&arr);
```

## TODO

- [ ] : 2D or ND array
- [ ] : reshape, transpose, T methods
- [ ] : boolean, string elements
- [ ] : update linalg
