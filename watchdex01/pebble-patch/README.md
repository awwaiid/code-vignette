# Pebble fork patch — watchdex01 ingest

Patch for [`awwaiid/pebble-mobileapp`](https://github.com/awwaiid/pebble-mobileapp)
that makes the Pebble Android app ingest audio captured by the
[`watchdex01`](../) Wear OS app.

A `WearableListenerService` receives the watch's recordings from the Wear OS
Data Layer, decodes the AAC payload to 16-bit LE PCM, and injects it into the
**same** code path the real Index 01 ring uses post-`TransferComplete` —
`RingTransferRepository` + `RecordingStorage` + `recordingProcessingQueue`.
The recording shows up in the app's feed identically to a ring capture and is
transcribed / uploaded by the existing pipeline.

The watch also gets a cosmetic entry in the Devices list (a
`KnownIndexDevice` registered via `IndexDeviceManager.update(...)`). It does
**not** participate in BLE pairing or claim the `prefs.ringPaired` slot, so
your real ring continues to work alongside it.

## Why this isn't full ring impersonation

The Index 01 ring's BLE protocol — service characteristics, advertisement
manufacturer-data layout, audio framing — lives entirely inside the
closed-source `io.github.coredevices.haversine:haversine` jar. Without that
source, the watch can't speak the ring's BLE wire protocol to the unmodified
Pebble app. This patch sidesteps that entirely by injecting at a higher layer
in the same app, post-decode.

## What this patch adds

Three new files (already laid out under `composeApp/src/androidMain/kotlin/coredevices/watchdex/` in this directory — copy them in as-is):

- `WatchdexListenerService.kt` — `WearableListenerService` that watches
  `wear://*/watchdex01/audio/*`, performs ingest, and registers the cosmetic
  device entry. Uses Koin's `org.koin.android.ext.android.inject` to grab
  `RingTransferRepository`, `RecordingStorage`, `RecordingProcessingQueue`,
  and `IndexDeviceManager` from the existing global context — no new Koin
  bindings required.
- `WatchdexAudioDecoder.kt` — `MediaExtractor` + `MediaCodec` based AAC→PCM
  decoder. Standalone Android; no fork-specific deps.
- `WatchdexIndexDevice.kt` — minimal `KnownIndexDevice` implementation for
  the Devices-list entry. `remove()` is a no-op so it doesn't clobber
  `prefs.ringPaired`.

Two small edits to existing files (snippets in `diffs/`):

- `composeApp/src/androidMain/AndroidManifest.xml` — register the service.
- `composeApp/build.gradle.kts` + `gradle/libs.versions.toml` — add
  `play-services-wearable`.

## Apply

From the root of your `pebble-mobileapp` checkout:

```sh
# 1. Drop in the new source files
PATCH=/path/to/code-vignette/watchdex01/pebble-patch
cp -r "$PATCH/composeApp/src/androidMain/kotlin/coredevices/watchdex" \
      composeApp/src/androidMain/kotlin/coredevices/

# 2. Manifest — paste diffs/composeApp-AndroidManifest.xml.snippet inside
#    <application>...</application>
$EDITOR composeApp/src/androidMain/AndroidManifest.xml

# 3. Gradle — add play-services-wearable per
#    diffs/composeApp-build.gradle.kts.snippet
$EDITOR composeApp/build.gradle.kts
$EDITOR gradle/libs.versions.toml

# 4. Build + install on the same phone that's paired with your watch
./gradlew :composeApp:assembleDebug
adb install -r composeApp/build/outputs/apk/debug/composeApp-debug.apk
```

## End-to-end test

1. Sideload `watchdex01-debug-apk` onto the watch (see `../README.md`).
2. Install this patched Pebble app on the **same** phone that's the watch's
   Wear OS companion.
3. Open both. Hold the watch's side button → release.
4. Within a couple of seconds the recording should land in the Pebble app's
   feed and start transcribing, and "watchdex01" should appear under
   Devices.

## Things that *probably* need adjustment in your fork

These can't be confirmed from the open-source slice:

- **`recordingProcessingQueue.queueAudioProcessing(transferId, buttonSequence)`** —
  the type of `buttonSequence` isn't visible (it comes from haversine's
  `TransferStatus.TransferTypeDetermined`). The patch passes `null`, mirroring
  what `RingSync` does when `collectionStartIndex != null && !final`. If your
  fork's signature insists on non-null, supply a sentinel value or extend the
  queue API to accept the watch as a known sourceless variant.
- **Resampling** — the watch records at exactly 16 kHz mono, which matches
  `TARGET_SAMPLE_RATE`. If you ever change the watch's `setAudioSamplingRate`
  away from 16 kHz, plumb the watch's decoded `sampleRate` through
  `coredevices.resampler.Resampler` before writing to `openRecordingSink`,
  mirroring `RingSync`'s `resample(...)` helper.
- **Cosmetic device entry persistence** — `IndexDeviceManager._rings` is
  in-memory only; the watch entry appears after the first recording is
  ingested in a given app launch and is dropped on process death. If you
  want it to survive restarts, store a `wasEverSeen` flag in
  `BasePreferences` and re-register on `IndexDeviceManager.init()`.

## Files

```
pebble-patch/
├── composeApp/src/androidMain/kotlin/coredevices/watchdex/
│   ├── WatchdexAudioDecoder.kt
│   ├── WatchdexIndexDevice.kt
│   └── WatchdexListenerService.kt
├── diffs/
│   ├── composeApp-AndroidManifest.xml.snippet
│   └── composeApp-build.gradle.kts.snippet
└── README.md                ← you are here
```
