use glean_core::metrics::{BooleanMetric, StringMetric};
use glean_core::{storage, CommonMetricData, Glean};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_METRIC: BooleanMetric = BooleanMetric::new(CommonMetricData {
        name: "global_metric".into()
    });
}

fn main() {
    Glean::initialize();
    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "local_metric".into(),
    });

    GLOBAL_METRIC.set(true);
    local_metric.set("I can set this");

    println!("{}", storage::StorageManager.dump());
}
