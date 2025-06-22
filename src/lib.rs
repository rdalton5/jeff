use nih_plug::prelude::*;
use std::sync::Arc;

/// Simple delay buffer for storing delayed samples
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

/// Plugin parameters
#[derive(Params)]
struct DelayParams {
    /// Delay time in milliseconds
    #[id = "delay_time"]
    pub delay_time: FloatParam,

    /// Feedback amount (0-95%)
    #[id = "feedback"]
    pub feedback: FloatParam,

    /// Wet signal level
    #[id = "wet_level"]
    pub wet_level: FloatParam,

    /// Dry signal level
    #[id = "dry_level"]
    pub dry_level: FloatParam,
}

impl Default for DelayParams {
    fn default() -> Self {
        Self {
            delay_time: FloatParam::new(
                "Delay Time",
                250.0, // 250ms default
                FloatRange::Skewed {
                    min: 1.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            feedback: FloatParam::new(
                "Feedback",
                30.0, // 30% default
                FloatRange::Linear { min: 0.0, max: 95.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            wet_level: FloatParam::new(
                "Wet Level",
                30.0, // 30% default
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            dry_level: FloatParam::new(
                "Dry Level",
                70.0, // 70% default
                FloatRange::Linear { min: 0.0, max: 100.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
        }
    }
}

/// The main delay plugin struct
pub struct Jeff {
    params: Arc<DelayParams>,
    delay_buffer_left: DelayBuffer,
    delay_buffer_right: DelayBuffer,
    sample_rate: f32,
}

impl Default for Jeff {
    fn default() -> Self {
        Self {
            params: Arc::new(DelayParams::default()),
            delay_buffer_left: DelayBuffer::new(192000), // Max 4 seconds at 48kHz
            delay_buffer_right: DelayBuffer::new(192000),
            sample_rate: 44100.0,
        }
    }
}

impl Plugin for Jeff {
    const NAME: &'static str = "Jeff";
    const VENDOR: &'static str = "Rob's Audio";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = env!("CARGO_PKG_AUTHORS");

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        self.delay_buffer_left = DelayBuffer::new(192000);
        self.delay_buffer_right = DelayBuffer::new(192000);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Get current parameter values
        let delay_time_ms = self.params.delay_time.value();
        let feedback = self.params.feedback.value() / 100.0; // Convert % to 0-1
        let wet_level = self.params.wet_level.value() / 100.0;
        let dry_level = self.params.dry_level.value() / 100.0;

        // Calculate delay in samples
        let delay_samples = ((delay_time_ms / 1000.0) * self.sample_rate) as usize;

        for channel_samples in buffer.iter_samples() {            
            // Process each channel
            for (channel_idx, sample) in channel_samples.into_iter().enumerate() {
                if channel_idx == 0 {
                    // Left channel
                    let input_sample = *sample;
                    let delayed_left = self.delay_buffer_left.process(input_sample, delay_samples);
                    
                    // Apply feedback
                    let feedback_sample = delayed_left * feedback;
                    if self.delay_buffer_left.write_pos > 0 {
                        self.delay_buffer_left.buffer[self.delay_buffer_left.write_pos - 1] += feedback_sample;
                    } else {
                        self.delay_buffer_left.buffer[self.delay_buffer_left.size - 1] += feedback_sample;
                    }
                    
                    *sample = dry_level * input_sample + wet_level * delayed_left;
                } else if channel_idx == 1 {
                    // Right channel
                    let input_sample = *sample;
                    let delayed_right = self.delay_buffer_right.process(input_sample, delay_samples);
                    
                    // Apply feedback
                    let feedback_sample = delayed_right * feedback;
                    if self.delay_buffer_right.write_pos > 0 {
                        self.delay_buffer_right.buffer[self.delay_buffer_right.write_pos - 1] += feedback_sample;
                    } else {
                        self.delay_buffer_right.buffer[self.delay_buffer_right.size - 1] += feedback_sample;
                    }
                    
                    *sample = dry_level * input_sample + wet_level * delayed_right;
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Jeff {
    const CLAP_ID: &'static str = "com.robs-audio.jeff";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Jeff - A simple delay effect");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Delay];
}

impl Vst3Plugin for Jeff {
    const VST3_CLASS_ID: [u8; 16] = *b"JeffVSTRobsAudio";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_clap!(Jeff);
nih_export_vst3!(Jeff);
