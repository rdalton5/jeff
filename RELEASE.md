# Release Guide

This document explains how to create releases for the Jeff VST plugin.

## How to Create a Release

### Method 1: Using Git Tags (Recommended)

1. **Update the version in Cargo.toml**:
   ```toml
   [package]
   name = "jeff"
   version = "1.0.0"  # Update this
   ```

2. **Update CHANGELOG.md** with the new version and changes.

3. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Release v1.0.0"
   ```

4. **Create and push a version tag**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

5. **GitHub Actions will automatically**:
   - Build the plugin for all platforms (Linux, Windows, macOS Intel, macOS ARM)
   - Create a GitHub release with the built artifacts
   - Attach the plugin bundles to the release

### Method 2: Manual Release Creation

1. Go to your GitHub repository
2. Click "Releases" → "Create a new release"
3. Choose a tag (e.g., `v1.0.0`) or create a new one
4. Fill in the release title and description
5. Upload manually built plugin files
6. Publish the release

## Build Artifacts

Each release will include:

- `jeff-linux-x64.tar.gz` - Linux VST3 + CLAP
- `jeff-windows-x64.zip` - Windows VST3 + CLAP  
- `jeff-macos-x64.tar.gz` - macOS Intel VST3 + CLAP
- `jeff-macos-arm64.tar.gz` - macOS ARM64 VST3 + CLAP

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner  
- **PATCH** version when you make backwards compatible bug fixes

Examples:
- `v1.0.0` - Initial release
- `v1.1.0` - Add new features (tempo sync, filters)
- `v1.0.1` - Bug fixes
- `v2.0.0` - Breaking changes (parameter changes, etc.)

## Pre-release Testing

Before creating a release:

1. **Test locally**:
   ```bash
   cargo test
   cargo run --example delay_test
   ```

2. **Build for all platforms**:
   ```bash
   cargo nih-plug bundle jeff --release
   ```

3. **Test in your DAW** to ensure the plugin loads and works correctly

4. **Check CI builds** pass on GitHub

## Release Notes

Include in your release notes:
- New features added
- Bug fixes
- Breaking changes (if any)
- Installation instructions
- Known issues

GitHub can auto-generate release notes, or you can write custom ones based on your CHANGELOG.md.
