use k8s_metrics::QuantityExt;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

pub fn quantity_to_float(q: &Quantity) -> Result<f64, ()> {
    match q.to_f64() {
        Ok(f) => Ok(f),
        Err(_) => Err(()),
    }
}

pub fn quantity_to_int(q: &Quantity) -> Result<i64, ()> {
    match q.to_memory() {
        Ok(i) => Ok(i),
        Err(_) => Err(()),
    }
}
