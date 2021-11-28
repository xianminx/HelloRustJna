public interface CLibrary extends Library {
  CLibrary INSTANCE = (CLibrary) Native.loadLibrary("c", CLibrary.class);

  int atol(String s);
  // public static native int atol(String s);
}