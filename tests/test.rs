/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/metricator
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use metricator::{AggregateMetric, MinMaxAvg, RateMetric};
use monotonic_time_rs::{Millis, MillisDuration};

#[test_log::test]
fn rate() {
    let mut now = Millis::new(0);
    let mut m = RateMetric::new(now);

    m.add(10);

    now += MillisDuration::from_secs(10.0).expect("should be positive");

    assert_eq!(m.rate(), 0.0);

    m.update(now);

    assert_eq!(m.rate(), 1.0);
}

#[test_log::test]
fn aggregate() {
    let mut aggregate = AggregateMetric::new(3).expect("should not be zero");

    aggregate.add(2.5);

    assert_eq!(aggregate.average(), None);

    aggregate.add(5.5);
    aggregate.add(1.0);

    assert_eq!(aggregate.average(), Some(3.0));
}

#[test_log::test]
fn aggregate_int() {
    let mut aggregate = AggregateMetric::new(3).expect("should not be zero");

    aggregate.add(-1);

    assert_eq!(aggregate.average(), None);

    aggregate.add(2);
    aggregate.add(5);

    assert_eq!(aggregate.average(), Some(2.0));
}

#[test_log::test]
fn zero_threshold() {
    let result: Result<AggregateMetric<f32>, String> = AggregateMetric::new(0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "threshold can not be zero".to_string());
}

#[test_log::test]
fn min_max_values() {
    let mut aggregate = AggregateMetric::new(3).expect("should not be zero");

    aggregate.add(5);
    aggregate.add(2);
    aggregate.add(8);

    let values = aggregate.values().expect("should calculate values");
    assert_eq!(values.min, 2);
    assert_eq!(values.avg, 5.0);
    assert_eq!(values.max, 8);
}

#[test_log::test]
fn all_equal_values_f32() {
    let mut aggregate = AggregateMetric::new(3).expect("threshold should not be zero");

    aggregate.add(5.0);
    aggregate.add(5.0);
    aggregate.add(5.0);

    assert_eq!(aggregate.average(), Some(5.0));
    assert_eq!(aggregate.values(), Some(MinMaxAvg::new(5.0, 5.0, 5.0)));
}

#[test_log::test]
fn all_equal_values_i32() {
    let mut aggregate = AggregateMetric::new(3).expect("threshold should not be zero");

    aggregate.add(7);
    aggregate.add(7);
    aggregate.add(7);

    assert_eq!(aggregate.average(), Some(7.0));
    assert_eq!(aggregate.values(), Some(MinMaxAvg::new(7, 7.0, 7)));
}
