import AppKit
import Foundation
import SnapshotTesting
import Testing
import WebDriver

private let macOSSnapshotPointSize = SnapshotSize(width: 960, height: 540)

private enum AppiumPlatform: String {
  case iOSSimulator = "ios-simulator"
  case macOS = "macos"
}

private enum AppiumTestError: Error, CustomStringConvertible {
  case invalidPNG(String)
  case missingEnvironment(String)
  case snapshotMismatch(URL, String)
  case unsupportedPlatform(String)

  var description: String {
    switch self {
    case .invalidPNG(let reason):
      return "invalid PNG: \(reason)"
    case .missingEnvironment(let name):
      return "missing required environment variable \(name)"
    case .snapshotMismatch(let url, let reason):
      return "UI snapshot differs from baseline at \(url.path): \(reason)"
    case .unsupportedPlatform(let platform):
      return "unsupported Appium platform \(platform)"
    }
  }
}

extension ElementLocator {
  fileprivate static func accessibilityID(_ value: String) -> Self {
    Self(using: "accessibility id", value: value)
  }
}

@Test func harmoniusLaunchesAndMatchesUISnapshot() throws {
  let environment = try AppiumEnvironment.current()
  let session = try makeSession(environment: environment)
  defer { try? session.delete() }

  let content = try session.findElement(
    locator: .accessibilityID("snapshot-content"),
    waitTimeout: 15
  )
  let metalView = try session.findElement(
    locator: .accessibilityID("metal-view"),
    waitTimeout: 15
  )
  #expect(try content.displayed)
  #expect(try metalView.displayed)

  let screenshot = try contentScreenshot(session: session, element: content)
  let identity = try SnapshotIdentity(environment: environment)
  try assertUISnapshot(screenshot: screenshot, identity: identity)
}

private struct AppiumEnvironment {
  let appPath: String
  let automationName: String
  let deviceName: String?
  let displayScale: Int?
  let platform: AppiumPlatform
  let platformVersion: String?
  let runtimeName: String?
  let serverURL: URL

  static func current() throws -> Self {
    let platformName = try environment("HARMONIUS_APPIUM_PLATFORM")
    guard let platform = AppiumPlatform(rawValue: platformName) else {
      throw AppiumTestError.unsupportedPlatform(platformName)
    }
    return Self(
      appPath: try environment("HARMONIUS_APPIUM_APP"),
      automationName: try environment("HARMONIUS_APPIUM_AUTOMATION_NAME"),
      deviceName: optionalEnvironment("HARMONIUS_APPIUM_DEVICE"),
      displayScale: try optionalEnvironmentInt("HARMONIUS_APPIUM_DISPLAY_SCALE"),
      platform: platform,
      platformVersion: optionalEnvironment("HARMONIUS_APPIUM_IOS_VERSION"),
      runtimeName: optionalEnvironment("HARMONIUS_APPIUM_IOS_RUNTIME"),
      serverURL: try environmentURL("APPIUM_SERVER_URL")
    )
  }
}

private struct SnapshotIdentity {
  let metadata: SnapshotMetadata
  let metadataURL: URL
  let pngURL: URL
  let snapshotDirectoryURL: URL
  let snapshotTestName: String

  init(environment: AppiumEnvironment) throws {
    let scale = try snapshotScale(environment: environment)
    let platformDirectory = try snapshotDirectory(environment: environment, scale: scale)
    let fileBaseName = "snapshot-content"
    snapshotDirectoryURL = platformDirectory
    snapshotTestName = fileBaseName
    pngURL = platformDirectory.appendingPathComponent("\(fileBaseName).1.png")
    metadataURL = platformDirectory.appendingPathComponent("\(fileBaseName).json")
    metadata = SnapshotMetadata(
      appBundleID: "dev.harmonius.App",
      deviceName: environment.deviceName,
      platform: environment.platform.rawValue,
      runtimeName: environment.runtimeName,
      scale: scale
    )
  }
}

private struct SnapshotMetadata: Encodable {
  let appBundleID: String
  let deviceName: String?
  let platform: String
  let runtimeName: String?
  let scale: Int?

  enum CodingKeys: String, CodingKey {
    case appBundleID
    case deviceName
    case platform
    case runtimeName
    case scale
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    try container.encode(appBundleID, forKey: .appBundleID)
    try container.encodeIfPresent(deviceName, forKey: .deviceName)
    try container.encode(platform, forKey: .platform)
    try container.encodeIfPresent(runtimeName, forKey: .runtimeName)
    try container.encodeIfPresent(scale, forKey: .scale)
  }
}

private struct SnapshotSize {
  let width: Int
  let height: Int

  var name: String { "\(width)x\(height)" }
}

private func makeSession(environment: AppiumEnvironment) throws -> Session {
  let webDriver = AppiumHTTPWebDriver(
    endpoint: environment.serverURL,
    requestTimeout: 240
  )
  var capabilities = AppiumCapabilities(
    automationName: environment.automationName,
    newCommandTimeout: 120,
    platformName: environment.platform.w3cPlatformName,
    printPageSourceOnFindFailure: true
  )

  switch environment.platform {
  case .macOS:
    capabilities.appPath = environment.appPath
    capabilities.environment = [
      "HARMONIUS_UI_TEST_WINDOW_POINTS": macOSSnapshotPointSize.name
    ]
  case .iOSSimulator:
    capabilities.app = environment.appPath
    capabilities.deviceName = environment.deviceName
    capabilities.platformVersion = environment.platformVersion
    capabilities.wdaLaunchTimeout = 180_000
    capabilities.wdaStartupRetries = 3
  }

  let response = try webDriver.send(
    CreateAppiumSession(capabilities: capabilities)
  )
  let session = Session(
    webDriver: webDriver,
    existingId: response.value.sessionId,
    owned: true
  )
  session.implicitWaitTimeout = 5
  return session
}

private func snapshotScale(
  environment: AppiumEnvironment
) throws -> Int? {
  guard environment.platform == .macOS else { return nil }
  guard let displayScale = environment.displayScale else {
    throw AppiumTestError.missingEnvironment("HARMONIUS_APPIUM_DISPLAY_SCALE")
  }
  guard displayScale > 0 else {
    throw AppiumTestError.invalidPNG("display scale must be positive")
  }
  return displayScale
}

private func contentScreenshot(session: Session, element: Element) throws -> Data {
  do {
    return try elementScreenshot(session: session, element: element)
  } catch {
    return try session.screenshot()
  }
}

private func elementScreenshot(session: Session, element: Element) throws -> Data {
  let base64: String = try session.webDriver.send(
    ElementScreenshot(session: session.id, element: element.id)
  ).value
  guard let data = Data(base64Encoded: base64) else {
    throw AppiumTestError.invalidPNG("element screenshot is not Base64")
  }
  return data
}

private func assertUISnapshot(
  screenshot: Data,
  identity: SnapshotIdentity
) throws {
  let image = try appiumScreenshotImage(data: screenshot)
  let failure = verifySnapshot(
    of: image,
    as: .image(precision: 0.99, perceptualPrecision: 0.99),
    record: isRecordingUISnapshots ? .all : .never,
    snapshotDirectory: identity.snapshotDirectoryURL.path,
    testName: identity.snapshotTestName
  )
  if isRecordingUISnapshots {
    try metadataData(identity.metadata).write(to: identity.metadataURL)
    return
  }

  if let failure {
    throw AppiumTestError.snapshotMismatch(identity.pngURL, failure)
  }
  #expect(FileManager.default.fileExists(atPath: identity.metadataURL.path))
}

private func appiumScreenshotImage(data: Data) throws -> NSImage {
  guard let image = NSImage(data: data),
    let cgImage = image.cgImage(forProposedRect: nil, context: nil, hints: nil),
    cgImage.width > 0,
    cgImage.height > 0
  else {
    throw AppiumTestError.invalidPNG("unable to decode Appium screenshot")
  }
  return image
}

private func metadataData(_ metadata: SnapshotMetadata) throws -> Data {
  let data = try JSONEncoder().encode(metadata)
  guard let object = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
    throw AppiumTestError.invalidPNG("snapshot metadata is not a JSON object")
  }
  let lines = try object.keys.sorted().enumerated().map { index, key in
    let comma = index == object.count - 1 ? "" : ","
    return "  \(try jsonString(key)): \(try jsonLiteral(object[key] as Any))\(comma)"
  }
  return Data((["{"] + lines + ["}"]).joined(separator: "\n").appending("\n").utf8)
}

private func jsonLiteral(_ value: Any) throws -> String {
  switch value {
  case let string as String:
    return try jsonString(string)
  case let number as NSNumber:
    return number.stringValue
  default:
    throw AppiumTestError.invalidPNG("unsupported snapshot metadata value: \(value)")
  }
}

private func jsonString(_ value: String) throws -> String {
  let data = try JSONEncoder().encode(value)
  guard let string = String(data: data, encoding: .utf8) else {
    throw AppiumTestError.invalidPNG("snapshot metadata string is not UTF-8")
  }
  return string
}

private func snapshotDirectory(
  environment: AppiumEnvironment,
  scale: Int?
) throws -> URL {
  var url = snapshotsRoot.appendingPathComponent(environment.platform.rawValue)
  switch environment.platform {
  case .macOS:
    guard let scale else {
      throw AppiumTestError.missingEnvironment("HARMONIUS_APPIUM_DISPLAY_SCALE")
    }
    url.appendPathComponent("scale-\(scale)x")
  case .iOSSimulator:
    url.appendPathComponent(sanitized(environment.deviceName ?? "unknown-device"))
    url.appendPathComponent(sanitized(environment.runtimeName ?? "unknown-runtime"))
  }
  return url
}

private func environment(_ name: String) throws -> String {
  guard let value = optionalEnvironment(name) else {
    throw AppiumTestError.missingEnvironment(name)
  }
  return value
}

private func optionalEnvironment(_ name: String) -> String? {
  guard let value = ProcessInfo.processInfo.environment[name], !value.isEmpty else {
    return nil
  }
  return value
}

private func optionalEnvironmentInt(_ name: String) throws -> Int? {
  guard let value = optionalEnvironment(name) else { return nil }
  guard let intValue = Int(value) else {
    throw AppiumTestError.missingEnvironment(name)
  }
  return intValue
}

private func environmentURL(_ name: String) throws -> URL {
  let value = try environment(name)
  guard let url = URL(string: value) else {
    throw AppiumTestError.missingEnvironment(name)
  }
  return url
}

private func sanitized(_ value: String) -> String {
  let allowed = CharacterSet.alphanumerics.union(CharacterSet(charactersIn: "-_"))
  return value.unicodeScalars.map { scalar in
    allowed.contains(scalar) ? Character(scalar) : "-"
  }.reduce(into: "") { result, character in
    result.append(character)
  }
}

private var isRecordingUISnapshots: Bool {
  ProcessInfo.processInfo.environment["HARMONIUS_RECORD_UI_SNAPSHOTS"] == "1"
}

private var snapshotsRoot: URL {
  URL(fileURLWithPath: #filePath)
    .deletingLastPathComponent()
    .appendingPathComponent("__Snapshots__")
}

extension AppiumPlatform {
  fileprivate var w3cPlatformName: String {
    switch self {
    case .macOS:
      return "mac"
    case .iOSSimulator:
      return "iOS"
    }
  }
}

private struct CreateAppiumSession: Request {
  let capabilities: AppiumCapabilities

  var pathComponents: [String] { ["session"] }
  var method: HTTPMethod { .post }
  var body: Body {
    Body(
      capabilities: CapabilityMatch(
        alwaysMatch: capabilities,
        firstMatch: nil
      )
    )
  }

  typealias Response = Requests.ResponseWithValue<ResponseValue>

  struct Body: Codable {
    let capabilities: CapabilityMatch
  }

  struct CapabilityMatch: Codable {
    let alwaysMatch: AppiumCapabilities
    let firstMatch: [AppiumCapabilities]?
  }

  struct ResponseValue: Codable {
    let sessionId: String
  }
}

private struct ElementScreenshot: Request {
  let session: String
  let element: String

  var pathComponents: [String] {
    ["session", session, "element", element, "screenshot"]
  }

  var method: HTTPMethod { .get }

  typealias Response = Requests.ResponseWithValue<String>
}

private struct AppiumCapabilities: Codable {
  var app: String?
  var appPath: String?
  var automationName: String?
  var bundleId: String?
  var deviceName: String?
  var environment: [String: String]?
  var newCommandTimeout: Int?
  var platformName: String?
  var platformVersion: String?
  var printPageSourceOnFindFailure: Bool?
  var wdaLaunchTimeout: Int?
  var wdaStartupRetries: Int?

  enum CodingKeys: String, CodingKey {
    case app = "appium:app"
    case appPath = "appium:appPath"
    case automationName = "appium:automationName"
    case bundleId = "appium:bundleId"
    case deviceName = "appium:deviceName"
    case environment = "appium:environment"
    case newCommandTimeout = "appium:newCommandTimeout"
    case platformName
    case platformVersion = "appium:platformVersion"
    case printPageSourceOnFindFailure = "appium:printPageSourceOnFindFailure"
    case wdaLaunchTimeout = "appium:wdaLaunchTimeout"
    case wdaStartupRetries = "appium:wdaStartupRetries"
  }
}

private struct AppiumHTTPWebDriver: WebDriver {
  let endpoint: URL
  let requestTimeout: TimeInterval
  let wireProtocol: WireProtocol = .w3c

  func send<Req: Request>(_ request: Req) throws -> Req.Response {
    var urlRequest = try buildURLRequest(for: request)
    urlRequest.timeoutInterval = requestTimeout

    let (status, responseData) = try send(urlRequest)
    guard status == 200 else {
      throw appiumHTTPError(status: status, data: responseData)
    }
    return try JSONDecoder().decode(Req.Response.self, from: responseData)
  }

  private func buildURLRequest<Req: Request>(
    for request: Req
  ) throws -> URLRequest {
    var url = endpoint
    for (index, pathComponent) in request.pathComponents.enumerated() {
      let last = index == request.pathComponents.count - 1
      url.appendPathComponent(pathComponent, isDirectory: !last)
    }

    var urlRequest = URLRequest(url: url)
    urlRequest.httpMethod = request.method.rawValue
    if Req.Body.self != CodableNone.self {
      urlRequest.addValue(
        "application/json;charset=UTF-8",
        forHTTPHeaderField: "content-type"
      )
      urlRequest.httpBody = try JSONEncoder().encode(request.body)
    }
    return urlRequest
  }

  private func send(_ request: URLRequest) throws -> (Int, Data) {
    let responseBox = AppiumHTTPResponse()
    let semaphore = DispatchSemaphore(value: 0)
    let task = URLSession.shared.dataTask(with: request) { data, urlResponse, error in
      if let error {
        responseBox.store(.failure(error))
      } else if let data, let urlResponse = urlResponse as? HTTPURLResponse {
        responseBox.store(.success((data, urlResponse)))
      } else {
        responseBox.store(
          .failure(
            NSError(domain: NSURLErrorDomain, code: URLError.unknown.rawValue)
          )
        )
      }
      semaphore.signal()
    }
    task.resume()
    semaphore.wait()

    switch responseBox.load() {
    case .failure(let error):
      throw error
    case .success((let data, let response)):
      return (response.statusCode, data)
    }
  }
}

private func appiumHTTPError(status: Int, data: Data) -> Error {
  if let error = try? JSONDecoder().decode(ErrorResponse.self, from: data) {
    return error
  }
  let body = String(data: data, encoding: .utf8) ?? "<\(data.count) bytes>"
  return AppiumHTTPError(status: status, body: body)
}

private struct AppiumHTTPError: Error, CustomStringConvertible {
  let status: Int
  let body: String

  var description: String {
    "Appium HTTP \(status): \(body)"
  }
}

private final class AppiumHTTPResponse: @unchecked Sendable {
  private let lock = NSLock()
  private var result: Result<(Data, HTTPURLResponse), Error> =
    .failure(NSError(domain: NSURLErrorDomain, code: URLError.unknown.rawValue))

  func store(_ newResult: Result<(Data, HTTPURLResponse), Error>) {
    lock.lock()
    result = newResult
    lock.unlock()
  }

  func load() -> Result<(Data, HTTPURLResponse), Error> {
    lock.lock()
    defer { lock.unlock() }
    return result
  }
}
