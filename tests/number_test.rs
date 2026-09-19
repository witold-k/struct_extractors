// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::{Float, One, Zero};
use struct_extractors::extract_number;

#[extract_number(value)]
#[derive(Clone, Copy, Default)]
pub struct FloatMetric<N>
where
    N: Float,
{
    pub value: N,
    pub tag: u16,
}

impl<N> FloatMetric<N>
where
    N: Float,
{
    fn new(value: N) -> Self {
        Self { value, tag: 7 }
    }
}

#[test]
fn arithmetic_uses_selected_field() {
    let a = FloatMetric::<f64>::new(10.0);
    let b = FloatMetric::<f64>::new(2.0);

    assert_eq!((a + b).value, 12.0);
    assert_eq!((a - b).value, 8.0);
    assert_eq!((a * b).value, 20.0);
    assert_eq!((a / b).value, 5.0);
}

#[test]
fn scalar_and_assign_ops_use_actual_field_type() {
    let mut value = FloatMetric::<f64>::new(10.0);

    assert_eq!((value * 3.0).value, 30.0);
    assert_eq!((value / 2.0).value, 5.0);

    value += FloatMetric::new(2.0);
    value -= FloatMetric::new(1.0);
    value *= 3.0;
    value /= 2.0;

    assert_eq!(value.value, 16.5);
}

#[test]
fn comparison_sum_product_zero_and_one_work() {
    let a = FloatMetric::<f64>::new(3.0);
    let b = FloatMetric::<f64>::new(7.0);
    assert!(a < b);

    let values = [
        FloatMetric::<f64>::new(2.0),
        FloatMetric::<f64>::new(3.0),
        FloatMetric::<f64>::new(5.0),
    ];

    assert_eq!(values.iter().copied().sum::<FloatMetric<f64>>().value, 10.0);
    assert_eq!(values.iter().copied().product::<FloatMetric<f64>>().value, 30.0);
    assert_eq!(FloatMetric::<f64>::zero().value, 0.0);
    assert_eq!(FloatMetric::<f64>::one().value, 1.0);
}

#[test]
fn untouched_fields_follow_struct_update_semantics() {
    let value = FloatMetric::<f64>::new(2.0);
    assert_eq!((-value).tag, 7);
}
