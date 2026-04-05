# Keeping It Real — Android App

A personal notification archive. Captures every notification on your Android device,
stores it locally, shows a chronological timeline, and lets you tap any entry to
fire the original app's deep link — exactly what tapping the real notification would have done.

## Architecture

```
Android OS
  └─ NotificationListenerService (Kotlin)
       ├─ KIRNotificationListenerService  — captures all posted notifications
       ├─ PendingIntentStore              — holds live PendingIntents in memory
       └─ NotificationDatabase           — SQLite history (write path)

React Native Bridge (Kotlin)
  └─ NotificationModule                  — exposes getNotifications(), firePendingIntent(),
                                           isPermissionGranted() to JS; emits realtime events

React Native App (TypeScript)
  ├─ NativeNotificationModule.ts         — typed JS wrapper for the bridge
  ├─ NotificationListScreen.tsx          — FlatList of all notifications
  └─ NotificationRow.tsx                 — single row; tap fires deep link
```

**Key design note:** The service and the RN bridge share the same process, so
`PendingIntentStore` is a simple in-memory singleton — no IPC needed. PendingIntents
are lost on process death (this is unavoidable; Android does not allow serializing them).
Rows without a live intent are shown dimmed and non-tappable.

## Setup

### 1. Create an Expo account and project

```bash
npm install -g eas-cli
eas login                        # create account at expo.dev if needed
eas init --id                    # creates the project on Expo's servers, fills in app.json
```

Copy the generated `projectId` into `app.json` → `expo.extra.eas.projectId`.

### 2. Bootstrap the native Android project

```bash
npm install
npx expo prebuild --platform android --no-install
```

This generates the `android/` directory from `app.json`. Run it once — after that, the
`android/` directory is yours to modify directly.

### 3. Add the custom native module

Copy `android-src/java/com/keepingitreal/` into `android/app/src/main/java/com/keepingitreal/`.

Then apply the two small patches:
- `android-src/AndroidManifest.patch.xml` — add the `<service>` entry inside `<application>`
- `android-src/MainApplication.patch.kt` — register `NotificationPackage`

### 4. Build a development APK (one-time, runs on Expo's servers)

```bash
eas build --platform android --profile development
```

This takes ~5-10 minutes. When done, EAS shows a QR code — scan it to install the APK.

### 5. Start the local dev server

```bash
npx expo start
```

Scan the Metro QR code with the installed development build. From here on, JS changes
reload instantly without rebuilding the APK.

### 6. Grant notification permission

On first launch, tap the orange banner → opens **Settings > Special App Access > Notification Access**.
Enable **Keeping It Real**. Return to the app — notifications stream in in realtime.

---

## CI/CD — GitHub Actions

Push to `main` (or trigger manually) → `.github/workflows/eas-build.yml` kicks off an EAS build.

**Required secret:** Add `EXPO_TOKEN` to the repo's GitHub Secrets
(get it at expo.dev → Account Settings → Access Tokens).

The workflow posts a build URL to the Actions summary. The EAS build page shows:
- A QR code to scan for direct APK install
- A download link for sideloading

**Manual trigger:** Go to Actions → EAS Build → Run workflow → pick `development` or `preview`.

## How deep-link passthrough works

When Android posts a notification, each one carries a `contentIntent: PendingIntent` —
a token that, when sent, tells the originating app to open a specific screen or action.

This app:
1. Captures `contentIntent` in `onNotificationPosted` and stores it in `PendingIntentStore`
2. When you tap a row, calls `pendingIntent.send()` — Android delivers the intent to the
   originating app, which opens exactly the screen it would have if you'd tapped the real notification
3. After firing, removes the intent from the store (can't fire twice)

**Limitation:** PendingIntents are alive only while the originating app's process is running.
After a phone restart or after the originating app is killed, the intent expires and the row
will show as non-tappable. The notification is still in your history — you just can't deep-link into it.

## Data model

| Field | Type | Notes |
|---|---|---|
| `key` | string | `"$packageName|$id|$postTime"` — unique per notification |
| `packageName` | string | e.g. `com.whatsapp` |
| `appName` | string | Human-readable app label |
| `title` | string? | Notification title |
| `text` | string? | Notification body |
| `postTime` | number | Epoch ms |
| `appIconBase64` | string? | PNG encoded as base64 |
| `hasPendingIntent` | boolean | Live; not persisted to DB |

## What's next

- **Filtering / priority rules** — tag notifications by source, keyword, app category
- **Context engine** — infer current state (meeting, sleeping, commuting) to surface what matters
- **Multi-device sync** — QR-code pairing between phone, laptop, cloud instance
- **Suppress originals + re-emit** — become the sole push notification proxy
- **Visual timeline** — web dashboard with swimlanes, heatmap, threaded conversations
