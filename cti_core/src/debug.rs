#[allow(dead_code)]
pub(crate) fn list_env_vars() {
    for (k, v) in std::env::vars() {
        log::debug!("[ENV] {k} = {v}");
    }
}
