// In android/app/src/main/java/com/keepingitreal/MainApplication.kt
// Add the import and the package registration shown below.

// ADD import:
import com.keepingitreal.bridge.NotificationPackage

// In the ReactNativeHost override, inside getPackages(), ADD NotificationPackage():
override fun getPackages(): List<ReactPackage> =
    PackageList(this).packages.apply {
        add(NotificationPackage())   // ADD THIS LINE
    }
