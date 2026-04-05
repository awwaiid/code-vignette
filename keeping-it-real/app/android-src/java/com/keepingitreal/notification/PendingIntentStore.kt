package com.keepingitreal.notification

import android.app.PendingIntent
import java.util.concurrent.ConcurrentHashMap

/**
 * Process-scoped singleton holding live PendingIntents captured from notifications.
 *
 * PendingIntents cannot be serialized or persisted — they exist only while the
 * originating process is alive. This store is populated by KIRNotificationListenerService
 * and consumed by NotificationModule (RN bridge). Both live in the same process, so
 * no IPC is needed.
 *
 * IMPORTANT: Do NOT add `:remote` to the service's AndroidManifest entry — that would
 * put it in a separate process and break this shared-memory design entirely.
 *
 * If the app process is killed and restarted, all entries are lost. The UI reflects this
 * via hasPendingIntent=false on records loaded from SQLite after a restart.
 */
object PendingIntentStore {

    private const val MAX_SIZE = 500

    // key -> PendingIntent, written by service thread, read by bridge thread
    private val store = ConcurrentHashMap<String, PendingIntent>(MAX_SIZE)

    // Insertion order tracking for LRU-style eviction when cap is hit
    private val insertionOrder = ArrayDeque<String>(MAX_SIZE)
    private val evictionLock = Any()

    fun put(key: String, intent: PendingIntent) {
        synchronized(evictionLock) {
            if (store.size >= MAX_SIZE) {
                val oldest = insertionOrder.removeFirstOrNull()
                if (oldest != null) store.remove(oldest)
            }
            store[key] = intent
            insertionOrder.addLast(key)
        }
    }

    fun get(key: String): PendingIntent? = store[key]

    fun remove(key: String) {
        synchronized(evictionLock) {
            store.remove(key)
            insertionOrder.remove(key)
        }
    }

    fun contains(key: String): Boolean = store.containsKey(key)

    fun size(): Int = store.size
}
