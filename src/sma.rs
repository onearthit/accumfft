pub struct SMA {
    alpha: f32,
    value: f32,
}

impl SMA {
    pub fn new(alpha: f32) -> Self {
        SMA {
            alpha,
            value: 0.0,
        }
    }

    pub fn feed(&mut self, value: f32) {
        self.value = self.alpha * value + (1.0 - self.alpha) * self.value;
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

#[cfg(test)]
mod ema_tests {
    use super::*;

    #[test]
    fn ema() {
        let mut ema = SMA::new(0.5);
        ema.feed(1.0);
        assert_eq!(ema.value(), 0.5);
        ema.feed(2.0);
        assert_eq!(ema.value(), 1.25);
        ema.feed(3.0);
        assert_eq!(ema.value(), 2.125);
    }

    #[test]
    fn resetable() {
        let mut ema = SMA::new(0.5);
        ema.feed(1.0);
        assert_eq!(ema.value(), 0.5);
        ema.reset();
        assert_eq!(ema.value(), 0.0);
        ema.feed(1.0);
        assert_eq!(ema.value(), 0.5);
    }
}