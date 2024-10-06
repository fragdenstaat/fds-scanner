import SwiftRs
import Tauri
import UIKit
import WebKit
import AuthenticationServices


class WebAuthArgs: Decodable {
  let url: String
  let redirectUrl: String
}


struct WebAuthResponse: Encodable {
  let url: String?
}

class WebAuthPlugin: Plugin, ASWebAuthenticationPresentationContextProviding {

  var webview: WKWebView!
  var invoke: Invoke? = nil
  var session: ASWebAuthenticationSession? = nil

  @objc public override func load(webview: WKWebView) {
    Logger.info("WebAuthPlugin loaded")
    self.webview = webview
  }

  @objc private func cancel(_ invoke: Invoke) {
    self.invoke?.reject("cancelled")

    destroy()
    invoke.resolve()
  }

  private func destroy() {
    self.session?.cancel()
    self.invoke = nil
    // if windowed {
    //   let backgroundColor = previousBackgroundColor ?? UIColor.white
      // webView.isOpaque = true
    //   webView.backgroundColor = backgroundColor
    //   webView.scrollView.backgroundColor = backgroundColor
    // }
  }

  @objc public func start_auth(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(WebAuthArgs.self)

    Logger.info("start_auth plugin called")

    guard let authURL = URL(string: args.url) else { return }
    let redirectUrl = args.redirectUrl

    Logger.info("authURL: \(authURL)")
    
    let url = URL(string: redirectUrl)
    let callbackScheme = url?.scheme

    // var iOS14min: Bool = false
    // if #available(iOS 14.0, *) { iOS14min = true }

    self.webview.isOpaque = false

    self.session = ASWebAuthenticationSession(
      url: authURL,
      callbackURLScheme: callbackScheme
    ) { url, error in
      self.webview.isOpaque = true
      invoke.resolve(WebAuthResponse(url: url?.absoluteString))
    }

    self.session!.presentationContextProvider = self
    print("AS Session start")
    self.session!.start()
  }

  public func presentationAnchor(for _: ASWebAuthenticationSession) -> ASPresentationAnchor {
    guard let window = self.webview?.window else {
      return ASPresentationAnchor()
    }
    print("AS presentation anchor is window")
    return window
  }
}

@_cdecl("init_plugin_webauth")
func initPlugin() -> Plugin {
  print("init_plugin_webauth")
  return WebAuthPlugin()
}
