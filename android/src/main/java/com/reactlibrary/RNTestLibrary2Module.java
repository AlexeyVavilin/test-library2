
package com.reactlibrary;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Callback;

public class RNTestLibrary2Module extends ReactContextBaseJavaModule {
static {
        System.loadLibrary("mtproto_crypto");
    }

  private final ReactApplicationContext reactContext;

  public RNTestLibrary2Module(ReactApplicationContext reactContext) {
    super(reactContext);
    this.reactContext = reactContext;
  }

  @Override
  public String getName() {
    return "RNTestLibrary2";
  }


  @ReactMethod
  public void factorize(String challenge, Promise promise) {
      promise.resolve(factorize(challenge));
  }

  private static native String factorize(String challenge);
}