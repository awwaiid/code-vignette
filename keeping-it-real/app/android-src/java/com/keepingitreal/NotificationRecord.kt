package com.keepingitreal

data class NotificationRecord(
    val key: String,               // "$packageName|$id|$postTime" — stable lookup handle
    val packageName: String,
    val appName: String,
    val title: String?,
    val text: String?,
    val postTime: Long,            // epoch ms
    val appIconBase64: String?,    // PNG bytes as base64; null if icon unavailable
    val hasPendingIntent: Boolean  // computed at runtime from PendingIntentStore, not persisted
)
