use opendal::{services::Memory, Operator, OperatorBuilder};

mod common;



#[tokio::test]
async fn test_underlay_layer() -> anyhow::Result<()> {
    let lower = Memory::default().build()?;
    let lower_op = OperatorBuilder::new(lower).finish();

    let upper
    let upper_op = Operator::new(Memory::default())?.finish();

    upper_op.layer(layer)

    let layer = UnderlayLayer::new(lower);
    let accessor = layer.layer(upper);
    let _ = accessor.read("foo", OpRead::default()).await;

    Ok(())
}
