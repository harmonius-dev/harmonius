import AppKit
import XCTest

enum SnapshotImageError: Error, CustomStringConvertible {
  case imageConversionFailed
  case referenceMissing(String)
  case sizeMismatch(expected: String, actual: String)
  case pixelMismatch(differingPixels: Int, totalPixels: Int, diffPath: String)

  var description: String {
    switch self {
    case .imageConversionFailed:
      return "Failed to convert screenshot to PNG bitmap."
    case .referenceMissing(let path):
      return "Missing reference snapshot at \(path)."
    case .sizeMismatch(let expected, let actual):
      return "Snapshot size mismatch. Expected \(expected), got \(actual)."
    case .pixelMismatch(let differing, let total, let diffPath):
      return "Snapshot mismatch: \(differing)/\(total) pixels differ. Diff: \(diffPath)"
    }
  }
}

private struct BitmapRGBA {
  let width: Int
  let height: Int
  let bytesPerRow: Int
  let rgba: [UInt8]
}

enum SnapshotImageTesting {
  private static let recordEnabled: Bool = {
    ProcessInfo.processInfo.environment["SNAPSHOT_RECORD"] == "1"
  }()

  static func assertSnapshot(
    of image: NSImage,
    named name: String,
    precision: Double = 0.99,
    file: StaticString = #filePath,
    line: UInt = #line
  ) {
    do {
      try compareSnapshot(of: image, named: name, precision: precision)
    } catch {
      XCTFail(String(describing: error), file: file, line: line)
    }
  }

  private static func compareSnapshot(
    of image: NSImage,
    named name: String,
    precision: Double
  ) throws {
    let referenceURL = referenceURL(for: name)
    let actualPNG = try pngData(from: image)

    if recordEnabled || !FileManager.default.fileExists(atPath: referenceURL.path) {
      try FileManager.default.createDirectory(
        at: referenceURL.deletingLastPathComponent(),
        withIntermediateDirectories: true
      )
      try actualPNG.write(to: referenceURL)
      return
    }

    let referencePNG = try Data(contentsOf: referenceURL)
    let referenceImage = try bitmap(from: referencePNG)
    let actualImage = try bitmap(from: actualPNG)

    guard referenceImage.width == actualImage.width,
      referenceImage.height == actualImage.height
    else {
      throw SnapshotImageError.sizeMismatch(
        expected: "\(referenceImage.width)x\(referenceImage.height)",
        actual: "\(actualImage.width)x\(actualImage.height)"
      )
    }

    let differing = differingPixelCount(reference: referenceImage, actual: actualImage)
    let total = referenceImage.width * referenceImage.height
    let matchRatio = 1.0 - (Double(differing) / Double(total))

    if matchRatio < precision {
      let diffURL = diffURL(for: name)
      try writeDiff(reference: referenceImage, actual: actualImage, to: diffURL)
      throw SnapshotImageError.pixelMismatch(
        differingPixels: differing,
        totalPixels: total,
        diffPath: diffURL.path
      )
    }
  }

  private static func referenceURL(for name: String) -> URL {
    URL(fileURLWithPath: #filePath)
      .deletingLastPathComponent()
      .appendingPathComponent("__Snapshots__")
      .appendingPathComponent("HarmoniusRenderTests")
      .appendingPathComponent("\(name).png")
  }

  private static func diffURL(for name: String) -> URL {
    referenceURL(for: name)
      .deletingLastPathComponent()
      .appendingPathComponent("\(name)-diff.png")
  }

  private static func pngData(from image: NSImage) throws -> Data {
    guard
      let tiff = image.tiffRepresentation,
      let rep = NSBitmapImageRep(data: tiff),
      let png = rep.representation(using: .png, properties: [:])
    else {
      throw SnapshotImageError.imageConversionFailed
    }
    return png
  }

  private static func bitmap(from png: Data) throws -> BitmapRGBA {
    guard let rep = NSBitmapImageRep(data: png) else {
      throw SnapshotImageError.imageConversionFailed
    }
    guard let rgbaPtr = rep.bitmapData else {
      throw SnapshotImageError.imageConversionFailed
    }
    let byteCount = rep.bytesPerRow * rep.pixelsHigh
    var rgba = [UInt8](repeating: 0, count: byteCount)
    rgba.withUnsafeMutableBytes { dest in
      guard let destBase = dest.baseAddress else { return }
      memcpy(destBase, rgbaPtr, byteCount)
    }
    return BitmapRGBA(
      width: rep.pixelsWide,
      height: rep.pixelsHigh,
      bytesPerRow: rep.bytesPerRow,
      rgba: rgba
    )
  }

  private static func pixel(
    atX x: Int,
    y: Int,
    in bitmap: BitmapRGBA
  ) -> (UInt8, UInt8, UInt8) {
    let offset = y * bitmap.bytesPerRow + x * 4
    return (
      bitmap.rgba[offset],
      bitmap.rgba[offset + 1],
      bitmap.rgba[offset + 2]
    )
  }

  private static func differingPixelCount(
    reference: BitmapRGBA,
    actual: BitmapRGBA
  ) -> Int {
    var differing = 0
    for y in 0..<reference.height {
      for x in 0..<reference.width {
        let ref = pixel(atX: x, y: y, in: reference)
        let act = pixel(atX: x, y: y, in: actual)
        if ref.0 != act.0 || ref.1 != act.1 || ref.2 != act.2 {
          differing += 1
        }
      }
    }
    return differing
  }

  private static func writeDiff(
    reference: BitmapRGBA,
    actual: BitmapRGBA,
    to url: URL
  ) throws {
    let width = reference.width
    let height = reference.height
    let bytesPerRow = width * 4
    var diff = [UInt8](repeating: 0, count: bytesPerRow * height)
    for y in 0..<height {
      for x in 0..<width {
        let offset = y * bytesPerRow + x * 4
        let ref = pixel(atX: x, y: y, in: reference)
        let act = pixel(atX: x, y: y, in: actual)
        let mismatch = ref.0 != act.0 || ref.1 != act.1 || ref.2 != act.2
        if mismatch {
          diff[offset] = 255
          diff[offset + 1] = 0
          diff[offset + 2] = 0
          diff[offset + 3] = 255
        } else {
          diff[offset] = ref.0
          diff[offset + 1] = ref.1
          diff[offset + 2] = ref.2
          diff[offset + 3] = 255
        }
      }
    }
    let rep = NSBitmapImageRep(
      bitmapDataPlanes: nil,
      pixelsWide: width,
      pixelsHigh: height,
      bitsPerSample: 8,
      samplesPerPixel: 4,
      hasAlpha: true,
      isPlanar: false,
      colorSpaceName: .deviceRGB,
      bytesPerRow: bytesPerRow,
      bitsPerPixel: 32
    )!
    memcpy(rep.bitmapData!, diff, diff.count)
    guard let png = rep.representation(using: .png, properties: [:]) else {
      throw SnapshotImageError.imageConversionFailed
    }
    try FileManager.default.createDirectory(
      at: url.deletingLastPathComponent(),
      withIntermediateDirectories: true
    )
    try png.write(to: url)
  }
}
