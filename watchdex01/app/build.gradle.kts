plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
}

android {
    // Kotlin code stays in com.awwaiid.watchdex01.*
    namespace = "com.awwaiid.watchdex01"
    compileSdk = 35

    defaultConfig {
        // Installed package MUST match the phone-side Pebble app so the Wear
        // Data Layer routes our DataItems to it (the Data Layer is scoped by
        // package name + signing cert). Both are debug-signed on the same
        // machine so the cert matches.
        applicationId = "coredevices.coreapp"
        minSdk = 30
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.activity.ktx)
    implementation(libs.kotlinx.coroutines.android)
    implementation(libs.play.services.wearable)
}
