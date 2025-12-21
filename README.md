This is a Kotlin Multiplatform project targeting Android, iOS, Web, Desktop (JVM).

* [/composeApp](./composeApp/src) is for code that will be shared across your Compose Multiplatform applications.
  It contains several subfolders:
  * [commonMain](./composeApp/src/commonMain/kotlin) is for code that’s common for all targets.
  * Other folders are for Kotlin code that will be compiled for only the platform indicated in the folder name.
    For example, if you want to use Apple’s CoreCrypto for the iOS part of your Kotlin app,
    the [iosMain](./composeApp/src/iosMain/kotlin) folder would be the right place for such calls.
    Similarly, if you want to edit the Desktop (JVM) specific part, the [jvmMain](./composeApp/src/jvmMain/kotlin)
    folder is the appropriate location.

* [/iosApp](./iosApp/iosApp) contains iOS applications. Even if you’re sharing your UI with Compose Multiplatform,
  you need this entry point for your iOS app. This is also where you should add SwiftUI code for your project.

### Build and Run Android Application

To build and run the development version of the Android app, use the run configuration from the run widget
in your IDE’s toolbar or build it directly from the terminal:
* on macOS/Linux

  ```shell
  ./gradlew :composeApp:assembleDebug
  ```

- on Windows

  ```shell
  .\gradlew.bat :composeApp:assembleDebug
  ```

### Build and Run Desktop (JVM) Application

To build and run the development version of the desktop app, use the run configuration from the run widget
in your IDE’s toolbar or run it directly from the terminal:
* on macOS/Linux

  ```shell
  ./gradlew :composeApp:run
  ```

- on Windows

  ```shell
  .\gradlew.bat :composeApp:run
  ```

### Build and Run Web Application

To build and run the development version of the web app, use the run configuration from the run widget
in your IDE's toolbar or run it directly from the terminal:
* for the Wasm target (faster, modern browsers):
  * on macOS/Linux

    ```shell
    ./gradlew :composeApp:wasmJsBrowserDevelopmentRun
    ```

  * on Windows

    ```shell
    .\gradlew.bat :composeApp:wasmJsBrowserDevelopmentRun
    ```

- for the JS target (slower, supports older browsers):
  * on macOS/Linux

    ```shell
    ./gradlew :composeApp:jsBrowserDevelopmentRun
    ```

  * on Windows

    ```shell
    .\gradlew.bat :composeApp:jsBrowserDevelopmentRun
    ```

### xcode configuration
The short answer is **no**. Unlike editors like VS Code or JetBrains IDEs, Xcode does not have a built-in UI setting to change the cursor's blink rate, thickness, or style (e.g., switching from a bar to a block).

However, because Xcode is a native macOS application, it respects system-wide defaults. You can actually modify the cursor behavior using the **Terminal**.

---

### How to Change the Blink Rate

If you find the blinking distracting or too slow, you can adjust the intervals (measured in milliseconds) via the command line.

1. **To stop the cursor from blinking (Solid Cursor):**
Open Terminal and run:
`defaults write -g AppleCursorBlinkPeriod -float 0`
*(You will need to restart Xcode for this to take effect).*
2. **To set a custom blink speed:**
The default is usually around 1000ms. To make it faster (e.g., 500ms):
`defaults write -g AppleCursorBlinkPeriod -float 500`
3. **To reset to default:**
`defaults delete -g AppleCursorBlinkPeriod`

### Other "Cursor" Customizations in Xcode

While you can't change the blink via settings, you can change how the cursor **behaves** and **looks** in other ways:

* **Line Highlighting:** Go to `Settings` > `Text Editing` > `Display` and check **"Highlight current line"**. This makes it much easier to find your cursor in a sea of code.
* **Insertion Point Color:** You can change the color of the cursor by going to `Settings` > `Themes`. Select your active theme and click on the **"Cursor"** color swatch to change it from the default blue to something higher contrast.
* **Vim Mode:** If you enable Vim mode (`Editor` > `Vim Mode`), the cursor will change to a **block** when in Normal mode and a **bar** when in Insert mode.

---

### Comparison with Other Editors

If you are coming from another IDE, here is how Xcode differs:

| Feature | Xcode | VS Code / IntelliJ |
| --- | --- | --- |
| **Blink Rate UI** | No | Yes |
| **Cursor Style (Block/Underline)** | No (unless in Vim mode) | Yes |
| **Multi-Cursor Support** | Yes (`Option` + Click) | Yes |
| **Smooth Caret Animation** | No | Yes |

Would you like the terminal commands to change the cursor thickness or help setting up a high-contrast theme to make the cursor more visible?


### Build and Run iOS Application

To build and run the development version of the iOS app, use the run configuration from the run widget
in your IDE’s toolbar or open the [/iosApp](./iosApp) directory in Xcode and run it from there.

---

Learn more about [Kotlin Multiplatform](https://www.jetbrains.com/help/kotlin-multiplatform-dev/get-started.html),
[Compose Multiplatform](https://github.com/JetBrains/compose-multiplatform/#compose-multiplatform),
[Kotlin/Wasm](https://kotl.in/wasm/)…

We would appreciate your feedback on Compose/Web and Kotlin/Wasm in the public Slack channel [#compose-web](https://slack-chats.kotlinlang.org/c/compose-web).
If you face any issues, please report them on [YouTrack](https://youtrack.jetbrains.com/newIssue?project=CMP).

# Build for iOS Simulator (arm64)

./gradlew :composeApp:linkDebugFrameworkIosSimulatorArm64

# Build for real device

./gradlew :composeApp:linkDebugFrameworkIosArm64

# The framework will be output to

composeApp/build/bin/iosSimulatorArm64/debugFramework/ComposeApp.framework

# Option 1: Use Xcode

open iosApp/iosApp.xcodeproj

# Then run from Xcode (Cmd+R)

# Option 2: Use xcodebuild CLI

# Build and run on simulator

cd iosApp
xcodebuild -project iosApp.xcodeproj \
  -scheme iosApp \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPhone 15' \
  build

# Then launch with xcrun

xcrun simctl boot "iPhone 15"
xcrun simctl install booted build/Debug-iphonesimulator/iosApp.app
xcrun simctl launch booted app.kamkash.physicsfx

# Build for iPad simulator

cd iosApp
xcodebuild -project iosApp.xcodeproj \
  -scheme iosApp \
  -sdk iphonesimulator \
  -destination 'platform=iOS Simulator,name=iPad Pro (11-inch) (3rd generation)' \
  build

# Launch on iPad simulator

xcrun simctl boot "iPad Pro (11-inch) (3rd generation)"
xcrun simctl install booted build/Debug-iphonesimulator/iosApp.app
xcrun simctl launch booted app.kamkash.physicsfx

####################################################

# Build and run on your connected iPad

cd iosApp
xcodebuild -project iosApp.xcodeproj \
  -scheme iosApp \
  -sdk iphoneos \
  -destination 'platform=iOS,name=iPad' \
  build

# Or use the device ID directly

xcodebuild -project iosApp.xcodeproj \
  -scheme iosApp \
  -sdk iphoneos \
  -destination 'platform=iOS,id=00008103-001A309022DA001E' \
  build

# Or use xcodebuild CLI

xcodebuild -project iosApp.xcodeproj -scheme iosApp -sdk iphoneos -destination 'platform=iOS,name=iPad' build

# Install the app (requires ios-deploy tool)

ios-deploy --bundle build/Debug-iphoneos/iosApp.app

# Or use Xcode's xcrun

xcrun devicectl device install app --device 00008103-001A309022DA001E build/Debug-iphoneos/iosApp.app
xcrun devicectl device process launch --device 00008103-001A309022DA001E app.kamkash.physicsfx  

# Your app is at

/Users/kamran/Library/Developer/Xcode/DerivedData/iosApp-dpvwhunoybihgofxznciyxordpcx/Build/Products/Debug-iphoneos/PhysicsFX.app

# Install

xcrun devicectl device install app --device 00008103-001A309022DA001E \
  ~/Library/Developer/Xcode/DerivedData/iosApp-dpvwhunoybihgofxznciyxordpcx/Build/Products/Debug-iphoneos/PhysicsFX.app

# Launch

xcrun devicectl device process launch --device 00008103-001A309022DA001E app.kamkash.physicsfx.PhysicsFX

# Or to build to a local folder, add -derivedDataPath ./build to xcodebuild

xcodebuild -project iosApp.xcodeproj -scheme iosApp -sdk iphoneos \
  -destination 'platform=iOS,name=iPad' -derivedDataPath ./build build

######################################################

# Android APK

# Android sdk

```bash
export PATH="$PATH:/Users/kamran/Library/Android/sdk/platform-tools"
./gradlew :composeApp:assembleDebug
adb devices -l
adb install ./composeApp/build/outputs/apk/debug/composeApp-debug.apk
adb -s <deviceId> install ./composeApp/build/outputs/apk/debug/composeApp-debug.apk
adb -s HNY09H0M install ./composeApp/build/outputs/apk/debug/composeApp-debug.apk
adb uninstall app.kamkash.physicsfx
adb shell am start -n app.kamkash.physicsfx/.MainActivity
adb shell am stop-app app.kamkash.physicsfx


adb logcat --pid=$(adb shell pidof app.kamkash.physicsfx) -v time 
adb logcat -s PhysicsFX
adb logcat -s PhysicsFX:D

adb kill-server
adb start-server
adb devices

```
