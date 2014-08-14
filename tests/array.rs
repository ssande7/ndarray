#![feature(phase)]
#![allow(uppercase_variables)]

#[phase(plugin, link)] extern crate itertools;
extern crate test;
extern crate ndarray;

use ndarray::{Array, C, Slice};
use ndarray::{arr0, arr1, arr2};

#[test]
fn test_matmul_rcarray()
{
    let mut A: Array<uint, (uint, uint)> = Array::zeros((2u, 3u));
    for (i, elt) in A.iter_mut().enumerate() {
        *elt = i;
    }

    let mut B: Array<uint, (uint, uint)> = Array::zeros((3u, 4u));
    for (i, elt) in B.iter_mut().enumerate() {
        *elt = i;
    }

    let c = A.mat_mul(&B);
    println!("A = \n{}", A);
    println!("B = \n{}", B);
    println!("A x B = \n{}", c);
    unsafe {
        let result = Array::from_vec_dim((2u, 4u), vec![20u, 23, 26, 29, 56, 68, 80, 92]);
        assert_eq!(c.shape(), result.shape());
        assert!(c.iter().zip(result.iter()).all(|(a,b)| a == b));
        assert!(c == result);
    }
}

#[test]
fn test_slice()
{
    let mut A: Array<uint, (uint, uint)> = Array::zeros((3u, 4u));
    for (i, elt) in A.iter_mut().enumerate() {
        *elt = i;
    }

    let vi = A.slice([Slice(1, None, 1), Slice(0, None, 2)]);
    assert_eq!(vi.shape(), &[2u, 2u]);
    let vi = A.slice([C, C]);
    assert_eq!(vi.shape(), A.shape());
    assert!(vi.iter().zip(A.iter()).all(|(a, b)| a == b));
}

#[test]
fn test_index()
{
    let mut A: Array<uint, (uint, uint)> = Array::zeros((2u, 3u));
    for (i, elt) in A.iter_mut().enumerate() {
        *elt = i;
    }

    for ((i, j), x) in iproduct!(range(0,2u), range(0,3u)).zip(A.iter()) {
        assert_eq!(*x, A[(i, j)]);
    }

    let vi = A.slice([Slice(1, None, 1), Slice(0, None, 2)]);
    let mut it = vi.iter();
    for (i, j) in iproduct!(range(0, 1u), range(0, 2u)) {
        let x = it.next().unwrap();
        assert_eq!(*x, vi[(i, j)]);
    }
    assert!(it.next().is_none());
}

#[test]
fn test_add()
{
    let mut A: Array<uint, (uint, uint)> = Array::zeros((2u, 2u));
    for (i, elt) in A.iter_mut().enumerate() {
        *elt = i;
    }

    let B = A.clone();
    A.iadd(&B);
    assert_eq!(A[(0,0)], 0u);
    assert_eq!(A[(0,1)], 2u);
    assert_eq!(A[(1,0)], 4u);
    assert_eq!(A[(1,1)], 6u);
}

#[test]
fn test_multidim()
{
    let mut mat = Array::zeros(2u*3*4*5*6).reshape((2u,3u,4u,5u,6u));
    mat[(0,0,0,0,0)] = 22u8;
    {
        for (i, elt) in mat.iter_mut().enumerate() {
            *elt = i as u8;
        }
    }
    //println!("shape={}, strides={}", mat.shape(), mat.strides);
    assert_eq!(mat.shape(), &[2u,3,4,5,6]);
}


/*
array([[[ 7,  6],
        [ 5,  4],
        [ 3,  2],
        [ 1,  0]],

       [[15, 14],
        [13, 12],
        [11, 10],
        [ 9,  8]]])
*/
#[test]
fn test_negative_stride_rcarray()
{
    let mut mat = Array::zeros((2u, 4u, 2u));
    mat[(0, 0, 0)] = 1.0f32;
    for (i, elt) in mat.iter_mut().enumerate() {
        *elt = i as f32;
    }

    {
        let vi = mat.slice([C, Slice(0, None, -1), Slice(0, None, -1)]);
        assert_eq!(vi.shape(), &[2,4,2]);
        // Test against sequential iterator
        let seq = [7f32,6., 5.,4.,3.,2.,1.,0.,15.,14.,13., 12.,11.,  10.,   9.,   8.];
        for (a, b) in vi.clone().iter().zip(seq.iter()) {
            assert_eq!(*a, *b);
        }
    }
    {
        let vi = mat.slice([C, Slice(0, None, -5), C]);
        let seq = [6_f32, 7., 14., 15.];
        for (a, b) in vi.iter().zip(seq.iter()) {
            assert_eq!(*a, *b);
        }
    }
}

#[test]
fn test_cow()
{
    let mut mat = Array::<int, _>::zeros((2u,2u));
    mat[(0, 0)] = 1;
    let n = mat.clone();
    mat[(0, 1)] = 2;
    mat[(1, 0)] = 3;
    mat[(1, 1)] = 4;
    assert_eq!(mat[(0,0)], 1);
    assert_eq!(mat[(0,1)], 2);
    assert_eq!(n[(0,0)], 1);
    assert_eq!(n[(0,1)], 0);
    let mut rev = mat.reshape(4u).slice([Slice(0, None, -1)]);
    assert_eq!(rev[0], 4);
    assert_eq!(rev[1], 3);
    assert_eq!(rev[2], 2);
    assert_eq!(rev[3], 1);
    let before = rev.clone();
    // mutation
    rev[0] = 5;
    assert_eq!(rev[0], 5);
    assert_eq!(rev[1], 3);
    assert_eq!(rev[2], 2);
    assert_eq!(rev[3], 1);
    assert_eq!(before[0], 4);
    assert_eq!(before[1], 3);
    assert_eq!(before[2], 2);
    assert_eq!(before[3], 1);
}

#[test]
fn test_sub()
{
    let mat = Array::from_iter(range(0.0f32, 16.0)).reshape((2u, 4u, 2u));
    let s1 = mat.subview(0,0);
    let s2 = mat.subview(0,1);
    assert_eq!(s1.shape(), &[4, 2]);
    assert_eq!(s2.shape(), &[4, 2]);
    let n = Array::from_iter(range(8.0f32, 16.0)).reshape((4u,2u));
    assert_eq!(n, s2);
    let m = Array::from_vec(vec![2f32, 3., 10., 11.]).reshape((2u, 2u));
    assert_eq!(m, mat.subview(1, 1));
}

#[test]
fn diag()
{
    let d = Array::from_slices([[1., 2., 3.0f32]]).diag();
    assert_eq!(d.shape(), &[1]);
    let d = Array::from_slices([[1., 2., 3.0f32], [0., 0., 0.]]).diag();
    assert_eq!(d.shape(), &[2]);
    let d = Array::<f32>::from_slices([]).diag();
    assert_eq!(d.shape(), &[0]);
    let d = Array::<f32, _>::zeros(()).diag();
    assert_eq!(d.shape(), &[1]);
}

#[test]
fn swapaxes()
{
    let mut a = Array::from_slices([[1., 2.], [3., 4.0f32]]);
    let     b = Array::from_slices([[1., 3.], [2., 4.0f32]]);
    assert!(a != b);
    a.swap_axes(0, 1);
    assert_eq!(a, b);
    a.swap_axes(1, 1);
    assert_eq!(a, b);
    assert_eq!(a.raw_data(), &[1., 2., 3., 4.]);
    assert_eq!(b.raw_data(), &[1., 3., 2., 4.]);
}

#[test]
fn standard_layout()
{
    let mut a = Array::from_slices([[1., 2.], [3., 4.0f32]]);
    assert!(a.is_standard_layout());
    a.swap_axes(0, 1);
    assert!(!a.is_standard_layout());
    a.swap_axes(0, 1);
    assert!(a.is_standard_layout());
    let x1 = a.subview(0, 0);
    assert!(x1.is_standard_layout());
    let x2 = a.subview(1, 0);
    assert!(!x2.is_standard_layout());
}

#[test]
fn assign()
{
    let mut a = Array::from_slices([[1., 2.], [3., 4.0f32]]);
    let     b = Array::from_slices([[1., 3.], [2., 4.0f32]]);
    a.assign(&b);
    assert_eq!(a, b);

    /* Test broadcasting */
    a.assign(&Array::zeros(1u));
    assert_eq!(a, Array::zeros((2u, 2u)));
}

#[test]
fn dyn_dimension()
{
    let a = Array::from_slices([[1., 2.], [3., 4.0f32]]).reshape(vec![2u, 2]);
    assert_eq!(a - a, Array::zeros(vec![2u,2u]));

    let mut dim = Vec::from_elem(1024, 1u);
    dim.as_mut_slice()[16] = 4;
    dim.as_mut_slice()[17] = 3;
    let z = Array::<f32, Vec<uint>>::zeros(dim.clone());
    assert_eq!(z.shape(), dim.as_slice());
}

#[test]
fn sum_mean()
{
    let a = arr2([[1., 2.], [3., 4.0_f32]]);
    assert_eq!(a.sum(0), arr1([4., 6.]));
    assert_eq!(a.sum(1), arr1([3., 7.]));
    assert_eq!(a.mean(0), arr1([2., 3.]));
    assert_eq!(a.mean(1), arr1([1.5, 3.5]));
    assert_eq!(a.sum(1).sum(0), arr0(10.));
}