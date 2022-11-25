use pyo3::prelude::*;
use std::time;

#[pyclass(name = "SlidingWindow", module="pycooldown.bindings")]
struct PySlidingWindow {
    capacity: i64,
    period: f64,
    window: f64,
    tokens: i64,
    last: f64,
}

fn time_since_epoch() -> f64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("How did you do this? Time can't go backward.")
        .as_secs_f64()
}

#[pymethods]
impl PySlidingWindow {
    #[new]
    fn new(capacity: i64, period: f64) -> Self {
        Self {
            capacity,
            period,
            window: 0.0,
            tokens: capacity,
            last: 0.0,
        }
    }

    fn get_tokens(&self, current: Option<f64>) -> i64 {
        let current = match current {
            Some(t) => t,
            None => time_since_epoch(),
        };

        if current > self.window + self.period {
            self.capacity
        } else {
            self.tokens
        }
    }

    fn get_retry_after(&self) -> f64 {
        let current = time_since_epoch();
        let tokens = self.get_tokens(Some(current));

        if tokens == 0 {
            self.period - (current - self.window)
        } else {
            0.0
        }
    }

    fn update_ratelimit(&mut self) -> Option<f64> {
        let current = time_since_epoch();
        self.last = current;

        self.tokens = self.get_tokens(Some(current));

        if self.tokens == self.capacity {
            self.window = current;
        };

        if self.tokens == 0 {
            Some(self.period - (current - self.window))
        } else {
            self.tokens -= 1;
            None
        }
    }

    fn reset(&mut self) -> () {
        self.tokens = self.capacity;
        self.last = 0.0;
    }
}

#[pymodule]
fn _rust_bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySlidingWindow>()?;
    Ok(())
}
