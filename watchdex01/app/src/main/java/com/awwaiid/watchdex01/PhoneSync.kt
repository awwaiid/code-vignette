package com.awwaiid.watchdex01

import android.content.Context
import com.google.android.gms.tasks.Tasks
import com.google.android.gms.wearable.Asset
import com.google.android.gms.wearable.PutDataMapRequest
import com.google.android.gms.wearable.Wearable
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileInputStream
import java.util.UUID

/**
 * Ships a recorded file to the paired phone over the Wear Data Layer.
 *
 * Each recording is sent at a unique path /watchdex01/audio/<transferId> so
 * rapid successive recordings don't overwrite one another in the Data Layer
 * cache. The phone-side listener is expected to delete each DataItem after
 * it has ingested it.
 *
 * Payload:
 *   Asset  "audio"      – AAC-in-MP4 bytes
 *   String "filename"   – original filename from the watch
 *   String "transferId" – UUID, also embedded in the path
 *   String "mimeType"   – "audio/mp4" (AAC LC, mono, 16 kHz, 64 kbps)
 *   Long   "recordedAt" – ms since epoch when recording finished
 */
class PhoneSync(context: Context) {

    private val dataClient = Wearable.getDataClient(context)
    private val nodeClient = Wearable.getNodeClient(context)

    suspend fun sendAudio(file: File): Boolean = withContext(Dispatchers.IO) {
        try {
            val nodes = Tasks.await(nodeClient.connectedNodes)
            if (nodes.isEmpty()) return@withContext false

            val bytes = FileInputStream(file).use { it.readBytes() }
            val asset = Asset.createFromBytes(bytes)
            val transferId = UUID.randomUUID().toString()
            val req = PutDataMapRequest.create("$AUDIO_PATH_PREFIX/$transferId").apply {
                dataMap.putAsset("audio", asset)
                dataMap.putString("filename", file.name)
                dataMap.putString("transferId", transferId)
                dataMap.putString("mimeType", "audio/mp4")
                dataMap.putLong("recordedAt", System.currentTimeMillis())
            }.asPutDataRequest().setUrgent()
            Tasks.await(dataClient.putDataItem(req))
            true
        } catch (e: Exception) {
            false
        }
    }

    companion object {
        const val AUDIO_PATH_PREFIX = "/watchdex01/audio"
    }
}
