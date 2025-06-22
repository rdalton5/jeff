use std::fs::File;
use std::io::Write;

// Copy the DelayBuffer from our main module for testing
struct DelayBuffer {
    buffer: Vec<f32>,
    write_pos: usize,
    size: usize,
}

impl DelayBuffer {
    fn new(size: usize) -> Self {
        DelayBuffer {
            buffer: vec![0.0; size],
            write_pos: 0,
            size,
        }
    }

    fn process(&mut self, input: f32, delay_samples: usize) -> f32 {
        // Ensure delay_samples doesn't exceed buffer size
        let delay_samples = delay_samples.min(self.size - 1);
        
        // Calculate read position
        let read_pos = if self.write_pos >= delay_samples {
            self.write_pos - delay_samples
        } else {
            self.size - (delay_samples - self.write_pos)
        };

        // Get delayed sample
        let delayed_sample = self.buffer[read_pos];

        // Write new sample to buffer
        self.buffer[self.write_pos] = input;

        // Advance write position
        self.write_pos = (self.write_pos + 1) % self.size;

        delayed_sample
    }
}

fn main() {
    println!("Testing VST Delay Buffer...");
    
    let sample_rate = 44100.0;
    let delay_time = 0.1; // 100ms delay
    let delay_samples = (delay_time * sample_rate) as usize;
    
    let mut delay_buffer = DelayBuffer::new(44100); // 1 second max
    
    // Create a test signal (impulse)
    let mut test_samples = vec![0.0; 8820]; // 200ms worth of samples
    test_samples[0] = 1.0; // Impulse at the beginning
    
    let mut output_samples = Vec::new();
    
    // Process through delay
    for sample in test_samples {
        let delayed = delay_buffer.process(sample, delay_samples);
        let mixed = 0.7 * sample + 0.3 * delayed; // 70% dry, 30% wet
        output_samples.push(mixed);
    }
    
    // Save to CSV for analysis
    let mut file = File::create("delay_test_output.csv").unwrap();
    writeln!(file, "sample,dry,delayed,mixed").unwrap();
    
    for (i, &mixed) in output_samples.iter().enumerate() {
        let dry = if i == 0 { 1.0 } else { 0.0 };
        let delayed = if i == delay_samples { 0.3 } else { 0.0 };
        writeln!(file, "{},{},{},{}", i, dry, delayed, mixed).unwrap();
    }
    
    println!("Test completed! Check delay_test_output.csv");
    println!("Expected: impulse at sample 0, delayed impulse at sample {}", delay_samples);
}
