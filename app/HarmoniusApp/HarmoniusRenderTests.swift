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

    let ready = app.descendants(matching: .any)["metal-view-ready"]
    if !ready.waitForExistence(timeout: 10) {
      let isCI = ProcessInfo.processInfo.environment["CI"] == "true"
      let isGitHubActions = FileManager.default.currentDirectoryPath.hasPrefix(
        "/Users/runner/work/"
      )
      if isCI || isGitHubActions {
        throw XCTSkip("Metal view did not present a frame on the CI runner.")
      }
      XCTFail("Metal view did not present a frame.")
      return
    }

    let metalView = app.descendants(matching: .any)["metal-view"]
    XCTAssertTrue(metalView.waitForExistence(timeout: 2))

    assertSnapshot(
      of: metalView.screenshot().image,
      as: .image(precision: 0.98),
      named: "triangle"
    )
  }
}
