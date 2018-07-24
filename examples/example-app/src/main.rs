extern crate kubos;

fn main() {
    // Comment out this line and remove "app" from the kubos feature list in Cargo.toml, and voila,
    // smaller binary!
    println!("App var = {}", kubos::app::app_var());
    println!("Tel var = {}", kubos::telemetry::TELEMETRY_VAR);
}
