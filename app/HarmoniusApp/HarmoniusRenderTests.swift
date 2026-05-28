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

    assertSnapshot(
      of: metalView.screenshot().image,
      as: .image(precision: 0.98),
      named: "triangle"
    )
  }
}
