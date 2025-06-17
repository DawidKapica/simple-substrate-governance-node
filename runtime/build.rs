use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new().with_current_project().build();
}