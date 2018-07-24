#[cfg(feature = "app")]
pub extern crate kubos_app as app;

#[cfg(feature = "telemetry")]
pub extern crate kubos_telemetry as telemetry;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
