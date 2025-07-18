use monotonic_time_rs::Millis;
use monotonic_time_rs::MillisDuration;
/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/metricator
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use num_traits::Bounded;
use num_traits::ToPrimitive;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Div};

#[derive(Debug, PartialEq)]
pub struct MinMaxAvg<T: Display> {
    pub min: T,
    pub avg: f32,
    pub max: T,
    pub unit: &'static str,
}

impl<T: Display> MinMaxAvg<T> {
    pub const fn new(min: T, avg: f32, max: T) -> Self {
        Self {
            min,
            avg,
            max,
            unit: "",
        }
    }

    pub fn with_unit(mut self, unit: &'static str) -> Self {
        self.unit = unit;
        self
    }
}

impl<T: Display> Display for MinMaxAvg<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min:{}{}, avg:{}{}, max:{}{}",
            self.min, self.unit, self.avg, self.unit, self.max, self.unit,
        )
    }
}

/// Evaluating how many times something occurs every second.
#[derive(Debug)]
pub struct RateMetric {
    count: u32,
    last_calculated_at: Millis,
    average: f32,
    measurement_interval: MillisDuration,
}

impl RateMetric {
    /// Creates a new `RateMetric` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The initial [`Millis`] from which time tracking starts.
    ///
    /// # Returns
    ///
    /// A `RateMetric` instance with an initialized count and time.
    pub fn new(time: Millis) -> Self {
        Self {
            count: 0,
            last_calculated_at: time,
            measurement_interval: MillisDuration::from_millis(500),
            average: 0.0,
        }
    }

    pub fn with_interval(time: Millis, measurement_interval: f32) -> Self {
        Self {
            count: 0,
            last_calculated_at: time,
            measurement_interval: MillisDuration::from_secs(measurement_interval)
                .expect("measurement interval should be positive"),
            average: 0.0,
        }
    }

    /// Increments the internal event count by one.
    ///
    /// Call this method each time an event occurs that you want to track.
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Adds a specified number of events to the internal count.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of events to add.
    pub fn add(&mut self, count: u32) {
        self.count += count;
    }

    /// Updates the rate calculation based on the elapsed time since the last calculation.
    ///
    /// # Arguments
    ///
    /// * `time` - The current [`Millis`] representing the time at which the update is triggered.
    ///
    /// If the elapsed time since the last calculation is less than the measurement interval,
    /// this method returns early without updating the rate.
    pub fn update(&mut self, time: Millis) {
        let elapsed_time = time - self.last_calculated_at;
        if elapsed_time < self.measurement_interval {
            return;
        }

        let rate = self.count as f32 / elapsed_time.as_secs();

        // Reset the counter and start time for the next period
        self.count = 0;
        self.last_calculated_at = time;
        self.average = rate;
    }

    pub fn rate(&self) -> f32 {
        self.average
    }
}

/// Tracks minimum, maximum, and average values for numeric data (e.g., `i32`, `u32`, `f32`).
#[derive(Debug)]
pub struct AggregateMetric<T> {
    sum: T,
    count: u8,
    max: T,
    min: T,
    threshold: u8,
    max_ack: T,
    min_ack: T,
    avg: f32,
    avg_is_set: bool,
    unit: &'static str,
}

impl<T> AggregateMetric<T>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Copy
        + PartialOrd
        + Default
        + Debug
        + Display
        + Bounded
        + ToPrimitive,
{
    /// Creates a new `AggregateMetric` instance with a given threshold.
    pub fn new(threshold: u8) -> Result<Self, String> {
        if threshold == 0 {
            Err("threshold can not be zero".to_string())
        } else {
            Ok(Self {
                sum: T::default(),
                count: 0,
                max: T::default(),
                min: T::default(),
                threshold,
                max_ack: T::min_value(),
                min_ack: T::max_value(),
                avg: 0.0,
                avg_is_set: false,
                unit: "",
            })
        }
    }

    pub fn with_unit(mut self, unit: &'static str) -> Self {
        self.unit = unit;
        self
    }

    /// Calculates the mean value, returning `None` if no values have been added.
    pub fn average(&self) -> Option<f32> {
        if self.avg_is_set {
            Some(self.avg)
        } else {
            None
        }
    }

    /// Adds a value of type `T` to the metric.
    pub fn add(&mut self, value: T) {
        self.sum = self.sum + value;
        self.count += 1;

        // Update the max and min acknowledgments
        if value > self.max_ack {
            self.max_ack = value;
        }
        if value < self.min_ack {
            self.min_ack = value;
        }

        // Check if the threshold is reached to calculate stats and reset counters
        if self.count >= self.threshold {
            let sum_f32 = self.sum.to_f32().unwrap_or(0.0);
            let avg_f32 = sum_f32 / self.count as f32;

            self.avg = avg_f32;

            self.min = self.min_ack;
            self.max = self.max_ack;
            self.max_ack = T::min_value();
            self.min_ack = T::max_value();
            self.count = 0;
            self.avg_is_set = true;
            self.sum = T::default();
        }
    }

    /// Returns the minimum, average, and maximum values as a tuple, if available.
    pub fn values(&self) -> Option<MinMaxAvg<T>> {
        if self.avg_is_set {
            Some(MinMaxAvg::new(self.min, self.avg, self.max).with_unit(self.unit))
        } else {
            None
        }
    }
}
