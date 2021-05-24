#[cfg(feature = "gcc-forwarder")]
include!("gcc_forwarder.rs");

#[cfg(not(feature = "gcc-forwarder"))]
include!("zig_cc.rs");
