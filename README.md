# Metricator

![Crates.io](https://img.shields.io/crates/v/metricator)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

**Metricator** is a lightweight Rust library designed to help you track and analyze metrics effortlessly. It provides two main utilities:

1. **RateMetric**: Evaluates how frequently events occur per second.
2. **AggregateMetric**: Tracks minimum, maximum, and average values for various numeric data types.

Whether you're monitoring system performance, tracking user interactions, or analyzing data streams, Metricator offers a simple and efficient way to gather and process essential metrics.

## Features

- **RateMetric**:
  - Calculate event rates based on elapsed time intervals.

- **AggregateMetric**:
  - Generic support for numeric types (`i32`, `u32`, `f32`, etc.).
  - Track minimum, maximum, and average values.
  - Configurable thresholds to trigger calculations.

- **Efficient and Lightweight**: Minimal overhead, suitable for performance-critical applications.

## Installation

Add `metricator` to your `Cargo.toml`:

```toml
[dependencies]
metricator = "^0.0.1"
```
