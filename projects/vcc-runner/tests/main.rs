use std::path::Path;
use vcc_runner::WasiRunner;

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
    let runner = WasiRunner::default();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/component.wat");
    let bytes = std::fs::read_to_string(&path)?;
    let mut component = runner.load_wast(&bytes).await?;
    let m = component.call_function_dynamic("random-i32", &[]).await?;
    println!("{:?}", m);

    // let m: (i32,) = component.call_function("random-i32", ()).await?;
    // println!("{:?}", m);

    // let mut exports = instance.exports(&mut store);
    // let mut root = exports.root();

    // let f_one = instance.get_typed_func::<(), (i64, )>(&mut store, "tuple1")?;
    // let result = f_one.call_async(&mut store, ()).await?;
    // println!("{:?}", result);

    Ok(())
}
