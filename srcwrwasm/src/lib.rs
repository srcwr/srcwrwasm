mod natives_wasm;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extshared::smext_conf_boilerplate_extension_info!();
extshared::smext_conf_boilerplate_load_funcs!();
