package com.keepingitreal.notification

import android.app.Notification
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import android.service.notification.NotificationListenerService
import android.service.notification.StatusBarNotification
import android.util.Base64
import android.util.Log
import com.keepingitreal.NotificationRecord
import com.keepingitreal.bridge.NotificationModule
import com.keepingitreal.db.NotificationDatabase
import java.io.ByteArrayOutputStream

class KIRNotificationListenerService : NotificationListenerService() {

    companion object {
        private const val TAG = "KIRNotifListener"
    }

    private lateinit var db: NotificationDatabase

    override fun onCreate() {
        super.onCreate()
        db = NotificationDatabase.getInstance(this)
        Log.d(TAG, "NotificationListenerService started")
    }

    override fun onNotificationPosted(sbn: StatusBarNotification) {
        val notification = sbn.notification ?: return
        val extras = notification.extras

        val title = extras.getString(Notification.EXTRA_TITLE)
        val text  = extras.getCharSequence(Notification.EXTRA_TEXT)?.toString()
            ?: extras.getCharSequence(Notification.EXTRA_BIG_TEXT)?.toString()

        // Skip invisible/internal notifications with no user-visible content
        if (title == null && text == null) return

        val packageName = sbn.packageName
        val key = "${packageName}|${sbn.id}|${sbn.postTime}"

        val appName = try {
            packageManager
                .getApplicationLabel(packageManager.getApplicationInfo(packageName, 0))
                .toString()
        } catch (e: Exception) {
            packageName
        }

        val iconBase64 = try {
            val icon = notification.smallIcon?.loadDrawable(this)
                ?: packageManager.getApplicationIcon(packageName)
            val w = icon.intrinsicWidth.coerceIn(1, 128)
            val h = icon.intrinsicHeight.coerceIn(1, 128)
            val bmp = if (icon is BitmapDrawable) {
                icon.bitmap
            } else {
                Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888).also { b ->
                    val canvas = Canvas(b)
                    icon.setBounds(0, 0, w, h)
                    icon.draw(canvas)
                }
            }
            ByteArrayOutputStream().use { out ->
                bmp.compress(Bitmap.CompressFormat.PNG, 100, out)
                Base64.encodeToString(out.toByteArray(), Base64.NO_WRAP)
            }
        } catch (e: Exception) {
            Log.w(TAG, "Failed to encode icon for $packageName: ${e.message}")
            null
        }

        // Store the live PendingIntent — only possible while service is running
        notification.contentIntent?.let { PendingIntentStore.put(key, it) }

        val record = NotificationRecord(
            key              = key,
            packageName      = packageName,
            appName          = appName,
            title            = title,
            text             = text,
            postTime         = sbn.postTime,
            appIconBase64    = iconBase64,
            hasPendingIntent = notification.contentIntent != null
        )

        db.insert(record)

        // Push realtime event to any open React Native screen
        NotificationModule.emitNotification(applicationContext, record)
    }

    override fun onNotificationRemoved(sbn: StatusBarNotification) {
        val key = "${sbn.packageName}|${sbn.id}|${sbn.postTime}"
        // Remove from live store; DB record is kept for history
        PendingIntentStore.remove(key)
    }
}
