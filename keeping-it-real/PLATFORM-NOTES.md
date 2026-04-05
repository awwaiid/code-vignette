# Platform Notes — Keeping It Real

Notes on what's possible for notification capture across platforms.

---

## Android

**Status: fully implemented** — see `app/`

Android's `NotificationListenerService` is a first-class, user-grantable system permission.
Once granted, every notification from every app flows through our service with full content
(title, body, app name, icon) and a live `PendingIntent` for deep-link passthrough.

**Deep-link passthrough:** `contentIntent.send()` fires exactly what tapping the original
notification would have done. Lost on process death (PendingIntents aren't serializable) —
rows without a live intent are shown dimmed.

**Permission model:** No runtime permission request. User grants via
Settings → Special App Access → Notification Access.

---

## macOS (laptop)

**Status: planned**

macOS is the easiest capture target — no native module needed at all.

### SQLite database (primary approach)

macOS stores all delivered notifications in a user-owned SQLite database:

```
~/Library/Application Support/com.apple.notificationcenter/db2/db
```

- No special entitlements; readable by any process running as the logged-in user
- Table: `record` — columns include `app_id`, `uuid`, `data` (NSKeyedArchiver binary plist),
  `delivered_date`
- Watch for new rows with `FSEvents` or periodic polling
- Decode the `data` blob: `plutil -convert json -o - <(xxd -r -p <hex>)` or a plist library

**Caveats:**
- Schema changed between macOS versions — needs version detection
  - macOS Monterey and earlier: above path
  - macOS Ventura+: same path but schema evolved; some fields moved inside the plist blob
- No `PendingIntent` equivalent — content only, no deep-link into originating app

### Other macOS avenues

- **NSDistributedNotificationCenter** — observe system-wide named notifications without any
  permission (network state, app lifecycle, iTunes/Music track changes, etc.). Good for
  context engine signals, not for per-app user notifications.
- **Accessibility API** — with Accessibility permission, can read Notification Center UI elements.
  Fragile (depends on UI structure), but can supplement the DB approach.
- **Endpoint Security Framework** — requires Apple entitlement
  (`com.apple.developer.endpoint-security.client`). Monitors process launches, file access,
  network connections. Useful for context engine (what apps are running, what the user is doing)
  rather than notification content.

### Implementation plan

A small Node.js or Python daemon:
1. Opens the SQLite DB (read-only)
2. Watches for new rows via polling or FSEvents
3. Decodes NSKeyedArchiver plist blobs
4. POSTs normalized `NotificationRecord` JSON to the same local backend as Android

---

## iOS

**Status: not feasible for App Store distribution**

iOS has no equivalent to Android's `NotificationListenerService`. This is enforced at the OS
level, not just a missing API. Apple's sandbox prevents any app from reading another app's
notification content.

**What iOS CAN do (own notifications only):**
- `UNNotificationServiceExtension` — intercept your own incoming push notifications before
  display (decrypt, enrich, modify). 30-second time limit.
- `UNUserNotificationCenterDelegate` — handle taps on your own notifications while app is
  in foreground.
- Screen Time API (`FamilyControls`) — block apps/sites, but deliberately exposes zero
  notification content (anonymized only, requires special Apple entitlement).
- Focus Filters — told when a Focus mode is active, can adjust own behavior only.

**Jailbreak:** SpringBoard tweaks (NotificationHub, etc.) can intercept all notifications,
but not shippable via App Store and fragile on iOS 17/18+.

**Will Apple open this up?** No indication. WWDC 2025 (iOS 26) added AlarmKit, expanded
Live Activities, SMS spam filtering — nothing cross-app. Apple Intelligence does cross-app
notification summarization but in Apple's privileged system process, not accessible to devs.

**iOS companion app scope (if built):** Would need to be scoped to managing own notifications,
Focus modes, or screen time — not a notification mirror.

---

## Multi-device sync (future)

Planned architecture for connecting phone, laptop, and optionally a cloud instance:

```
Android phone  ──────┐
                      ├──► local backend (events DB + HTTP API)
macOS laptop   ──────┘         │
                                ├──► web timeline UI
                                └──► sync to other devices via QR-code pairing
```

**QR-code pairing idea:**
- Device A runs backend, shows QR code containing its local IP + a session token
- Device B scans QR, establishes WebSocket connection, begins syncing event stream
- No cloud account required for local-network pairing
- Optional: a minimal relay server for when devices are on different networks

**Data sovereignty:**
- All storage is local-first (SQLite on each device)
- Events replicated via op-log sync (each event has a UUID + timestamp; merge = union)
- Encryption at rest: libsodium secretbox; keys never leave the device
- Cloud relay (if used) sees only encrypted blobs

---

## Event data model (canonical, cross-platform)

```typescript
interface NotificationRecord {
  id: string;            // UUID, globally unique
  source: 'android' | 'macos';
  deviceId: string;      // stable per-device identifier
  packageName: string;   // e.g. com.whatsapp, com.tinyspeck.slackmacgap
  appName: string;
  title: string | null;
  text: string | null;
  postTime: number;      // epoch ms
  appIconBase64: string | null;
  hasPendingIntent: boolean;  // Android only; always false on macOS
}
```
