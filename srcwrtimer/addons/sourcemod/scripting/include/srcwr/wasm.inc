#if defined _srcwr_wasm_included
	#endinput
#endif
#define _srcwr_wasm_included
#pragma semicolon 1


methodmap SRCWRWASM < Handle {
	public native SRCWRWASM();
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
	MarkNativeAsOptional("SRCWRWASM.SRCWRWASM");
}
#endif
