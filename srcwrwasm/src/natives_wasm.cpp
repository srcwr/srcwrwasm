#include "../../../srcwrtimer/extshared/src/extension.h"
#include "../../../srcwrtimer/extshared/src/coreident.hpp"
#include "rust_exports_wasm.h"


HandleType_t g_WasmModule = 0;
HandleType_t g_WasmInstance = 0;
extern const sp_nativeinfo_t WasmNatives[];


void MyExtension::OnHandleDestroy(HandleType_t type, void* object)
{
	if (type == g_WasmModule)
		rust_handle_destroy_WasmModule(object);
	else if (type == g_WasmInstance)
		rust_handle_destroy_WasmInstance(object);
}
bool MyExtension::GetHandleApproxSize(HandleType_t type, void* object, unsigned int* size)
{
	return false;
}


bool Extension_OnLoad(char* error, size_t maxlength)
{
	char sourcemod_data_path[PLATFORM_MAX_PATH];
	smutils->BuildPath(Path_SM, sourcemod_data_path, sizeof(sourcemod_data_path), "data");
	rust_init(error, maxlength, sourcemod_data_path);

	sharesys->AddNatives(myself, WasmNatives);
	return true;
}

void Extension_OnUnload()
{
}

void Extension_OnAllLoaded() {}

static cell_t N_WasmModule_WasmModule(IPluginContext* ctx, const cell_t* params)
{
	char* path;
	(void)ctx->LocalToString(params[1], &path);
	char pathbuf[PLATFORM_MAX_PATH];
	// TODO: Check if Path_SM or Path_Game or similar
	smutils->BuildPath(Path_Game, pathbuf, sizeof(pathbuf), "%s", path);
	return g_MyExtension.HandleOrDestroy(ctx, rust_WasmModule_WasmModule(ctx, pathbuf), g_WasmModule);
}

static cell_t N_WasmModule_Instantiate(IPluginContext* ctx, const cell_t* params)
{
	void* object;
	GET_HANDLE(params[1], object, g_WasmModule);
	bool enable_wasi = params[2] == 1;
	return g_MyExtension.HandleOrDestroy(ctx, rust_WasmModule_Instantiate(ctx, object, enable_wasi), g_WasmInstance);
}

static cell_t N_WasmInstance_RunInBackground(IPluginContext* ctx, const cell_t* params)
{
	void* object;
	GET_HANDLE(params[1], object, g_WasmInstance);
	rust_WasmInstance_RunInBackground(object);
	return 0;
}

static cell_t N_WasmInstance_Stop(IPluginContext* ctx, const cell_t* params)
{
	void* object;
	GET_HANDLE(params[1], object, g_WasmInstance);
	rust_WasmInstance_Stop(object);
	return 0;
}

static cell_t N_WasmInstance_inherit_env(IPluginContext* ctx, const cell_t* params)
{
	void* object;
	GET_HANDLE(params[1], object, g_WasmInstance);
	rust_WasmInstance_inherit_env(object);
	return 0;
}

extern const sp_nativeinfo_t WasmNatives[] = {
	{"WasmModule.WasmModule", N_WasmModule_WasmModule},
	{"WasmModule.Instantiate", N_WasmModule_Instantiate},

	{"WasmInstance.RunInBackground", N_WasmInstance_RunInBackground},
	{"WasmInstance.Stop", N_WasmInstance_Stop},

	//args
	{"WasmInstance.inherit_env", N_WasmInstance_inherit_env},
	//set_env
	{NULL, NULL}
};
