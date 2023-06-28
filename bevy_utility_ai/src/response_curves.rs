/// The trait for implementing a Response Curve which transforms an arbitrary input.
/// Note that the output of evaluate will be capped between 0.0 and 1.0. Therefore if
/// implementing this yourself it's probably a good idea to cap the output yourself in
/// the transform to avoid unexpected capping by the framework.
pub trait ResponseCurve: Send + Sync {
    fn transform(&self, input: f32) -> f32;
}

/// Implements the formula `y = slope * (x - x_shift) + y_shift`
#[derive(Debug, Clone, Copy)]
pub struct LinearCurve {
    pub slope: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl LinearCurve {
    pub fn new(slope: f32) -> Self {
        Self {
            slope,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            slope: self.slope,
            x_shift,
            y_shift,
        }
    }
}

impl ResponseCurve for LinearCurve {
    fn transform(&self, input: f32) -> f32 {
        self.slope * (input - self.x_shift) + self.y_shift
    }
}

/// Implements the formula `y = slope * (x - x_shift) ^ k + y_shift`
#[derive(Debug, Clone, Copy)]
pub struct PolynomialCurve {
    pub slope: f32,
    pub k: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl PolynomialCurve {
    pub fn new(slope: f32, k: f32) -> Self {
        Self {
            slope,
            k,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            slope: self.slope,
            k: self.k,
            x_shift,
            y_shift,
        }
    }
}

impl ResponseCurve for PolynomialCurve {
    fn transform(&self, input: f32) -> f32 {
        self.slope * (input - self.x_shift).powf(self.k) + self.y_shift
    }
}

/// Implements the formula `y = (1 / (1 + k ^ - (x - x_shift))) + y_shift`
#[derive(Debug, Clone, Copy)]
pub struct LogisticCurve {
    pub k: f32,
    pub x_shift: f32,
    pub y_shift: f32,
}

impl LogisticCurve {
    pub fn new(k: f32) -> Self {
        Self {
            k,
            x_shift: 0.0,
            y_shift: 0.0,
        }
    }

    pub fn shifted(self, x_shift: f32, y_shift: f32) -> Self {
        Self {
            k: self.k,
            x_shift,
            y_shift,
        }
    }
}

impl ResponseCurve for LogisticCurve {
    fn transform(&self, input: f32) -> f32 {
        1.0 / (1.0 + self.k.powf(-input + self.x_shift)) + self.y_shift
    }
}
