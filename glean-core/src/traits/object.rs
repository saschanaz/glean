// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::Deserialize;
pub use serde::Serialize as SerdeSerialize;
use serde_json::Value as JsonValue;

/// no
pub trait ObjectObject: for<'de> Deserialize<'de> {
    /// no
    fn into_serialized_object(self) -> JsonValue;
}

impl<V> ObjectObject for V
where
    V: SerdeSerialize,
    V: for<'de> Deserialize<'de>,
{
    fn into_serialized_object(self) -> JsonValue {
        serde_json::to_value(self).expect("failed to serialize object. This should be impossible.")
    }
}
