# GitHub Actions CI/CD

This repository uses GitHub Actions to automatically build the application for multiple platforms.

## Workflows

### 1. Build Workflow (`build.yml`)

**Trigger:** Automatically on every push to `main` or `develop` branches, or pull requests.

**Platforms:**
- Windows (x86_64)
- macOS (Apple Silicon & Intel)
- Linux (x86_64)
- Android

**Output:** Build artifacts uploaded to GitHub Actions for 90 days.

### 2. Release Workflow (`release.yml`)

**Trigger:** 
- Git tag push (e.g., `v1.0.0`)
- Manual workflow dispatch

**Output:** Creates a GitHub Release with all platform installers attached.

## Usage

### Quick Start - Download Builds

1. Go to **Actions** tab in GitHub
2. Click on the latest successful workflow run
3. Scroll to **Artifacts** section
4. Download the artifact for your platform

### Creating a Release

#### Option 1: Git Tag (Recommended)

```bash
# Tag the current commit
git tag v1.0.0

# Push the tag to GitHub
git push origin v1.0.0
```

The release workflow will automatically:
- Build for all platforms
- Create a draft release
- Attach all installers to the release
- You can then edit and publish the release

#### Option 2: Manual Trigger

1. Go to **Actions** tab
2. Click **Release Build** workflow
3. Click **Run workflow**
4. Enter version number (e.g., `1.0.0`)
5. Click **Run workflow** button

### Platform-Specific Details

#### Windows
- **Outputs:** `.msi` (Windows Installer), `.exe` (NSIS installer)
- **Location:** `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/`
- **Installation:** Double-click the installer

#### macOS
- **Outputs:** `.dmg` (Disk Image)
- **Location:** `src-tauri/target/{aarch64,x86_64}-apple-darwin/release/bundle/dmg/`
- **Architectures:** 
  - `aarch64` for Apple Silicon (M1/M2/M3)
  - `x86_64` for Intel Macs
- **Installation:** Open DMG, drag to Applications folder

#### Linux
- **Outputs:** `.deb` (Debian package), `.AppImage` (portable)
- **Location:** `src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/`
- **Installation:**
  - DEB: `sudo dpkg -i *.deb`
  - AppImage: `chmod +x *.AppImage && ./AppImage`

#### Android
- **Outputs:** `.apk` (sideload), `.aab` (Play Store)
- **Location:** `src-tauri/gen/android/app/build/outputs/`
- **Requirements:**
  - Java 17
  - Android NDK 26
  - Android SDK
- **Installation:**
  - APK: Enable "Unknown Sources" and install
  - AAB: Upload to Google Play Console

## Caching Strategy

Both workflows use caching to speed up builds:

- **pnpm cache:** Node dependencies cached by lockfile hash
- **Rust cache:** Cargo dependencies and build artifacts cached
- **Typical build times:**
  - First build: 15-25 minutes
  - Cached build: 3-8 minutes

## Troubleshooting

### Build Fails on Android

**Issue:** NDK or SDK not found

**Solution:** Ensure `NDK_HOME` is set correctly:
```yaml
env:
  NDK_HOME: ${{ env.ANDROID_SDK_ROOT }}/ndk/26.1.10909125
```

### macOS Notarization

**Issue:** App won't open on macOS (security warning)

**Solution:** Add code signing and notarization:
1. Set up Apple Developer certificates
2. Add secrets to repository:
   - `APPLE_CERTIFICATE`
   - `APPLE_CERTIFICATE_PASSWORD`
   - `APPLE_ID`
   - `APPLE_PASSWORD`
   - `APPLE_TEAM_ID`
3. Update `tauri.conf.json` with signing configuration

### Windows Defender Flags Executable

**Issue:** Windows Defender or antivirus flags the .exe

**Solution:** Add code signing certificate:
1. Obtain a code signing certificate
2. Add to repository secrets:
   - `WINDOWS_CERTIFICATE`
   - `WINDOWS_CERTIFICATE_PASSWORD`
3. Sign the executable in the workflow

## Customization

### Change Platforms

Edit the matrix in `build.yml`:

```yaml
matrix:
  platform:
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
    # Add or remove platforms here
```

### Add Linux ARM64

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  name: linux-aarch64
```

Install cross-compilation tools:
```bash
sudo apt-get install gcc-aarch64-linux-gnu
```

### Modify Artifact Upload

Change which files are uploaded:

```yaml
- name: Upload artifacts
  uses: actions/upload-artifact@v4
  with:
    name: my-app-windows
    path: |
      src-tauri/target/release/bundle/msi/*.msi
      # Add more patterns here
```

## Security Best Practices

1. **Never commit secrets:** Use GitHub Secrets for sensitive data
2. **Use dependabot:** Keep actions and dependencies updated
3. **Pin action versions:** Use SHA or specific versions
4. **Code signing:** Sign all releases for production
5. **Draft releases:** Review before publishing

## Cost Considerations

GitHub Actions is free for public repositories with these limits:
- 2,000 minutes/month for private repos
- macOS builds count as 10x minutes
- Windows builds count as 2x minutes
- Linux/Android builds count as 1x minutes

**Estimated usage for this project:**
- Full build all platforms: ~40 minutes (~200 minutes equivalent)
- ~10 releases per month = ~2,000 minutes

For private repositories, consider:
- Using Linux-only for testing
- Triggering releases manually
- Caching aggressively

## Additional Resources

- [Tauri Actions Documentation](https://tauri.app/v1/guides/building/ci-cd/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Tauri v2 Migration Guide](https://v2.tauri.app/start/migrate/from-tauri-1/)
