#pragma once

typedef unsigned bigbool;

extern "C" {

bool rust_init(char* error, size_t errorlen, const char* sourcemod_data_path);

}
