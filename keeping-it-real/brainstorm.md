# Keeping It Real — Brainstorm

A personal realtime event stream aggregation, filtering, and push notification proxy service.

Core concept: ingest every signal in your life → understand context → deliver only what matters, when it matters, the way it should arrive.

---

## Feature / Idea Tree

### 1. Universal Ingestion Layer
- **1a. Feed connectors** — RSS/Atom, email (IMAP/SMTP), SMS (via Android API or Twilio), calendar invites, webhooks
- **1b. App notification mirror** — Android Notification Listener Service captures all app notifications before they fire; raw stream into the pipeline
- **1c. Phone call metadata** — caller ID, duration, contact tags, voicemail transcripts (Whisper)
- **1d. Meeting transcription** — auto-join Google Meet / Zoom as a silent recorder; pipe transcript in realtime
- **1e. Screen/activity context** — foreground app detection, screen-on/off, DND status, GPS geofence tags (home, office, commute)

### 2. Context Engine (Situation Awareness)
- **2a. State inference** — derive current state (sleeping, commuting, in-meeting, reading, focused-work, socializing) from signals: calendar, phone motion, foreground app, time-of-day, location
- **2b. Manual override** — quick-tap widget or Siri/Assistant shortcut to say "I'm in a 2-hour focus block" or "family dinner until 8pm"
- **2c. Recurring pattern learning** — ML over your own history to predict state transitions ("you usually nap 1–3pm Sundays")
- **2d. Wearable integration** — heart rate / HRV from Wear OS / Fitbit to infer stress level or sleep stage

### 3. Priority & Routing Engine
- **3a. Rule-based filter** — declarative YAML/TOML rules: `if source=work-email AND keywords=["URGENT","prod down"] AND state!=sleeping → priority=critical`
- **3b. LLM summarization + scoring** — send each event through a small local LLM (Mistral 7B) or Claude API to score urgency, extract action items, and write a 1-line summary
- **3c. Dependency graph** — link events: a follow-up email to a meeting gets lower priority if the meeting transcript already covered it
- **3d. Quiet hours decay** — events that arrive while sleeping are re-ranked by morning; truly time-sensitive ones still wake you

### 4. Visual Timeline UI
- **4a. Web dashboard** — scrollable infinite timeline (React + D3 or canvas) showing all events color-coded by source and priority; tap to expand full content
- **4b. Swimlane view** — lanes per source category (comms, calendar, feeds, system); zoom from hour to month
- **4c. Heatmap summary** — daily/weekly heatmap of event volume and priority, useful for spotting noise sources
- **4d. Conversation threads** — stitch related events (email reply chain, SMS thread, meeting + follow-up) into a single collapsible thread in the timeline

### 5. Smart Push Notification Proxy (Android)
- **5a. Suppress-all + re-emit** — use Android's Notification Access + DND + a foreground service to cancel original notifications and re-post only approved ones under one unified channel
- **5b. Deep-link passthrough** — store original notification's `contentIntent` PendingIntent; tapping the proxy notification fires it, opening exactly the original app destination
- **5c. Batched digests** — group lower-priority notifications into a single digest card delivered at a natural break (end of focus block, calendar gap)
- **5d. Escalation ladder** — if a critical event is unacknowledged for N minutes, escalate: silent → sound → vibrate → phone call from the system

### 6. React Native Android App
- **6a. Notification Listener via native module** — bridge to `NotificationListenerService` in Kotlin; stream events over websocket or local gRPC to the backend
- **6b. Unified inbox widget** — home-screen widget (Expo WidgetKit / react-native-android-widget) showing top-3 pending items
- **6c. Gesture quick-actions** — swipe-left on proxy notification to snooze, swipe-right to mark done, long-press for full context
- **6d. Offline-first with sync** — SQLite local cache (op-log style) so the timeline works with no connection; sync when back online

### 7. Privacy & Storage Architecture
- **7a. Local-first backend** — all ingestion and processing runs on a home server or self-hosted VPS; no vendor has your data
- **7b. End-to-end encryption at rest** — events stored encrypted (libsodium secretbox); keys never leave the device
- **7c. Retention policies** — configurable per-source TTL (keep SMS 1 year, keep RSS forever, delete call metadata after 30 days)
- **7d. Audit log** — every filter decision (why was this suppressed?) is logged so you can tune rules and understand the system's reasoning

---

## Summary Map

```
Ingestion
├── RSS / email / SMS / webhooks
├── Android Notification Listener (native module)
├── Call metadata + voicemail transcription
└── Meeting transcription (auto-join bot)

Context Engine
├── State inference (calendar + location + app + motion)
├── Manual override widget
├── Pattern learning (personal ML model)
└── Wearable signals (sleep, HRV)

Priority Engine
├── Declarative rules (YAML)
├── LLM scoring + 1-line summary
├── Event dependency graph
└── Quiet-hours decay + re-rank

Timeline UI (web)
├── Infinite scroll, color-coded
├── Swimlane + zoom
├── Heatmap
└── Stitched conversation threads

Push Proxy (Android)
├── Suppress originals, re-emit filtered
├── Deep-link passthrough (original intent)
├── Batched digest cards
└── Escalation ladder

React Native App
├── Native Notification Listener bridge
├── Home-screen widget (top-3)
├── Gesture quick-actions on proxy notifications
└── Offline-first SQLite + sync

Privacy / Storage
├── Local-first / self-hosted
├── Encryption at rest
├── Per-source retention TTL
└── Filter audit log
```
