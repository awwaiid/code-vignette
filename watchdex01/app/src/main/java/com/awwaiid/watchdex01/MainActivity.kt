package com.awwaiid.watchdex01

import android.Manifest
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.view.WindowManager
import android.widget.TextView
import androidx.activity.ComponentActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private lateinit var statusText: TextView
    private lateinit var recorder: AudioRecorder
    private lateinit var phoneSync: PhoneSync
    private var isRecording = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        statusText = findViewById(R.id.status_text)
        recorder = AudioRecorder(this)
        phoneSync = PhoneSync(this)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

        // Touch-and-hold anywhere on the screen is the guaranteed trigger.
        // Physical buttons are still honored via onKeyDown/onKeyUp below when
        // the OS doesn't intercept them (the Pixel Watch's crown long-press is
        // reserved for Gemini and can't be overridden — use STEM_1 or the
        // screen instead).
        findViewById<View>(R.id.touch_target).setOnTouchListener { _, event ->
            when (event.actionMasked) {
                MotionEvent.ACTION_DOWN -> { startRecording(); true }
                MotionEvent.ACTION_UP,
                MotionEvent.ACTION_CANCEL -> { stopRecording(); true }
                else -> false
            }
        }

        requestPermissionsIfNeeded()
        setStatus(getString(R.string.initial_status))
    }

    private fun requestPermissionsIfNeeded() {
        val needed = mutableListOf<String>()
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO)
            != PackageManager.PERMISSION_GRANTED
        ) {
            needed += Manifest.permission.RECORD_AUDIO
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
            ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS)
            != PackageManager.PERMISSION_GRANTED
        ) {
            needed += Manifest.permission.POST_NOTIFICATIONS
        }
        if (needed.isNotEmpty()) {
            ActivityCompat.requestPermissions(this, needed.toTypedArray(), REQ_PERMS)
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
        Log.d(TAG, "onKeyDown keyCode=$keyCode (${KeyEvent.keyCodeToString(keyCode)}) repeat=${event.repeatCount}")
        if (isStemKey(keyCode) && event.repeatCount == 0) {
            startRecording()
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent): Boolean {
        Log.d(TAG, "onKeyUp keyCode=$keyCode (${KeyEvent.keyCodeToString(keyCode)})")
        if (isStemKey(keyCode)) {
            stopRecording()
            return true
        }
        return super.onKeyUp(keyCode, event)
    }

    private fun isStemKey(keyCode: Int): Boolean =
        keyCode == KeyEvent.KEYCODE_STEM_PRIMARY ||
            keyCode == KeyEvent.KEYCODE_STEM_1 ||
            keyCode == KeyEvent.KEYCODE_STEM_2 ||
            keyCode == KeyEvent.KEYCODE_STEM_3

    private fun startRecording() {
        if (isRecording) return
        val granted = ContextCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO) ==
            PackageManager.PERMISSION_GRANTED
        if (!granted) {
            setStatus(getString(R.string.need_mic_permission))
            requestPermissionsIfNeeded()
            return
        }
        if (recorder.start()) {
            isRecording = true
            setStatus(getString(R.string.recording))
        } else {
            setStatus(getString(R.string.start_failed))
        }
    }

    private fun stopRecording() {
        if (!isRecording) return
        val file = recorder.stop()
        isRecording = false
        if (file == null || !file.exists() || file.length() == 0L) {
            setStatus(getString(R.string.recording_failed))
            return
        }
        val kb = file.length() / 1024
        setStatus(getString(R.string.sending, kb))
        lifecycleScope.launch {
            val sent = phoneSync.sendAudio(file)
            setStatus(
                if (sent) getString(R.string.sent, kb)
                else getString(R.string.saved_local, kb)
            )
        }
    }

    private fun setStatus(text: String) {
        statusText.text = text
    }

    override fun onPause() {
        if (isRecording) {
            recorder.stop()
            isRecording = false
        }
        super.onPause()
    }

    companion object {
        private const val REQ_PERMS = 1
        private const val TAG = "watchdex01"
    }
}
