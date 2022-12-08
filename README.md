# flutter_rust_counter

이 프로젝트는 Flutter에서 Rust를 이용하는 방법을 다룹니다. Flutter의 기본 템플릿인 Counter 앱을 Rust를 이용하여 재작성합니다.

Counter 앱의 `increase` 비즈니스 로직을 Rust가 가져가고 Flutter의 위젯은 View역할만 `counter` 변수는 ViewModel 역할만 합니다.


## Flutter 프로젝트 만들기

```bash
flutter create flutter_rust_counter
# 혹은 이 프로젝트를 클론
```

## Rust 라이브러리 프로젝트 만들기

```bash
# rust_lib은 변경할 수 있습니다. 이 경로를 이용하여 클래스 이름이 만들어집니다
cargo new --lib rust_lib 
```

## Flutter, Rust 의존성 추가

Flutter 프로젝트 루트에서

```bash
cargo install flutter_rust_bridge_codegen
flutter pub add --dev ffigen && flutter pub add ffi
# if building for iOS or MacOS
cargo install cargo-xcode
```

### Rust의 enum 을 사용한다면 추가로 설치

```bash
flutter pub add flutter_rust_bridge
# if using Dart codegen
flutter pub add -d build_runner
flutter pub add -d freezed
flutter pub add freezed_annotation
```

### Rust에서

```yaml
[dependencies]
flutter_rust_bridge = "1"
```

## Android 셋업

### Rust 타겟 셋업

```
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

### Android Studio에서 NDK 설치

```yaml
ANDROID_NDK=~/Library/Android/sdk/ndk/<version> # 절대경로여야함
```

### Android gradle hook 설치

```
cargo install cargo-ndk
```

android/app/build.gradle 에 다음 코드 붙여넣기

```java
[
    new Tuple2('Debug', ''),
    new Tuple2('Profile', '--release'),
    new Tuple2('Release', '--release')
].each {
    def taskPostfix = it.first
    def profileMode = it.second
    tasks.whenTaskAdded { task ->
        if (task.name == "javaPreCompile$taskPostfix") {
            task.dependsOn "cargoBuild$taskPostfix"
        }
    }
    tasks.register("cargoBuild$taskPostfix", Exec) {
        // Until https://github.com/bbqsrc/cargo-ndk/pull/13 is merged,
        // this workaround is necessary.

        def ndk_command = """cargo ndk \
            -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 \
            -o ../android/app/src/main/jniLibs build $profileMode"""
        // $crate를 위에서 만든 rust_lib 으로 변경해야합니다.
        workingDir "../../$crate"
        environment "ANDROID_NDK_HOME", "$ANDROID_NDK"
        if (org.gradle.nativeplatform.platform.internal.DefaultNativePlatform.currentOperatingSystem.isWindows()) {
            commandLine 'cmd', '/C', ndk_command
        } else {
            commandLine 'sh', '-c', ndk_command
        }
    }
}
```

## iOS 셋업

```bash
# 64 bit targets (real device & simulator):
rustup target add aarch64-apple-ios x86_64-apple-ios
# New simulator target for Xcode 12 and later
rustup target add aarch64-apple-ios-sim
```

### Rust 프로젝트에 xcode 프로젝트 만들기

```
cargo xcode
```

### Flutter Xcode 프로젝트에 Rust 프로젝트 연결

XCode를 열어 Runner 아래에 rust_lib의 xcode 프로적트를 서브프로젝트로 추가합니다.

Runner 프로젝트의 Build Phases에서 다음과 같이 설정합니다

- Target Dependencies에 cdylib 추가
- Link Binary With Libaries에 _static.a 추가

## Rust 라이브러리 작성

rust_lib/src 디렉터리에 api.rs 파일을 만들고 다음과 같이 코드를 적습니다

```rust
pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}
```

그리고 rust_lib/src/lib.rs 에 다음과 같이 모듈을 추가합니다.


## iOS 그리고 Dart 코드 생성

```bash
# Flutter 프로젝트 루트에서 실행
flutter_rust_bridge_codegen \
    -r rust_lib/src/api.rs \
    -d lib/bridge_generated.dart \
    -c ios/Runner/bridge_generated.h
```


### iOS 더미헤더 추가

`ios/Runner/Runner-Bridging-Header.h` 를 열고 다음 코드를 추가합니다.

```c
#import "bridge_generated.h"
```

그리고 AppDelegate.swift를 열어 다음 코드를 추가합니다

```swift
let dummy = dummy_method_to_enforce_bundling()
print(dummy)
```


## 개발 중 Rust 코드를 변경하는 경우

iOS 그리고 Dart 코드 생성 섹션의 다음 코드를 다시 실행합니다.

```
flutter_rust_bridge_codegen \
    -r rust_lib/src/api.rs \
    -d lib/bridge_generated.dart \
    -c ios/Runner/bridge_generated.h
```

## Rust increase 메소드 구현

```rust
pub fn increase(current: usize) -> usize {
    current + 1
}
```

## Flutter increase 메소드 사용

```dart
final _api = RustLibImpl(
Platform.isIOS
    ? DynamicLibrary.process()
    : DynamicLibrary.open('librust_lib.so'),
);
```


```dart
void _incrementCounter() async {
    _counter = await _api.increase(current: _counter);
    setState(() {});
}
```

