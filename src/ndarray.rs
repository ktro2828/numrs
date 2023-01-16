pub mod dtype;
use dtype::dtype::Dtype;
use rand::Rng;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug)]
pub struct NdArray<T> {
    data: Vec<T>,
    pub size: usize,
    pub shape: (usize, usize),
}

impl<T: Dtype> NdArray<T> {
    // A::new()
    pub fn new(v: Vec<T>) -> Self {
        let d = v.clone();
        NdArray {
            data: d,
            size: v.len(),
            shape: (1, v.len()),
        }
    }

    pub fn min(&self) -> T {
        let mut ret: T = self.data[0];
        for i in 1..self.len() {
            if ret > self.data[i] {
                ret = self.data[i];
            }
        }
        return ret;
    }

    pub fn max(&self) -> T {
        let mut ret: T = self.data[0];
        for i in 1..self.len() {
            if ret < self.data[i] {
                ret = self.data[i];
            }
        }
        return ret;
    }

    pub fn sum(&self) -> T {
        let mut ret = self[0];
        for i in 1..self.len() {
            ret += self[i];
        }
        return ret;
    }

    pub fn dot(&self, other: &NdArray<T>) -> T {
        if self.len() == other.len() {
            let mut ret = self[0] * other[0];
            for i in 1..self.len() {
                ret += self[i] * other[i];
            }
            return ret;
        } else {
            panic!("uncomfortable NdArray.");
        }
    }

    pub fn arange(start: T, stop: T, step: T) -> NdArray<T> {
        let mut data: Vec<T> = Vec::new();
        let mut i = start;
        while i < stop {
            data.push(i);
            i += step;
        }
        return NdArray::new(data);
    }
}

impl<T> NdArray<T> {
    // A.len()
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // A.shape()
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    // A.size()
    pub fn size(&self) -> usize {
        self.size
    }
}

impl NdArray<f64> {
    pub fn zeros(size: usize) -> Self {
        NdArray {
            data: vec![0.0; size],
            size: size,
            shape: (1, size),
        }
    }

    pub fn ones(size: usize) -> Self {
        NdArray {
            data: vec![1.0; size],
            size: size,
            shape: (1, size),
        }
    }

    pub fn random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<f64> = (0..size).map(|_| rng.gen::<f64>()).collect();
        NdArray {
            data,
            size,
            shape: (1, size),
        }
    }
}

// A[i]
impl<T> Index<usize> for NdArray<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}

// A[i] = a
impl<T> IndexMut<usize> for NdArray<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        return &mut self.data[idx];
    }
}

// A.clone()
impl<T: Dtype> Clone for NdArray<T> {
    fn clone(&self) -> NdArray<T> {
        NdArray::<T> {
            data: self.data.clone(),
            size: self.size,
            shape: self.shape,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data = source.data.clone();
    }
}

// A + B
impl<T: Dtype> Add<NdArray<T>> for NdArray<T> {
    type Output = NdArray<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            let mut ret = NdArray::<T>::new(self.data.clone());
            for i in 0..self.len() {
                ret[i] = self[i] + rhs[i];
            }
            return ret;
        } else {
            panic!("uncomfortable NdArray.");
        }
    }
}

// A - B
impl<T: Dtype> Sub<NdArray<T>> for NdArray<T> {
    type Output = NdArray<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            let mut ret = NdArray::<T>::new(self.data.clone());
            for i in 0..self.len() {
                ret[i] = self[i] - rhs[i];
            }
            return ret;
        } else {
            panic!("uncomfortable NdArray.");
        }
    }
}

// A * B
impl<T: Dtype> Mul<NdArray<T>> for NdArray<T> {
    type Output = NdArray<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            let mut ret = NdArray::<T>::new(self.data.clone());
            for i in 0..self.len() {
                ret[i] = self[i] * rhs[i];
            }
            return ret;
        } else {
            panic!("uncomfortable NdArray.");
        }
    }
}

// A * b
impl<T: Dtype> Mul<T> for NdArray<T> {
    type Output = NdArray<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] = self[i] * rhs;
        }
        return ret;
    }
}

// -A
impl<T: Dtype + Neg<Output = T>> Neg for NdArray<T> {
    type Output = NdArray<T>;

    fn neg(self) -> Self::Output {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret.data[i] = -self.data[i];
        }
        return ret;
    }
}

impl<T: Dtype> PartialEq<NdArray<T>> for NdArray<T> {
    // A == B
    fn eq(&self, rhs: &NdArray<T>) -> bool {
        self.data == rhs.data
    }

    // A != B
    fn ne(&self, rhs: &NdArray<T>) -> bool {
        self.data != rhs.data
    }
}
