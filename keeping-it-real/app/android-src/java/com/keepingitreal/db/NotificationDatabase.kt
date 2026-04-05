package com.keepingitreal.db

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import com.keepingitreal.NotificationRecord
import com.keepingitreal.notification.PendingIntentStore

private const val DB_NAME    = "kir_notifications.db"
private const val DB_VERSION = 1
private const val TABLE      = "notifications"

class NotificationDatabase private constructor(context: Context) :
    SQLiteOpenHelper(context.applicationContext, DB_NAME, null, DB_VERSION) {

    companion object {
        @Volatile private var instance: NotificationDatabase? = null

        fun getInstance(context: Context): NotificationDatabase =
            instance ?: synchronized(this) {
                instance ?: NotificationDatabase(context).also { instance = it }
            }
    }

    override fun onCreate(db: SQLiteDatabase) {
        db.execSQL("""
            CREATE TABLE $TABLE (
                key          TEXT PRIMARY KEY,
                package_name TEXT NOT NULL,
                app_name     TEXT NOT NULL,
                title        TEXT,
                text         TEXT,
                post_time    INTEGER NOT NULL,
                app_icon     TEXT
            )
        """.trimIndent())
        db.execSQL("CREATE INDEX idx_post_time ON $TABLE (post_time DESC)")
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        db.execSQL("DROP TABLE IF EXISTS $TABLE")
        onCreate(db)
    }

    /** Insert or silently ignore duplicates (same key = same notification posted twice). */
    fun insert(record: NotificationRecord) {
        val values = ContentValues().apply {
            put("key",          record.key)
            put("package_name", record.packageName)
            put("app_name",     record.appName)
            put("title",        record.title)
            put("text",         record.text)
            put("post_time",    record.postTime)
            put("app_icon",     record.appIconBase64)
        }
        writableDatabase.insertWithOnConflict(TABLE, null, values, SQLiteDatabase.CONFLICT_IGNORE)
    }

    /**
     * Returns all notifications, most-recent first.
     * hasPendingIntent is computed live from PendingIntentStore.
     */
    fun getAll(): List<NotificationRecord> {
        val results = mutableListOf<NotificationRecord>()
        readableDatabase.query(
            TABLE,
            arrayOf("key", "package_name", "app_name", "title", "text", "post_time", "app_icon"),
            null, null, null, null,
            "post_time DESC"
        ).use { cursor ->
            while (cursor.moveToNext()) {
                val key = cursor.getString(0)
                results += NotificationRecord(
                    key             = key,
                    packageName     = cursor.getString(1),
                    appName         = cursor.getString(2),
                    title           = cursor.getString(3),
                    text            = cursor.getString(4),
                    postTime        = cursor.getLong(5),
                    appIconBase64   = cursor.getString(6),
                    hasPendingIntent = PendingIntentStore.contains(key)
                )
            }
        }
        return results
    }
}
