/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// This implements the developer facing API for recording URL metrics.
///
/// Instances of this class type are automatically generated by the parsers at build time,
/// allowing developers to record values that were previously registered in the metrics.yaml file.
///
/// The URL API only exposes the `UrlMetricType.set(_:)` method, which takes care of validating the input
/// data and making sure that limits are enforced.
public class UrlMetricType {
    let inner: UrlMetric

    /// The public constructor used by automatically generated metrics.
    public init(_ meta: CommonMetricData) {
        self.inner = UrlMetric(meta)
    }

    /// Set a URL value.
    ///
    /// - parameters:
    ///     * url This is a user defined url value. If the length of the url exceeds
    ///             the maximum length, it will be not be recorded.
    public func set(url: URL) {
        let absolute = url.absoluteString
        self.set(absolute)
    }

    /// Set a URL value.
    ///
    /// - parameters:
    ///     * value This is a user defined url value. If the length of the url exceeds
    ///             the maximum length, it will be not be recorded.
    public func set(_ value: String) {
        self.inner.set(value)
    }

    /// Returns the stored value for testing purposes only. This function will attempt to await the
    /// last task (if any) writing to the the metric's storage engine before returning a value.
    ///
    /// Throws a `Missing value` exception if no value is stored
    ///
    /// - parameters:
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    ///
    /// - returns:  value of the stored metric
    public func testGetValue(_ pingName: String? = nil) -> String? {
        return inner.testGetValue()
    }

    /// Returns the number of errors recorded for the given metric.
    ///
    /// - parameters:
    ///     * errorType: The type of error recorded.
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    ///
    /// - returns: The number of errors recorded for the metric for the given error type.
    public func testGetNumRecordedErrors(_ error: ErrorType, pingName: String? = nil) -> Int32 {
        inner.testGetNumRecordedErrors(error, pingName)
    }
}
