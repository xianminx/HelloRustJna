package com.example.hellorustjna;

public class RustJNILib {
    static {
        System.load("rustlib");
    }

    /**
     * A native method that is implemented by the 'rust' native library,
     * which is packaged with this application.
     */
    public static native void invokeCallbackViaJNI(JNICallback callback);
}
