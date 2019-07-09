/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.getAndConsumeRustString
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import java.util.UUID

class UuidMetricType(
    private var handle: Long,
    private val sendInPings: List<String>
) {
    /**
     * The public constructor used by automatically generated metrics.
     */
    constructor(
        disabled: Boolean,
        category: String,
        lifetime: Lifetime,
        name: String,
        sendInPings: List<String>
    ) : this(handle = 0, sendInPings = sendInPings) {
        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_uuid_metric(
            category = category,
            name = name,
            send_in_pings = ffiPingsList,
            send_in_pings_len = sendInPings.size,
            lifetime = lifetime.ordinal,
            disabled = disabled.toByte())
    }

    protected fun finalize() {
        if (this.handle != 0L) {
            val error = RustError.ByReference()
            LibGleanFFI.INSTANCE.glean_destroy_uuid_metric(this.handle, error)
        }
    }

    private fun shouldRecord(): Boolean {
        // Don't record metrics if we aren't initialized
        if (!Glean.isInitialized()) {
            return false
        }

        return LibGleanFFI.INSTANCE.glean_uuid_should_record(Glean.handle, this.handle).toBoolean()
    }

    /**
     * Generate a new UUID value and set it in the metric store.
     */
    fun generateAndSet(): UUID? {
        // Even if `set` is already checking if we're allowed to record,
        // we need to check here as well otherwise we'd return a `UUID`
        // that won't be stored anywhere.
        if (!shouldRecord()) {
            return null
        }

        val uuid = UUID.randomUUID()
        set(uuid)
        return uuid
    }

    /**
     * Explicitly set an existing UUID value
     *
     * @param value a valid [UUID] to set the metric to
     */
    fun set(value: UUID) {
        if (!shouldRecord()) {
            return
        }

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_uuid_set(
                Glean.handle,
                this@UuidMetricType.handle,
                value.toString())
        }
    }

    /**
     * Tests whether a value is stored for the metric for testing purposes only. This function will
     * attempt to await the last task (if any) writing to the the metric's storage engine before
     * returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return true if metric value exists, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testHasValue(pingName: String = sendInPings.first()): Boolean {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        val res = LibGleanFFI.INSTANCE.glean_uuid_test_has_value(Glean.handle, this.handle, pingName)
        return res.toBoolean()
    }

    /**
     * Returns the stored value for testing purposes only. This function will attempt to await the
     * last task (if any) writing to the the metric's storage engine before returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return value of the stored metric
     * @throws [NullPointerException] if no value is stored
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetValue(pingName: String = sendInPings.first()): UUID {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        if (!testHasValue(pingName)) {
            throw NullPointerException()
        }
        val ptr = LibGleanFFI.INSTANCE.glean_uuid_test_get_value(Glean.handle, this.handle, pingName)!!
        return UUID.fromString(ptr.getAndConsumeRustString())
    }
}