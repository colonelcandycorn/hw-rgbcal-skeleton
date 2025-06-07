use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    
    ///    calculates the frame time for the RGB LEDs based on the frame rate.
    ///    The frame rate should be a multiple of 10 and be between 10 and 160
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// Creates a new Rgb instance with the specified RGB pins and frame rate.
    pub async fn new(rgb: RgbPins) -> Self {
        let tick_time = Self::frame_tick_time(get_frame_rate().await);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Sets the specified LED to high for the given duration and then low for the same duration.
    /// This simulates a PWM effect by turning the LED on and off at a specific frequency.
    /// # Arguments
    /// * `led` - The index of the LED to control (0 for red, 1 for green, 2 for blue).
    /// # Returns
    /// This function does not return a value, but it will pause the execution for the duration of the LED on and off cycles.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Runs the RGB LED control loop, continuously updating the LED levels and stepping through each LED.
    /// # Returns
    /// This function does not return a value; it runs indefinitely, updating the RGB LEDs based on the current levels.
    pub async fn run(mut self) -> ! {
        loop {
            // going to set the frame rate here alongside the RGB levels
            self.tick_time = Self::frame_tick_time(get_frame_rate().await);
            self.levels = get_rgb_levels().await;

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
