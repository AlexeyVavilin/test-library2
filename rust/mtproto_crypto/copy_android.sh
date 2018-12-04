#! /bin/bash

mkdir -p ../../android/src/main/jniLibs
mkdir -p ../../android/src/main/jniLibs/x86
mkdir -p ../../android/src/main/jniLibs/arm64-v8a
mkdir -p ../../android/src/main/jniLibs/armeabi-v7a

cp ./target/i686-linux-android/release/libmtproto_crypto.so ../../android/src/main/jniLibs/x86/libmtproto_crypto.so
cp ./target/aarch64-linux-android/release/libmtproto_crypto.so ../../android/src/main/jniLibs/arm64-v8a/libmtproto_crypto.so
cp ./target/armv7-linux-androideabi/release/libmtproto_crypto.so ../../android/src/main/jniLibs/armeabi-v7a/libmtproto_crypto.so