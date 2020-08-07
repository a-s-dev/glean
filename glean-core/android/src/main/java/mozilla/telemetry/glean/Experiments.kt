package mozilla.telemetry.glean

import android.content.Context
import android.os.Build
import com.sun.jna.Pointer
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError
import java.util.*
import java.util.concurrent.atomic.AtomicLong
import com.google.protobuf.CodedOutputStream
import com.google.protobuf.MessageLite
import com.sun.jna.Native
import java.nio.ByteBuffer
import java.nio.ByteOrder

// A LOT OF THIS IS COPIED FOR THE SAKE OF THE PROTOTYPE, NOT COMPLETE
fun <T : MessageLite> T.toNioDirectBuffer(): Pair<ByteBuffer, Int> {
    val len = this.serializedSize
    val nioBuf = ByteBuffer.allocateDirect(len)
    nioBuf.order(ByteOrder.nativeOrder())
    val output = CodedOutputStream.newInstance(nioBuf)
    this.writeTo(output)
    output.checkNoSpaceLeft()
    return Pair(first = nioBuf, second = len)
}

open class ExperimentsInternalAPI internal constructor () {
    private var raw: AtomicLong = AtomicLong(0)

    fun initialize(
            applicationContext: Context,
            dbPath: String
    ) {
        val appCtx = MsgTypes.AppContext.newBuilder()
                .setAppId(applicationContext.packageName)
                .setAppVersion(applicationContext.packageManager.getPackageInfo(applicationContext.packageName, 0).versionName)
                .setDeviceManufacturer(Build.MANUFACTURER)
                .setLocaleCountry(
                        try {
                            Locale.getDefault().isO3Country
                        } catch (e: MissingResourceException) {
                            Locale.getDefault().country
                        }
                )
                .setLocaleLanguage(
                        try {
                            Locale.getDefault().isO3Language
                        } catch (e: MissingResourceException) {
                            Locale.getDefault().language
                        }
                )
                .setDeviceModel(Build.DEVICE)
                .build()
        val (nioBuf, len) = appCtx.toNioDirectBuffer()
        raw.set( rustCall { error ->
            val ptr = Native.getDirectBufferPointer(nioBuf)
            LibGleanFFI.INSTANCE.experiments_new(ptr, len, dbPath, error)
        })
    }

    fun getBranch(experimentName: String): String {
        var ptr = rustCall { error ->
            LibGleanFFI.INSTANCE.experiments_get_branch(raw.get(), experimentName, error)
        }
        return ptr.getAndConsumeRustString()
    }

    /**
     * Helper to read a null terminated String out of the Pointer and free it.
     *
     * Important: Do not use this pointer after this! For anything!
     */
    internal fun Pointer.getAndConsumeRustString(): String {
        return this.getRustString()
        // PLEASE INSERT A FREE HERE!!!!!!!
    }

    /**
     * Helper to read a null terminated string out of the pointer.
     *
     * Important: doesn't free the pointer, use [getAndConsumeRustString] for that!
     */
    internal fun Pointer.getRustString(): String {
        return this.getString(0, "utf8")
    }

    // In practice we usually need to be synchronized to call this safely, so it doesn't
    // synchronize itself
    private inline fun <U> nullableRustCall(callback: (RustError.ByReference) -> U?): U? {
        val e = RustError.ByReference()
        try {
            val ret = callback(e)
            if (e.isFailure()) {
                // We ignore it for now, although we shouldn't just cuz protoype
                //throw e.intoException()
            }
            return ret
        } finally {
            // This only matters if `callback` throws (or does a non-local return, which
            // we currently don't do)
            e.ensureConsumed()
        }
    }

    private inline fun <U> rustCall(callback: (RustError.ByReference) -> U?): U {
        return nullableRustCall(callback)!!
    }
}

/**
 * The main experiments object
 * ```
 */
object Experiments : ExperimentsInternalAPI()