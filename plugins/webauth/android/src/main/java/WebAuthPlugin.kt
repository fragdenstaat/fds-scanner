package de.fragdenstaat.scanner.webauth

import android.app.Activity
import android.content.Intent
import android.net.Uri
import androidx.browser.customtabs.CustomTabsIntent
import app.tauri.Logger
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin


@InvokeArg
class WebAuthArgs {
  lateinit var url: String
  lateinit var redirectUrl: String
}

@TauriPlugin
class WebAuthPlugin(private val activity: Activity): Plugin(activity) {
    private var savedInvoke: Invoke? = null
    private var loginOpened: Boolean = false
    private val CHROME_PACKAGE_NAME: String = "com.android.chrome"

    @Command
    fun start_auth(invoke: Invoke) {
      Logger.info("WebAuthPlugin", "start_auth")
        val args = invoke.parseArgs(WebAuthArgs::class.java)
        savedInvoke = invoke

        val builder = CustomTabsIntent.Builder()
        builder.setBookmarksButtonEnabled(false)
        builder.setDownloadButtonEnabled(false)
        val customTabsIntent = builder.build()
        customTabsIntent.intent.addFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP);
        customTabsIntent.intent.setPackage(CHROME_PACKAGE_NAME);
        loginOpened = true
        customTabsIntent.launchUrl(activity, Uri.parse(args.url))
    }

    override fun onResume() {
        Logger.info("WebAuth", "onResume called")
        if (loginOpened) {
            loginOpened = false
            savedInvoke?.reject("Canceled")
        }
    }

    override fun onNewIntent(intent: Intent) {
        loginOpened = false
        Logger.info("WebAuth", "onNewIntent: $intent")
        if (intent.action == Intent.ACTION_VIEW) {
            val url = intent.data.toString()
            Logger.info("WebAuth", "onNewIntent View: $url")
            val ret = JSObject()
            ret.put("url", url)
            savedInvoke?.resolve(ret)
        }
    }
}
