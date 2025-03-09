#if defined _srcwr_wasm_included
	#endinput
#endif
#define _srcwr_wasm_included
#pragma semicolon 1


methodmap WasmModule < Handle {
	public native WasmModule(const char[] path);

	public native WasmInstance Instantiate(bool enable_wasi);
}

methodmap WasmInstance < Handle {
	public native void RunInBackground();
	public native void Stop();

	///// WASI only methods:
	//
	public native void args(const char[][] argv={}, int argc=0);
	// Inherit environment variables from SRCDS.
	public native void inherit_env();
	//
	//public native void set_env(const char[] key, const char[] fmt, ...);
}


public Extension __ext_srcwrwasm =
{
	name = "srcwrwasm",
	file = "srcwrwasm.ext",
#if defined AUTOLOAD_EXTENSIONS
	autoload = 1,
#else
	autoload = 0,
#endif
#if defined REQUIRE_EXTENSIONS
	required = 1,
#else
	required = 0,
#endif
};

#if !defined REQUIRE_EXTENSIONS
public void __ext_srcwrwasm_SetNTVOptional()
{
	MarkNativeAsOptional("WasmModule.WasmModule");
	MarkNativeAsOptional("WasmModule.Instantiate");

	MarkNativeAsOptional("WasmInstance.RunInBackground");
	MarkNativeAsOptional("WasmInstance.Stop");
	MarkNativeAsOptional("WasmInstance.args");
	MarkNativeAsOptional("WasmInstance.inherit_env");
}
#endif
