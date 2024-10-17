// Inspired by
// https://alexanderweiss.dev/blog/2020-11-28-from-uiimage-to-searchable-pdf-part-1

import Foundation
import PDFKit
import SwiftRs
import Tauri
import UIKit
import Vision
import VisionKit
import WebKit

class DocumentCameraArgs: Decodable {
  let path: String
}

struct ScanResponse: Encodable {
  let path: String?
}

class DocumentCameraPlugin: Plugin, VNDocumentCameraViewControllerDelegate {
  var webview: WKWebView!
  var invoke: Invoke? = nil
  var pdfPath: String?

  @objc public override func load(webview: WKWebView) {
    Logger.info("DocumentCameraPlugin loaded")
    self.webview = webview
  }

  @objc public func scan(_ invoke: Invoke) throws {
    self.invoke = invoke
    let args = try invoke.parseArgs(DocumentCameraArgs.self)

    guard VNDocumentCameraViewController.isSupported else {
      invoke.reject("Document scanning is not supported on this device.")
      return
    }

    self.pdfPath = args.path

    DispatchQueue.main.async { [self] in
      let documentCameraViewController = VNDocumentCameraViewController()
      documentCameraViewController.delegate = self
      self.webview.window?.rootViewController?.present(documentCameraViewController, animated: true)
    }
  }

  func reportProgress(_ page: Int, of count: Int) {
    trigger("pdfprogress", data: ["page": page, "total": count])
  }

  func documentCameraViewController(
    _ controller: VNDocumentCameraViewController, didFinishWith scan: VNDocumentCameraScan
  ) {
    // Initialize progress report and hide camera
    self.reportProgress(0, of: scan.pageCount)
    controller.dismiss(animated: true)

    // Process images into PDF
    let images: [UIImage] = (0..<scan.pageCount).map { scan.imageOfPage(at: $0) }
    let data: Data = self.createSearchablePDF(from: images)

    if let pdfDocument = PDFDocument(data: data) {
      pdfDocument.documentAttributes?[PDFDocumentAttribute.creatorAttribute] =
        "FragDenStaat Scanner"
      pdfDocument.write(toFile: self.pdfPath!)
    }

    self.invoke?.resolve(ScanResponse(path: self.pdfPath!))
  }

  func documentCameraViewControllerDidCancel(_ controller: VNDocumentCameraViewController) {
    controller.dismiss(animated: true)
    self.invoke?.resolve(ScanResponse(path: nil))
  }

  func documentCameraViewController(
    _ controller: VNDocumentCameraViewController, didFailWithError error: Error
  ) {
    controller.dismiss(animated: true)
    self.invoke?.reject(error.localizedDescription)
  }

  private func recognizeText(from image: CGImage) -> [VNRecognizedText] {
    var textObservations: [VNRecognizedText] = []
    let recognizeTextRequest = VNRecognizeTextRequest { request, error in

      guard error == nil else { return }

      guard let observations = request.results as? [VNRecognizedTextObservation] else { return }

      for observation in observations {
        guard let candidate = observation.topCandidates(1).first else {
          continue
        }
        textObservations.append(candidate)
      }
    }

    recognizeTextRequest.recognitionLevel = .accurate

    let requestHandler = VNImageRequestHandler(cgImage: image, options: [:])
    try? requestHandler.perform([recognizeTextRequest])

    return textObservations
  }

  func createSearchablePDF(from images: [UIImage]) -> Data {
    let data = UIGraphicsPDFRenderer().pdfData { (context) in

      var imageCount = 0
      images.forEach { image in
        guard let cgImage = image.cgImage else {
          return
        }
        let recognizedText: [VNRecognizedText] = self.recognizeText(from: cgImage)

        let pageWidth = image.size.width
        let pageHeight = image.size.height
        let pageRect = CGRect(x: 0, y: 0, width: pageWidth, height: pageHeight)

        context.beginPage(withBounds: pageRect, pageInfo: [:])

        recognizedText.forEach { text in
          self.writeText(recognizedText: text, bounds: pageRect)
        }
        image.draw(in: pageRect)
        imageCount += 1
        reportProgress(imageCount, of: images.count)
      }

    }

    return data
  }

  private func writeText(
    recognizedText: VNRecognizedText, bounds: CGRect
  ) {

    let text = recognizedText.string
    let pageWidth = bounds.size.width
    let pageHeight = bounds.size.height

    let start = text.index(text.startIndex, offsetBy: 0)
    let end = text.index(text.endIndex, offsetBy: 0)
    let bBox = try? recognizedText.boundingBox(for: start..<end)

    guard let boundingBox = bBox else {
      return
    }

    let transform = CGAffineTransform(scaleX: 1, y: -1).translatedBy(x: 0, y: -pageHeight)
    let rect: CGRect = VNImageRectForNormalizedRect(
      boundingBox.boundingBox, Int(pageWidth), Int(pageHeight)
    )
    .applying(transform)

    let fontSize = FontSizeCalculator.shared.fontSizeThatFits(text: text, rectSize: rect.size)
    let font = UIFont.systemFont(ofSize: fontSize)

    let attributedString = NSAttributedString(
      string: text,
      attributes: [
        NSAttributedString.Key.font: font
      ]
    )

    attributedString.draw(in: rect)
  }
}

@_cdecl("init_plugin_documentcamera")
func initPlugin() -> Plugin {
  return DocumentCameraPlugin()
}
