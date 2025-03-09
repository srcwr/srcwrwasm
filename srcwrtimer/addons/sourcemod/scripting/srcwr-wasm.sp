#include <srcwr/wasm>

public void OnPluginStart()
{
	char plugins[PLATFORM_MAX_PATH];
	BuildPath(Path_SM, plugins, sizeof(plugins), "wasm/plugins");

	DirectoryListing dir = OpenDirectory(plugins);
	char name[PLATFORM_MAX_PATH];
	FileType type;
	while (dir && dir.GetNext(name, sizeof(name), type))
	{
		// . & ..
		if (name[0] == '.' && (name[1] == '\0' || (name[1] == '.' && name[2] == '\0')))
		{
			continue;
		}

		int ext = FindCharInString(name, '.', true);
		if (StrEqual("autoload", name[1+ext]))
		{
			name[ext] = '\0';
			char buf[PLATFORM_MAX_PATH];
			FormatEx(buf, sizeof(buf), "%s/%s.wat", plugins, name);
			SRCWRWASM wasm = new SRCWRWASM(buf, true);
			if (wasm && wasm.RunInBackground())
				PrintToServer(">>> wasm: started %s.wat in the background", name);
			FormatEx(buf, sizeof(buf), "%s/%s.wasm", plugins, name);
			if (wasm && wasm.RunInBackground())
				PrintToServer(">>> wasm: started %s.wasm in the background", name);
		}
	}
	delete dir;
}
