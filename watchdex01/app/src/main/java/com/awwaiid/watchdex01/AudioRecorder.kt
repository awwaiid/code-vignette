package com.awwaiid.watchdex01

import android.content.Context
import android.media.MediaRecorder
import android.os.Build
import java.io.File

class AudioRecorder(private val context: Context) {

    private var recorder: MediaRecorder? = null
    private var currentFile: File? = null

    fun start(): Boolean {
        if (recorder != null) return false
        val outFile = File(context.cacheDir, "rec-${System.currentTimeMillis()}.m4a")
        val r = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            MediaRecorder(context)
        } else {
            @Suppress("DEPRECATION")
            MediaRecorder()
        }
        return try {
            r.setAudioSource(MediaRecorder.AudioSource.MIC)
            r.setOutputFormat(MediaRecorder.OutputFormat.MPEG_4)
            r.setAudioEncoder(MediaRecorder.AudioEncoder.AAC)
            r.setAudioSamplingRate(16_000)
            r.setAudioChannels(1)
            r.setAudioEncodingBitRate(64_000)
            r.setOutputFile(outFile.absolutePath)
            r.prepare()
            r.start()
            recorder = r
            currentFile = outFile
            true
        } catch (e: Exception) {
            runCatching { r.release() }
            outFile.delete()
            false
        }
    }

    fun stop(): File? {
        val r = recorder ?: return null
        val f = currentFile
        recorder = null
        currentFile = null
        return try {
            r.stop()
            r.release()
            f
        } catch (e: Exception) {
            runCatching { r.release() }
            f?.delete()
            null
        }
    }
}
