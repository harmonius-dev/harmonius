import AppKit
import SnapshotTesting
import XCTest

@MainActor
final class HarmoniusRenderTests: XCTestCase {
  override func invokeTest() {
    let record: SnapshotTestingConfiguration.Record =
      ProcessInfo.processInfo.environment["SNAPSHOT_RECORD"] == "1" ? .all : .never
    withSnapshotTesting(record: record) {
      super.invokeTest()
    }
  }

  override func setUpWithError() throws {
    continueAfterFailure = false
  }

  func testTriangleRendersSnapshot() throws {
    let app = XCUIApplication()
    app.launchArguments.append("-HarmoniusSnapshotMode")
    app.launch()

    let metalView = app.descendants(matching: .any)["metal-view"]
    XCTAssertTrue(metalView.waitForExistence(timeout: 10))
    let image = try renderedScreenshot(of: metalView)

    assertSnapshot(
      of: image,
      as: .image(precision: 0.98),
      named: "triangle"
    )
  }

  private func renderedScreenshot(
    of element: XCUIElement,
    timeout: TimeInterval = 10
  ) throws -> NSImage {
    let deadline = Date().addingTimeInterval(timeout)
    var image = element.screenshot().image

    while Date() < deadline {
      image = element.screenshot().image
      if image.containsRenderedTriangle {
        return image
      }
      Thread.sleep(forTimeInterval: 0.25)
    }

    XCTFail("Timed out waiting for the Metal triangle to render.")
    return image
  }
}

extension NSImage {
  fileprivate var containsRenderedTriangle: Bool {
    guard let cgImage = cgImage(forProposedRect: nil, context: nil, hints: nil) else {
      return false
    }
    let width = cgImage.width
    let height = cgImage.height
    let bytesPerPixel = 4
    let bytesPerRow = width * bytesPerPixel
    var pixels = [UInt8](repeating: 0, count: height * bytesPerRow)
    guard
      let context = CGContext(
        data: &pixels,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: CGColorSpaceCreateDeviceRGB(),
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
      )
    else {
      return false
    }

    context.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))
    let stepX = max(width / 32, 1)
    let stepY = max(height / 32, 1)
    for y in stride(from: 0, to: height, by: stepY) {
      for x in stride(from: 0, to: width, by: stepX) {
        let offset = (y * bytesPerRow) + (x * bytesPerPixel)
        let red = pixels[offset]
        let green = pixels[offset + 1]
        let blue = pixels[offset + 2]
        if red > 120 || green > 120 || blue > 120 {
          return true
        }
      }
    }
    return false
  }
}
