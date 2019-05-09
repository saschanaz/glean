/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError

// import mozilla.components.service.glean.Dispatchers
// import mozilla.components.service.glean.storages.CountersStorageEngine
// import mozilla.components.support.base.log.logger.Logger

/**
 * This implements the developer facing API for recording counter metrics.
 *
 * Instances of this class type are automatically generated by the parsers at build time,
 * allowing developers to record values that were previously registered in the metrics.yaml file.
 *
 * The counter API only exposes the [add] method, which takes care of validating the input
 * data and making sure that limits are enforced.
 */
class CounterMetricType(
    disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    val sendInPings: List<String>
) {

    //private val logger = Logger("glean/CounterMetricType")

    private var handle: Long

    init {
        println("New Counter: $category.$name")
        val e = RustError.ByReference()
        this.handle = LibGleanFFI.INSTANCE.glean_new_counter_metric(category, name, e)
    }

    /**
     * Add to counter value.
     *
     * @param amount This is the amount to increment the counter by, defaulting to 1 if called
     * without parameters.
     */
    fun add(amount: Int = 1) {
        /*if (!shouldRecord(logger)) {
            return
        }

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            // Delegate storing the new counter value to the storage engine.
            CountersStorageEngine.record(
                    this@CounterMetricType,
                    amount = amount
            )
        }*/
        val e = RustError.ByReference()
        LibGleanFFI.INSTANCE.glean_counter_add(Glean.handle, this.handle, amount.toLong(), e)

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
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return CountersStorageEngine.getSnapshot(pingName, false)?.get(identifier) != null*/
        return false
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
    fun testGetValue(pingName: String = sendInPings.first()): Int {
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return CountersStorageEngine.getSnapshot(pingName, false)!![identifier]!!*/
        return 1
    }
}
