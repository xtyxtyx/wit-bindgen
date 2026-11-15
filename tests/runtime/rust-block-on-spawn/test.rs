include!(env!("BINDINGS"));

struct Component;

export!(Component);

impl crate::exports::test::rust_block_on_spawn::i::Guest for Component {
    async fn ping() {
        wit_bindgen::yield_async().await;
    }
}
