# Copilot Instructions

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust VST audio plugin project called "Jeff" using the nih-plug framework for creating delay effects. 

## Project Context
- This project creates a VST delay plugin called "Jeff" with configurable delay time, feedback, wet/dry mix
- Uses the `nih-plug` crate for VST3/CLAP plugin development
- Implements audio processing with delay buffers for stereo audio
- Built as a cdylib (C dynamic library) for VST host compatibility

## Key Components
- `DelayBuffer`: Circular buffer for storing delayed audio samples
- `DelayParams`: Plugin parameters (delay time, feedback, wet/dry levels)
- `Jeff`: Main plugin struct implementing the `Plugin` trait
- Audio processing happens in the `process` method with real-time constraints

## Development Guidelines
- Follow Rust best practices for audio processing (avoid allocations in process method)
- Keep feedback values below 1.0 to prevent runaway feedback
- Use proper sample rate conversion for delay time calculations
- Maintain thread safety as VST hosts may call plugin methods from different threads
- Test with different buffer sizes and sample rates
