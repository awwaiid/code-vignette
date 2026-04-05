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

### 1. Bootstrap the React Native project

```bash
npx react-native@0.76 init KeepingItReal --template react-native-template-typescript
cd KeepingItReal
```

### 2. Copy custom source files

Copy the contents of `android-src/java/com/keepingitreal/` into:
```
android/app/src/main/java/com/keepingitreal/
```

Copy the contents of `src/` into the project's `src/` directory.

Replace `App.tsx` and `index.js` at the project root.

### 3. Patch AndroidManifest.xml

See `android-src/AndroidManifest.patch.xml` — add the `<service>` entry inside `<application>`.

### 4. Patch MainApplication.kt

See `android-src/MainApplication.patch.kt` — import `NotificationPackage` and add it to `getPackages()`.

### 5. Build and run

```bash
npm install
npx react-native run-android
```

### 6. Grant permission

On first launch, tap the orange banner → this opens **Settings > Special App Access > Notification Access**.
Enable **Keeping It Real**. Return to the app — notifications will now stream in.

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
