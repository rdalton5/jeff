# Jeff - Delay VST Plugin

A simple delay effect plugin written in Rust using the NIH-plug framework.

## ✅ Current Status

The delay plugin is **fully functional with parameter automation**! The plugin includes:

- ✅ Circular buffer delay implementation
- ✅ Stereo processing (left/right channels)
- ✅ **Full parameter automation** with proper names and units
- ✅ **Parameter smoothing** for glitch-free automation
- ✅ **VST3 and CLAP support** (modern plugin formats)
- ✅ Cross-platform builds (Linux and Windows)
- ✅ Professional parameter UI integration

## Features

- **Delay Time**: 1-2000ms with logarithmic scaling
- **Feedback**: 0-95% with safety limiting to prevent runaway feedback
- **Wet Level**: 0-100% delayed signal mix
- **Dry Level**: 0-100% original signal mix
- **Parameter Automation**: All parameters are fully automatable in DAWs
- **Parameter Smoothing**: Smooth parameter changes prevent audio artifacts

## Parameters

Your DAW will show these named parameters with proper units:

1. **Delay Time** (1-2000 ms): Sets the delay time with logarithmic scaling
2. **Feedback** (0-95%): Amount of delayed signal fed back into the delay line
3. **Wet Level** (0-100%): Level of the delayed signal in the output
4. **Dry Level** (0-100%): Level of the original signal in the output

## Building

### Prerequisites

- Rust (latest stable version)
- Cargo

**For Windows cross-compilation on Linux:**
- MinGW cross-compiler: `sudo apt install gcc-mingw-w64-x86-64`
- Windows target: `rustup target add x86_64-pc-windows-gnu`

### Build Instructions

1. Clone or download this project
2. Navigate to the project directory
3. Build and bundle the plugin:

```bash
# Build universal VST3 bundle + CLAP for all platforms
cargo nih-plug bundle delay_vst --release && \
cargo nih-plug bundle delay_vst --release --target x86_64-pc-windows-gnu

# Or build raw libraries (optional, for development)
cargo build --release
```

Or use VS Code tasks:
- **Ctrl+Shift+P** → "Tasks: Run Task" → "Bundle VST Plugin (All Platforms)" (Default build)
- **Ctrl+Shift+P** → "Tasks: Run Task" → "Install VST3 Plugin" (Build + Install locally)

The bundled plugins will be located at:
- **Universal VST3 Bundle**: `target/bundled/delay_vst.vst3/` 
  - Contains binaries for: Linux, Windows (and macOS when built on Mac)
  - Single bundle works on all platforms
- **CLAP Plugin**: `target/bundled/delay_vst.clap` (Linux/macOS)

## Installation

### Linux
Copy the VST3 bundle or CLAP plugin to your plugin directory:
```bash
# VST3 (recommended - works in most modern DAWs)
mkdir -p ~/.vst3
cp -r target/bundled/delay_vst.vst3 ~/.vst3/

# CLAP (modern open-source format)
mkdir -p ~/.clap
cp target/bundled/delay_vst.clap ~/.clap/
```

### macOS
Copy the VST3 bundle to your plugin directory:
```bash
cp -r target/bundled/delay_vst.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

### Windows
Copy the VST3 bundle to your plugin directory:
```bash
# Example Windows VST3 directory:
# C:\Program Files\Common Files\VST3\

# Copy the entire delay_vst.vst3 folder to:
# C:\Program Files\Common Files\VST3\delay_vst.vst3\
```

**Important**: VST3 plugins are **bundles** (folders), not single files. Make sure to copy the entire `delay_vst.vst3` folder.

## Usage

1. Load the plugin in your DAW
   - Look for "Delay" by "Rob's Audio" in your VST3 or CLAP plugin list
   - The plugin provides **full parameter automation** with named controls
2. Adjust the parameters in your DAW:
   - **Delay Time**: 1-2000ms with smooth automation
   - **Feedback**: 0-95% (safety limited to prevent feedback loops)
   - **Wet Level**: 0-100% delayed signal mix
   - **Dry Level**: 0-100% original signal mix
3. All parameters are **automatable** and will show proper names and units in your DAW

## Testing

Run the delay algorithm test:
```bash
cargo run --example delay_test
```

This creates a `delay_test_output.csv` file showing how the delay buffer responds to an impulse.

## Development

This plugin is built using:
- [NIH-plug](https://github.com/robbert-vdh/nih-plug) - Modern Rust plugin framework
- VST3 and CLAP plugin format support
- Circular buffer implementation for delay processing
- Real-time safe audio processing with parameter automation

### Project Structure
- `src/lib.rs` - Main plugin implementation with NIH-plug
- `examples/delay_test.rs` - Test program for the delay algorithm  
- `Cargo.toml` - Rust project configuration
- `target/bundled/` - Generated VST3 and CLAP plugin bundles

### Next Steps
- [x] Full parameter automation support ✅
- [x] Professional parameter UI integration ✅  
- [x] VST3 and CLAP format support ✅
- [ ] Add more delay types (ping-pong, multi-tap)
- [ ] Add filtering to the delay line
- [ ] Add tempo sync for delay time
- [ ] Create custom GUI with NIH-plug

## License

This project is open source. See LICENSE file for details.

## Contributing

Feel free to submit issues and pull requests to improve the plugin!
