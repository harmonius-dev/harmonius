import Foundation

public struct HarmoniusPointSize: Equatable, Sendable {
  public let height: Int
  public let width: Int

  public init(width: Int, height: Int) {
    self.height = height
    self.width = width
  }
}

public struct HarmoniusUIConfiguration: Equatable, Sendable {
  public static let defaultMacOSWindowPointSize = HarmoniusPointSize(
    width: 960,
    height: 540
  )

  public let macOSWindowPointSize: HarmoniusPointSize?

  public init(environment: [String: String] = ProcessInfo.processInfo.environment) {
    macOSWindowPointSize = Self.pointSize(
      from: environment["HARMONIUS_UI_TEST_WINDOW_POINTS"]
    )
  }

  private static func pointSize(from value: String?) -> HarmoniusPointSize? {
    guard let value else { return nil }
    let parts = value.split(separator: "x")
    guard parts.count == 2,
      let width = Int(parts[0]),
      let height = Int(parts[1]),
      width > 0,
      height > 0
    else {
      return nil
    }
    return HarmoniusPointSize(width: width, height: height)
  }
}
