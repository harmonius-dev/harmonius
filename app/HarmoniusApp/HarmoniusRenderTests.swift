import XCTest

final class HarmoniusRenderTests: XCTestCase {
  override func setUpWithError() throws {
    continueAfterFailure = false
  }

  @MainActor
  func testTriangleRendersSnapshot() throws {
    let app = XCUIApplication()
    app.launch()

    let ready = app.descendants(matching: .any)["metal-view-ready"]
    XCTAssertTrue(ready.waitForExistence(timeout: 10))

    let content = app.descendants(matching: .any)["snapshot-content"]
    XCTAssertTrue(content.waitForExistence(timeout: 2))
    let screenshot = content.screenshot()
    SnapshotImageTesting.assertSnapshot(
      of: screenshot.image,
      named: "triangle",
      precision: 0.98
    )
  }
}
