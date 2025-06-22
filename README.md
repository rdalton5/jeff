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

## Downloads

Get the latest release from the [Releases page](https://github.com/YOUR_USERNAME/jeff/releases) or build from source below.

**Release contains:**
- `jeff.vst3/` - Universal VST3 bundle (works on Linux, Windows, macOS Intel, macOS ARM64)
- `jeff-linux.clap` - CLAP plugin for Linux
- `jeff-macos-intel.clap` - CLAP plugin for macOS Intel  
- `jeff-macos-arm64.clap` - CLAP plugin for macOS ARM64

Available as both `jeff.tar.gz` and `jeff.zip` archives.

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
cargo nih-plug bundle jeff --release && \
cargo nih-plug bundle jeff --release --target x86_64-pc-windows-gnu

# Or build raw libraries (optional, for development)
cargo build --release
```

Or use VS Code tasks:
- **Ctrl+Shift+P** → "Tasks: Run Task" → "Bundle VST Plugin (All Platforms)" (Default build)
- **Ctrl+Shift+P** → "Tasks: Run Task" → "Install VST3 Plugin" (Build + Install locally)

The bundled plugins will be located at:
- **Universal VST3 Bundle**: `target/bundled/jeff.vst3/` 
  - Contains binaries for: Linux, Windows (and macOS when built on Mac)
  - Single bundle works on all platforms
- **CLAP Plugin**: `target/bundled/jeff.clap` (Linux/macOS)

## Installation

### Linux
Copy the VST3 bundle or CLAP plugin to your plugin directory:
```bash
# VST3 (recommended - works in most modern DAWs)
mkdir -p ~/.vst3
cp -r target/bundled/jeff.vst3 ~/.vst3/

# CLAP (modern open-source format)
mkdir -p ~/.clap
cp target/bundled/jeff.clap ~/.clap/
```

### macOS
Copy the VST3 bundle to your plugin directory:
```bash
cp -r target/bundled/jeff.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

### Windows
Copy the VST3 bundle to your plugin directory:
```bash
# Example Windows VST3 directory:
# C:\Program Files\Common Files\VST3\

# Copy the entire jeff.vst3 folder to:
# C:\Program Files\Common Files\VST3\jeff.vst3\
```

**Important**: VST3 plugins are **bundles** (folders), not single files. Make sure to copy the entire `jeff.vst3` folder.

## Usage

1. Load the plugin in your DAW
   - Look for "Jeff" by "Rob's Audio" in your VST3 or CLAP plugin list
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
