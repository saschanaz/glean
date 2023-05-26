// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::common_metric_data::CommonMetricDataInternal;
use crate::error_recording::{record_error, test_get_num_recorded_errors, ErrorType};
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

use serde_json::Value as JsonValue;

/// A counter metric.
///
/// Used to count things.
/// The value can only be incremented, not decremented.
#[derive(Clone, Debug)]
pub struct ObjectMetric {
    meta: Arc<CommonMetricDataInternal>,
}

impl MetricType for ObjectMetric {
    fn meta(&self) -> &CommonMetricDataInternal {
        &self.meta
    }
}

// IMPORTANT:
//
// When changing this implementation, make sure all the operations are
// also declared in the related trait in `../traits/`.
impl ObjectMetric {
    /// Creates a new counter metric.
    pub fn new(meta: CommonMetricData) -> Self {
        Self {
            meta: Arc::new(meta.into()),
        }
    }

    /// Sets to the specified structure.
    ///
    /// # Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `value` - the value to set.
    #[doc(hidden)]
    pub fn set_sync(&self, glean: &Glean, value: JsonValue) {
        let value = Metric::Object(serde_json::to_string(&value).unwrap());
        dbg!(&value);
        glean.storage().record(glean, &self.meta, &value)
    }

    /// Sets to the specified structure.
    ///
    /// # Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `value` - the value to set.
    pub fn set(&self, value: JsonValue) {
        let metric = self.clone();
        crate::launch_with_glean(move |glean| metric.set_sync(glean, value))
    }

    /// Record an `InvalidValue` error for this metric.
    ///
    /// Only to be used by the RLB.
    pub fn record_invalid_value(&self) {
        let metric = self.clone();
        crate::launch_with_glean(move |glean| {
            let msg = "Value did not match predefined schema";
            record_error(glean, &metric.meta, ErrorType::InvalidValue, msg, None);
        });
    }

    /// Sets to the specified structure from a string in JSON encoding
    ///
    /// # Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `value` - the value to set.
    pub fn set_str(&self, value: String) {
        let metric = self.clone();
        crate::launch_with_glean(move |glean| {
            let value = match serde_json::from_str(&value) {
                Ok(s) => s,
                Err(_) => {
                    let msg = "not valiod json";
                    record_error(glean, &metric.meta, ErrorType::InvalidValue, msg, None);
                    return;
                }
            };
            metric.set_sync(glean, value)
        })
    }

    /// Get current value
    #[doc(hidden)]
    pub fn get_value<'a, S: Into<Option<&'a str>>>(
        &self,
        glean: &Glean,
        ping_name: S,
    ) -> Option<String> {
        let queried_ping_name = ping_name
            .into()
            .unwrap_or_else(|| &self.meta().inner.send_in_pings[0]);

        match StorageManager.snapshot_metric_for_test(
            glean.storage(),
            queried_ping_name,
            &self.meta.identifier(glean),
            self.meta.inner.lifetime,
        ) {
            Some(Metric::Object(o)) => Some(o),
            _ => None,
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Gets the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, ping_name: Option<String>) -> Option<String> {
        crate::block_on_dispatcher();
        crate::core::with_glean(|glean| self.get_value(glean, ping_name.as_deref()))
    }

    /// **Exported for test purposes.**
    ///
    /// Gets the number of recorded errors for the given metric and error type.
    ///
    /// # Arguments
    ///
    /// * `error` - The type of error
    /// * `ping_name` - represents the optional name of the ping to retrieve the
    ///   metric for. inner to the first value in `send_in_pings`.
    ///
    /// # Returns
    ///
    /// The number of errors reported.
    pub fn test_get_num_recorded_errors(&self, error: ErrorType) -> i32 {
        crate::block_on_dispatcher();

        crate::core::with_glean(|glean| {
            test_get_num_recorded_errors(glean, self.meta(), error).unwrap_or(0)
        })
    }
}
