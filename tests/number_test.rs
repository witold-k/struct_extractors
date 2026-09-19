// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_number;
use num_traits::{Float, Zero, One};

#[extract_number(val)]
#[derive(Clone, Copy, Default)]
pub struct FloatMetric<T: Float> {
    pub val: T,
    // other arbitary user defined fields
    pub other_field1: u16,
    pub other_field2: u32
}

impl<T: Float> FloatMetric<T> {
    fn new(v: T) -> Self {
        Self { val: v, other_field1: 1u16, other_field2: 2u32 }
    }
}

#[test]
fn test_basic_arithmetic() {
    let a = FloatMetric::<f64>::new(10.0);
    let b = FloatMetric::<f64>::new(2.0);

    assert_eq!((a + b).val, 12.0);
    assert_eq!((a - b).val, 8.0);
    assert_eq!((a * b).val, 20.0);
    assert_eq!((a / b).val, 5.0);
}

#[test]
fn test_mul_div_with_inner_type() {
    let a = FloatMetric::<f64>::new(10.0);

    assert_eq!((a * 3.0).val, 30.0);
    assert_eq!((a / 2.0).val, 5.0);
}

#[test]
fn test_assign_ops() {
    let mut x = FloatMetric::<f64>::new(10.0);
    let y = FloatMetric::<f64>::new(2.0);

    x += y;
    assert_eq!(x.val, 12.0);

    x -= y;
    assert_eq!(x.val, 10.0);

    x *= 3.0;
    assert_eq!(x.val, 30.0);

    x /= 2.0;
    assert_eq!(x.val, 15.0);
}

#[test]
fn test_neg() {
    let a = FloatMetric::<f64>::new(5.0);
    assert_eq!((-a).val, -5.0);
}

#[test]
fn test_comparisons() {
    let a = FloatMetric::<f64>::new(3.0);
    let b = FloatMetric::<f64>::new(7.0);

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);
}

#[test]
fn test_sum_and_product() {
    let values = [
        FloatMetric::<f64>::new(2.0),
        FloatMetric::<f64>::new(3.0),
        FloatMetric::<f64>::new(5.0),
    ];

    let sum: FloatMetric<f64> = values.iter().copied().sum();
    assert_eq!(sum.val, 10.0);

    let product: FloatMetric<f64> = values.iter().copied().product();
    assert_eq!(product.val, 30.0);
}

#[test]
fn test_zero_one() {
    let z = FloatMetric::<f64>::zero();
    let o = FloatMetric::<f64>::one();

    assert_eq!(z.val, 0.0);
    assert_eq!(o.val, 1.0);
}

