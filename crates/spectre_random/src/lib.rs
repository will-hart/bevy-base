use rand::Rng;
/// A quasi-random number generator (i.e. DOTA 2 style proc)
pub struct QRNG {
    pub initial: f32,
    pub increment: f32,
    pub maximum: f32,
    unsuccessful: isize,
}

impl QRNG {
    pub fn new() -> Self {
        QRNG {
            initial: 0.1,
            increment: 0.05,
            maximum: 0.4,
            unsuccessful: 0,
        }
    }

    /// Returns true if the QRNG proc'd and resets
    pub fn test(&mut self) -> bool {
        let p =
            (self.initial + (self.unsuccessful as f32) * self.increment).max(self.maximum) as f64;

        let success = rand::thread_rng().gen_bool(p);

        if success {
            self.unsuccessful = 0;
        } else {
            self.unsuccessful += 1;
        }

        success
    }
}
