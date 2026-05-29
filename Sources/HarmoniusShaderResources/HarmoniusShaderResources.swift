import Foundation

#if canImport(Metal)
  import Metal
#endif

public enum HarmoniusShaderResources {
  public static let defaultMetallibName = "default.metallib"

  #if canImport(Metal)
    public static func makeDefaultLibrary(device: MTLDevice) throws -> MTLLibrary {
      if let bundledURL = Bundle.module.url(
        forResource: "default",
        withExtension: "metallib"
      ) {
        return try device.makeLibrary(URL: bundledURL)
      }
      if let bundledLibrary = device.makeDefaultLibrary() {
        return bundledLibrary
      }
      return try device.makeLibrary(URL: defaultMetallibURL)
    }
  #endif
}
