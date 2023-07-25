// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::ffi::{c_int, c_void};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use once_cell::sync::Lazy;
use tempfile::Builder;

use glean::{private::PingType, ClientInfoMetrics, ConfigurationBuilder};

static ALLOW_THREAD_SPAWNED: AtomicU32 = AtomicU32::new(0);

#[allow(non_camel_case_types)]
type pthread_ft = extern "C" fn(
    native: *mut libc::pthread_t,
    attr: *const libc::pthread_attr_t,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
    value: *mut c_void,
) -> c_int;

#[no_mangle]
pub unsafe extern "C" fn pthread_create(
    native: *mut libc::pthread_t,
    attr: *const libc::pthread_attr_t,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
    value: *mut c_void,
) -> c_int {
    let name = b"pthread_create\0".as_ptr() as *const i8;
    let symbol = libc::dlsym(libc::RTLD_NEXT, name);
    if symbol.is_null() {
        panic!("oops, error in dlsym.");
    }

    let real_pthread_create = &*(&symbol as *const *mut _ as *const pthread_ft);

    if ALLOW_THREAD_SPAWNED.fetch_add(1, Ordering::SeqCst) == 4 {
        return -1;
    }

    return real_pthread_create(native, attr, f, value);
}

pub mod glean_metrics {
    use glean::{private::BooleanMetric, CommonMetricData, Lifetime};

    #[allow(non_upper_case_globals)]
    pub static sample_boolean: once_cell::sync::Lazy<BooleanMetric> =
        once_cell::sync::Lazy::new(|| {
            BooleanMetric::new(CommonMetricData {
                name: "sample_boolean".into(),
                category: "test.metrics".into(),
                send_in_pings: vec!["prototype".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            })
        });
}

#[allow(non_upper_case_globals)]
pub static PrototypePing: Lazy<PingType> =
    Lazy::new(|| PingType::new("prototype", true, true, vec![]));

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    };

    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    glean_metrics::sample_boolean.set(true);

    PrototypePing.submit(None);

    glean::shutdown();
}
