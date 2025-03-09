#pragma once

namespace SourcePawn {
class IPluginContext;
}

extern "C" {

bool rust_init(char* error, size_t errorlen, const char* sourcemod_data_path);

void rust_handle_destroy_WasmModule(void* object);
void rust_handle_destroy_WasmInstance(void* object);

void* rust_WasmModule_WasmModule(IPluginContext* ctx, const char* path);
void* rust_WasmModule_Instantiate(IPluginContext* ctx, void* object, bool enable_wasi);

void rust_WasmInstance_RunInBackground(void* object);
void rust_WasmInstance_Stop(void* object);

//void rust_WasmInstance_args(void* object);
void rust_WasmInstance_inherit_env(void* object);
//void rust_WasmInstance_set_env(void* object, const char* key, const char* value);

}
