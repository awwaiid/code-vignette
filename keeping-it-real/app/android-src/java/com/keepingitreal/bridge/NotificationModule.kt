package com.keepingitreal.bridge

import android.content.Context
import android.provider.Settings
import androidx.core.app.NotificationManagerCompat
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule
import com.keepingitreal.NotificationRecord
import com.keepingitreal.db.NotificationDatabase
import com.keepingitreal.notification.PendingIntentStore
import java.lang.ref.WeakReference

class NotificationModule(private val reactContext: ReactApplicationContext) :
    ReactContextBaseJavaModule(reactContext) {

    companion object {
        private const val EVENT_NOTIFICATION_POSTED = "onNotificationPosted"

        // Weak reference so the service can emit events without leaking the context
        private var weakContext: WeakReference<ReactApplicationContext>? = null

        fun emitNotification(appContext: Context, record: NotificationRecord) {
            val rctContext = weakContext?.get() ?: return
            if (!rctContext.hasActiveReactInstance()) return
            val params = recordToMap(record)
            rctContext
                .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter::class.java)
                ?.emit(EVENT_NOTIFICATION_POSTED, params)
        }

        private fun recordToMap(record: NotificationRecord): WritableMap =
            Arguments.createMap().apply {
                putString("key",             record.key)
                putString("packageName",     record.packageName)
                putString("appName",         record.appName)
                putString("title",           record.title)
                putString("text",            record.text)
                putDouble("postTime",        record.postTime.toDouble())
                putString("appIconBase64",   record.appIconBase64)
                putBoolean("hasPendingIntent", record.hasPendingIntent)
            }
    }

    override fun getName(): String = "NotificationModule"

    override fun initialize() {
        super.initialize()
        weakContext = WeakReference(reactContext)
    }

    override fun invalidate() {
        super.invalidate()
        weakContext = null
    }

    /** Returns all stored notifications, most-recent first. */
    @ReactMethod
    fun getNotifications(promise: Promise) {
        try {
            val db = NotificationDatabase.getInstance(reactContext)
            val records = db.getAll()
            val array = Arguments.createArray()
            for (record in records) array.pushMap(recordToMap(record))
            promise.resolve(array)
        } catch (e: Exception) {
            promise.reject("DB_ERROR", e.message, e)
        }
    }

    /**
     * Fire the original PendingIntent for a notification.
     * Returns true if fired successfully, false if the intent was not found or was cancelled.
     */
    @ReactMethod
    fun firePendingIntent(key: String, promise: Promise) {
        val intent = PendingIntentStore.get(key)
        if (intent == null) {
            promise.resolve(false)
            return
        }
        try {
            intent.send()
            PendingIntentStore.remove(key)
            promise.resolve(true)
        } catch (e: android.app.PendingIntent.CanceledException) {
            PendingIntentStore.remove(key)
            promise.resolve(false)
        } catch (e: Exception) {
            promise.reject("INTENT_ERROR", e.message, e)
        }
    }

    /**
     * Returns true if this app has been granted Notification Listener access.
     * If false, the UI should prompt the user to open Settings.
     */
    @ReactMethod
    fun isPermissionGranted(promise: Promise) {
        val enabled = NotificationManagerCompat
            .getEnabledListenerPackages(reactContext)
            .contains(reactContext.packageName)
        promise.resolve(enabled)
    }

    // Required stubs for NativeEventEmitter compatibility
    @ReactMethod fun addListener(eventName: String) {}
    @ReactMethod fun removeListeners(count: Int) {}
}
