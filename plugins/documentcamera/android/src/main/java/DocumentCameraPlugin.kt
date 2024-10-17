package de.fragdenstaat.scanner.documentcamera

import android.app.Activity
import android.content.Intent
import androidx.activity.result.ActivityResult
import app.tauri.Logger
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class DocumentCameraArgs {
  lateinit var path: String
}

enum class DocumentCameraResultType {
    SUCCESS, ERROR, CANCELED
}

@TauriPlugin
class DocumentCameraPlugin(private val activity: Activity): Plugin(activity) {
    companion object {
        var RESULT_EXTRA_PREFIX = ""
        const val RESULT_TYPE = "type"
        const val PATH = "path"
        const val RESULT_ERROR_MESSAGE = "errorMessage"
    }

    @Command
    fun scan(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(DocumentCameraArgs::class.java)

            RESULT_EXTRA_PREFIX = activity.packageName + "."
            val intent = Intent(
                activity,
                DocumentCameraActivity::class.java
            )
            intent.putExtra(PATH, args.path)

            startActivityForResult(invoke, intent, "scanResult")
        } catch (ex: Exception) {
            val message = ex.message ?: "Failed to scan document"
            Logger.error(message)
            invoke.reject(message)
        }
    }

    @ActivityCallback
    private fun scanResult(invoke: Invoke, result: ActivityResult) {
        Logger.info("DocumentCamera", "scanResult called $result")

        val resultCode = result.resultCode

        // If the system canceled the activity, we might get RESULT_CANCELED in resultCode.
        // In that case return that immediately, because there won't be any data.
        if (resultCode == Activity.RESULT_CANCELED) {
            Logger.info("DocumentCamera", "scanResult canceled!")
            val ret = JSObject()
            ret.put("path", null)
            invoke.resolve(ret)
            return
        }

        // Convert the string result type to an enum
        val data = result.data
        val resultTypeName = data?.getStringExtra(
            RESULT_EXTRA_PREFIX + RESULT_TYPE
        )
        if (resultTypeName == null) {
            invoke.reject(
                "Missing data in the result of the activity"
            )
            return
        }
        val resultType = try {
            DocumentCameraResultType.valueOf(resultTypeName)
        } catch (e: IllegalArgumentException) {
            invoke.reject(
                "Invalid data in the result of the activity",
            )
            return
        }
        val errorMessage = data.getStringExtra(
            RESULT_EXTRA_PREFIX + RESULT_ERROR_MESSAGE
        )
        when (resultType) {
            DocumentCameraResultType.SUCCESS -> {
                val resultPath = data.getStringExtra(
                    RESULT_EXTRA_PREFIX + PATH
                )
                val ret = JSObject()
                ret.put("path", resultPath)
                invoke.resolve(ret)
                Logger.info("DocumentCamera", "scanResult returned path $resultPath")
            }
            DocumentCameraResultType.CANCELED -> {
                val ret = JSObject()
                ret.put("success", false)
                invoke.resolve(ret)
            }
            DocumentCameraResultType.ERROR -> {
                // The user cancelled, the system cancelled, or some error occurred.
                // If the user cancelled, errorMessage is the text of the "negative" button,
                // which is not especially descriptive.

                invoke.reject(errorMessage)
            }
        }
    }
}
