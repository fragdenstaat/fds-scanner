package de.fragdenstaat.scanner.documentcamera

import android.app.Activity
import android.content.Intent
import android.content.IntentSender
import android.os.Bundle
import androidx.activity.result.ActivityResult
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.IntentSenderRequest
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import com.google.mlkit.vision.documentscanner.GmsDocumentScannerOptions
import com.google.mlkit.vision.documentscanner.GmsDocumentScanning
import com.google.mlkit.vision.documentscanner.GmsDocumentScanningResult
import app.tauri.Logger


class DocumentCameraActivity : AppCompatActivity() {
    private lateinit var scannerLauncher: ActivityResultLauncher<IntentSenderRequest>
    private lateinit var path: String
    private var resultsSent = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val intent = intent
        val intentPath = intent.getStringExtra(DocumentCameraPlugin.PATH)
        if (intentPath == null) {
            finishActivity(DocumentCameraResultType.ERROR, "Missing path")
            return
        }
        path = intentPath

        scannerLauncher = registerForActivityResult(ActivityResultContracts.StartIntentSenderForResult()) { result ->
            handleActivityResult(result)
        }

        val options =
            GmsDocumentScannerOptions.Builder()
                .setScannerMode(GmsDocumentScannerOptions.SCANNER_MODE_FULL)
                .setResultFormats(GmsDocumentScannerOptions.RESULT_FORMAT_PDF)
                .setGalleryImportAllowed(false)

        GmsDocumentScanning.getClient(options.build())
            .getStartScanIntent(this)
            .addOnSuccessListener { intentSender: IntentSender ->
                scannerLauncher.launch(IntentSenderRequest.Builder(intentSender).build())
            }
            .addOnFailureListener { e: Exception ->
                finishActivity(DocumentCameraResultType.ERROR, e.message)
            }
            .addOnCanceledListener {
                finishActivity(DocumentCameraResultType.CANCELED)
            }
    }

//    override fun onStop() {
//        // Call the superclass method first.
//        super.onStop()
//        Logger.info("DocumentCamera", "onStop called: $resultsSent")
//        if (resultsSent) {
//            return
//        }
//        setResult(Activity.RESULT_CANCELED, intent)
//    }

    private fun handleActivityResult(activityResult: ActivityResult) {
        val resultCode = activityResult.resultCode
        val result = GmsDocumentScanningResult.fromActivityResultIntent(activityResult.data)
        if (resultCode == Activity.RESULT_OK && result != null) {
            result.pdf?.uri?.path?.let { pdfPath ->
//                Logger.info("DocumentCamera", "copying $pdfPath to $path")
//                File(pdfPath).copyTo(File(path));
                finishActivity(DocumentCameraResultType.SUCCESS, "", pdfPath)
            }
        } else if (resultCode == Activity.RESULT_CANCELED) {
            finishActivity(DocumentCameraResultType.CANCELED)
        } else {
            finishActivity(DocumentCameraResultType.ERROR)
        }
    }

    @JvmOverloads
    fun finishActivity(
        resultType: DocumentCameraResultType = DocumentCameraResultType.SUCCESS,
        errorMessage: String? = "",
        path: String? = null
    ) {
        val intent = Intent()
        val prefix = DocumentCameraPlugin.RESULT_EXTRA_PREFIX
        intent
            .putExtra(prefix + DocumentCameraPlugin.RESULT_TYPE, resultType.toString())
            .putExtra(
                prefix + DocumentCameraPlugin.RESULT_ERROR_MESSAGE,
                errorMessage
            )
        if (path != null) {
            intent.putExtra(prefix + DocumentCameraPlugin.PATH, path)
        }
        setResult(Activity.RESULT_OK, intent)
        resultsSent = true
        finish()
    }
}