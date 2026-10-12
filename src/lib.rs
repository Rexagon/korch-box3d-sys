#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(
    clippy::approx_constant,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::useless_transmute
)]

#[cfg(feature = "glam")]
mod glam;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// The cookie is stolen. Rust has its own `Default` which copies the underlying logic.
///
/// NOTE: Make it sync with `B3_SECRET_COOKIE` (`box3d/src/core.h`).
pub const B3_SECRET_COOKIE: u32 = 1152023;

#[cfg(test)]
mod tests {
    #[test]
    fn links_box3d() {
        unsafe {
            let version = crate::b3GetVersion();
            assert_eq!(version.major, 0);
            assert_eq!(version.minor, 2);
            assert!(crate::b3GetWorldCount() >= 0);
        }
    }
}
