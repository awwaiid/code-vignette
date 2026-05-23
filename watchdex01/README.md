# watchdex01

A Wear OS app that turns a Google watch into a push-to-talk capture device,
mimicking the pebble01 / index01 ring flow: **hold the side button → record →
release → ship audio to the paired phone**.

## What it does

- Foreground activity intercepts the watch's physical stem button
  (`KEYCODE_STEM_PRIMARY`, plus `STEM_1..3` for watches with extras).
- While the button is held, `MediaRecorder` captures **mono AAC, 16 kHz,
  64 kbps** into the app's cache directory (`rec-<timestamp>.m4a`).
- On release, the file is published to the paired phone over the Wear OS
  **Data Layer** as an `Asset` at a unique path `/watchdex01/audio/<uuid>`
  (so back-to-back recordings don't overwrite each other in the cache).
- The screen is held on while the app is foregrounded so the OS doesn't kill
  recording mid-hold.

## Receiving the audio on the phone

The watch hands audio to the **paired phone** over the Wear Data Layer; the
Pebble app needs a small ingest patch to pick it up. That patch lives in
[`pebble-patch/`](pebble-patch/) — a `WearableListenerService` + AAC→PCM
decoder + cosmetic device-list entry that injects watch audio into the same
`RingTransferRepository` / `RecordingStorage` / `recordingProcessingQueue`
the real Index 01 ring uses. Recordings appear in the Pebble app's feed
identically to ring captures.

Apply instructions and trade-offs in [`pebble-patch/README.md`](pebble-patch/README.md).

> **Why not full BLE ring impersonation?** The Index 01 ring's BLE protocol
> lives in the closed-source `io.github.coredevices.haversine:haversine` jar.
> Without that source, the watch can't speak the ring's wire protocol to an
> unmodified Pebble app. Injecting at the recording-pipeline layer in your
> Pebble fork is the path that actually works.

## Build locally

Requires JDK 17 and the Android SDK (`platforms;android-35`,
`build-tools;35.0.0`; AGP also auto-installs `build-tools;34.0.0` on first
build for desugaring). Wrapper is checked in:

```sh
cd watchdex01
./gradlew assembleDebug
```

Output APK: `app/build/outputs/apk/debug/app-debug.apk`. About 7 MB.

If `sdk.dir` isn't already configured, drop a `local.properties` next to
`gradlew` (it's `.gitignore`d) pointing at your SDK:

```
sdk.dir=/path/to/Android/Sdk
```

## CI build

GitHub Actions: [`.github/workflows/watchdex01.yml`](../.github/workflows/watchdex01.yml).
Every push touching `watchdex01/` builds a debug APK and uploads it as an
artifact named `watchdex01-debug-apk`. Download it from the workflow run
page and skip straight to sideloading.

## Sideloading onto your watch

Wear OS doesn't have a one-tap installer for unsigned APKs — you need ADB. Pick
the path that matches your watch:

### One-time: enable developer mode on the watch

1. Watch → **Settings → System → About → Versions** → tap **Build number** 7×.
2. Back up one menu → **Developer options**.
3. Turn on **ADB debugging**. If the watch has no USB port (most don't), also
   turn on **Wireless debugging** (Pixel Watch / Galaxy Watch) **or**
   **Debug over Wi-Fi** (older Wear OS).

### Connect ADB

**Wireless debugging (Pixel Watch, modern Galaxy Watch)**

1. On the watch, tap **Wireless debugging → Pair new device**. Note the IP +
   pairing port + 6-digit code.
2. On your computer:

   ```sh
   adb pair <watch-ip>:<pair-port>      # paste the 6-digit code when prompted
   adb connect <watch-ip>:<connect-port>
   ```

   The connect port is the larger one shown on the Wireless debugging screen
   (not the pairing port).

**Debug over Wi-Fi (legacy)**

1. Pair the watch with the phone's Wear OS app on the same Wi-Fi.
2. Watch shows an IP. From your computer:

   ```sh
   adb connect <watch-ip>:5555
   ```

### Install

```sh
adb -s <watch-ip>:<port> install -r app/build/outputs/apk/debug/app-debug.apk
```

The watch will prompt to **Allow USB debugging** for your host's RSA
fingerprint — accept it. Re-run the install once you've accepted.

### Run it

1. Open the watch app drawer → **watchdex01**.
2. First launch: grant the **microphone** permission (and notifications on
   Android 13+).
3. Hold the side button. Status flips to **Recording…** until you release.
4. Status then shows either **Sent N KB** (phone reached) or
   **Saved (no phone) N KB** (no Data Layer node available — recording is
   still on the watch in app cache).

## Known caveats

- **Short presses of the side button may exit the app on some watches** — the
  OS routes a tap-up of `KEYCODE_STEM_PRIMARY` to the watchface. Hold the
  button instead of tapping; key-down still fires and recording starts. If a
  specific watch swallows the event entirely, the workaround is to use
  `STEM_1`/`STEM_2` on watches that have them.
- **The phone-side bridge lives in [`pebble-patch/`](pebble-patch/)** —
  apply it to your fork of [`awwaiid/pebble-mobileapp`](https://github.com/awwaiid/pebble-mobileapp)
  to actually receive the audio. Without the patch (or a forwarder), the
  watch will say *Sent N KB* (delivered to the Wear Data Layer) but
  nothing on the phone consumes it.
- Recording happens from the activity, not a foreground service. If the
  screen sleeps mid-hold the recorder is stopped; this is fine for
  push-to-talk-style use because the screen stays on while the activity is
  visible.
