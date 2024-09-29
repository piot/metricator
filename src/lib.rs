/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/metricator
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use num_traits::Bounded;
use num_traits::ToPrimitive;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::{Add, Div};
use std::time::Instant;

/// Evaluating how many times something occurs every second.
#[derive(Debug)]
pub struct RateMetric {
    count: u32,
    last_calculated_at: Instant,
    average: f32,
    measurement_interval: f32,
}

impl RateMetric {
    /// Creates a new `RateMetric` instance.
    ///
    /// # Arguments
    ///
    /// * `time` - The initial `Instant` from which time tracking starts.
    ///
    /// # Returns
    ///
    /// A `RateMetric` instance with an initialized count and time.
    pub fn new(time: Instant) -> Self {
        Self {
            count: 0,
            last_calculated_at: time,
            measurement_interval: 0.5,
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
    /// * `time` - The current `Instant` representing the time at which the update is triggered.
    ///
    /// If the elapsed time since the last calculation is less than the measurement interval,
    /// this method returns early without updating the rate.
    pub fn update(&mut self, time: Instant) {
        let elapsed_time = time - self.last_calculated_at;
        let seconds = elapsed_time.as_secs_f32();
        if seconds < self.measurement_interval {
            return;
        }

        let rate = self.count as f32 / seconds;

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
}

impl<T> AggregateMetric<T>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Copy
        + PartialOrd
        + Default
        + From<u8>
        + Debug
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
            })
        }
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
    pub fn values(&self) -> Option<(T, f32, T)> {
        if self.avg_is_set {
            Some((self.min, self.avg, self.max))
        } else {
            None
        }
    }
}
