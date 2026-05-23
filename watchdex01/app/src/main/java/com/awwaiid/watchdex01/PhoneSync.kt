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

/**
 * Ships a recorded file to the paired phone over the Wear Data Layer.
 *
 * Path: /watchdex01/audio
 * Payload: Asset "audio" (raw bytes), String "filename", Long "recordedAt".
 *
 * A WearableListenerService on the phone (in the Pebble app, or a small
 * forwarder APK) is responsible for picking the asset up and handing it to
 * whatever pipeline the Pebble app uses.
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
            val req = PutDataMapRequest.create(AUDIO_PATH).apply {
                dataMap.putAsset("audio", asset)
                dataMap.putString("filename", file.name)
                dataMap.putLong("recordedAt", System.currentTimeMillis())
            }.asPutDataRequest().setUrgent()
            Tasks.await(dataClient.putDataItem(req))
            true
        } catch (e: Exception) {
            false
        }
    }

    companion object {
        const val AUDIO_PATH = "/watchdex01/audio"
    }
}
