package com.example.hellorustjna;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;
import com.sun.jna.Structure;

import java.util.List;

public class RustJNALib {
    public interface RustLibrary extends Library {
        RustLibrary INSTANCE = (RustLibrary)
                Native.load("rustlib", RustLibrary.class);

        int invokeCallbackViaJNA(JNACallback callback);
    }


    public static void invokeCallbackViaJNA(JNACallback callback) {
        RustLibrary.INSTANCE.invokeCallbackViaJNA(callback);
    }

    // must be public to populate array
    public static class Point extends Structure {
        public static final List<String> FIELDS = createFieldsOrder("x", "y");
        public int x;
        public int y;

        @Override
        protected List<String> getFieldOrder() {
            return FIELDS;
        }
    }
}
