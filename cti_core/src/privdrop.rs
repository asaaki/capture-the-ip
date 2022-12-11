#[cfg(target_os = "linux")]
#[inline]
#[allow(clippy::panic)]
pub fn privdrop() {
    if nix::unistd::Uid::effective().is_root() {
        privdrop::PrivDrop::default()
            .chroot("/app")
            .user("nobody")
            .apply()
            .unwrap_or_else(|e| {
                panic!("Failed to drop privileges: {}", e);
            });
    } else {
        log::debug!("No privilege drop needed. Thanks for playing nice. ;-)")
    }
}

#[cfg(not(target_os = "linux"))]
#[inline]
pub fn privdrop() {}
