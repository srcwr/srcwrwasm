#include "../../../srcwrtimer/extshared/src/extension.h"
#include "../../../srcwrtimer/extshared/src/coreident.hpp"
#include "rust_exports_wasm.h"


extern const sp_nativeinfo_t WasmNatives[];


void MyExtension::OnHandleDestroy(HandleType_t type, void* object) {}
bool MyExtension::GetHandleApproxSize(HandleType_t type, void* object, unsigned int* size) { return false; }


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

static cell_t N_SRCWRWASM_SRCWRWASM(IPluginContext* ctx, const cell_t* params)
{
	return 0;
}

extern const sp_nativeinfo_t WasmNatives[] = {
	{"SRCWRWASM.SRCWRWASM", N_SRCWRWASM_SRCWRWASM},
	{NULL, NULL}
};
