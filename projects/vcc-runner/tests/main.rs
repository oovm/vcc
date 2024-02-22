use std::{path::Path, sync::Arc};
use vcc::Pipeline;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let builder = Pipeline::new(Arc::from("wit"), &Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    builder.run().unwrap();
}
