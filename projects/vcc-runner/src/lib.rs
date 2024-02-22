use wasmtime::{
    component::{Component, Func, Instance, Linker, ResourceTable, Val},
    AsContextMut, Config, Engine, Module, Store,
};
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct WasiRunner {
    engine: Engine,
    store: Store<ContextView>,
    linker: Linker<ContextView>,
}

impl Default for WasiRunner {
    fn default() -> Self {
        let mut config = Config::new();
        config.wasm_reference_types(true);
        config.wasm_function_references(true);
        Self::new(config).unwrap()
    }
}

impl WasiRunner {
    pub fn new(mut config: Config) -> anyhow::Result<Self> {
        config.async_support(true);
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::<ContextView>::new(&engine);
        linker.allow_shadowing(true);
        wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stderr();
        let store = Store::new(&engine, ContextView::new(ResourceTable::default(), builder.build()));
        Ok(Self { engine, linker, store })
    }

    pub async fn load_wast(mut self, wat: &str) -> anyhow::Result<WasiInstance> {
        let component = Component::new(&self.engine, &wat.as_bytes())?;
        let instance = self.linker.instantiate_async(&mut self.store, &component).await?;
        Ok(WasiInstance { store: self.store, instance })
    }
}

pub struct WasiInstance {
    store: Store<ContextView>,
    instance: Instance,
}

impl WasiInstance {
    pub async fn call_function(&mut self, name: &str, inputs: &[Val]) -> anyhow::Result<Vec<Val>> {
        let f = match self.instance.get_func(&mut self.store, name) {
            Some(s) => s,
            None => Err(anyhow::anyhow!("Function not found: {}", name))?,
        };
        let mut result = vec![];
        f.call_async(&mut self.store, inputs, &mut result).await?;
        Ok(result)
    }
}

pub struct ContextView {
    wasi: WasiCtx,
    resources: ResourceTable,
}

impl ContextView {
    fn new(table: ResourceTable, wasi: WasiCtx) -> Self {
        Self { resources: table, wasi }
    }
}

impl WasiView for ContextView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
