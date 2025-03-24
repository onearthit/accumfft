use rustfft::{FftPlanner, num_complex::Complex};

pub struct AccumFFT {
    samples: Vec<Complex<f32>>
}

impl AccumFFT {
    pub fn new(freq: f32) -> Self {
        AccumFFT {
            samples: // Ref: Nyquist Frequency
                Vec::with_capacity((freq * 2_f32).ceil() as usize)
        }
    }

    /// Feed an input to FFT Accumulator \
    /// If samples are enough, the feeded sample are ignored
    pub fn feed(&mut self, elem: f32) {
        if self.samples.len() < self.samples.capacity() {
            self.samples.push(Complex{ re: elem, im: 0.0 });
        }
    }

    pub fn amplitude(&self) -> Option<f32> {
        // If not sampling enough, We cannot continue further
        if self.samples.len() != self.samples.capacity() {
            return None
        }

        // FftPlanner is process in-place we must clone the sampling freq to perform on them instead
        let mut cloned_samples = self.samples.clone();
        FftPlanner::<f32>::new()
            .plan_fft_forward(self.samples.capacity())
            .process(&mut cloned_samples);

        // Now, we need to find |x| where x is amplitude of desired frequency and returned it
        cloned_samples
            .iter()
            .map(|cmplx| cmplx.norm())
            .take(self.samples.capacity() / 2)
            .next()
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }
}

#[cfg(test)]
mod accum_fft_tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn amplitude_of_50hz() {
        let mut accum_fft = AccumFFT::new(50_f32);
        test_50hz(&mut accum_fft);
    }

    fn test_50hz(accum_fft: &mut AccumFFT) {
        for amp in (0..100)
            .map(|i| i as f32)
            .map(|i| (2.0 * PI * i / 100.0 * 50.0).sin()) {
                accum_fft.feed(amp);
        }
        assert_eq!(accum_fft.amplitude(), Some(0.00011316923));
    }

    #[test]
    fn not_enough_samples() {
        let mut accum_fft = AccumFFT::new(50_f32);
        test_not_enough_samples(&mut accum_fft);
    }

    fn test_not_enough_samples(accum_fft: &mut AccumFFT) {
        for amp in (0..99)
            .map(|i| i as f32)
            .map(|i| (2.0 * PI * i / 100.0 * 50.0).sin()) {
                accum_fft.feed(amp);
        }
        assert_eq!(accum_fft.amplitude(), None);
    }

    #[test]
    fn resetable() {
        let mut accum_fft = AccumFFT::new(50_f32);
        test_50hz(&mut accum_fft);
        accum_fft.reset();
        test_not_enough_samples(&mut accum_fft);
        accum_fft.reset();
        test_50hz(&mut accum_fft);
    }
}
