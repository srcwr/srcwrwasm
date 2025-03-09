// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2025 rtldg <rtldg@protonmail.com>

// https://docs.rs/wasmtime/latest/wasmtime/
// https://docs.rs/wasmtime-wasi/latest/wasmtime_wasi/

use std::{
	ffi::c_char,
	path::PathBuf,
	sync::{LazyLock, OnceLock},
};

use anyhow::Context;
use wasmtime::{Config, Engine};

static CACHE_CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

static ENGINE: LazyLock<Engine> = LazyLock::new(|| {
	Engine::new(
		Config::new()
			//.allocation_strategy(Pooling?)
			//.async_support(true)
			.cache_config_load(CACHE_CONFIG_PATH.get().unwrap())
			.unwrap()
			//.consume_fuel(true)
			.cranelift_opt_level(wasmtime::OptLevel::Speed) // also SpeedAndSize
			//.epoch_interruption(true)
			.max_wasm_stack(2 * 1024 * 1024)
			.parallel_compilation(true)
			.strategy(wasmtime::Strategy::Cranelift)
			//.wasm_gc(true) // >still in progress and generally not ready for primetime
			//.wasm_threads(true) // still WIP too
			.wasm_wide_arithmetic(true),
	)
	.unwrap()
});

#[unsafe(no_mangle)]
pub extern "C" fn rust_init(ebuf: *mut u8, ebufsz: usize, sourcemod_data_path: *const c_char) -> bool {
	let Err(e) = rust_init_inner(sourcemod_data_path) else {
		return true;
	};
	let _ = extshared::write_to_sp_buf(e.to_string().as_bytes(), None, ebuf, 0, ebufsz, 0);
	false
}

fn rust_init_inner(sourcemod_data_path: *const c_char) -> anyhow::Result<()> {
	let sourcemod_data_path = extshared::strxx(sourcemod_data_path, false, -1).unwrap();
	rust_init_files(sourcemod_data_path)?;
	Ok(())
}

fn rust_init_files(sourcemod_data_path: &str) -> anyhow::Result<()> {
	let mut path = std::fs::canonicalize(sourcemod_data_path)?;
	path.push("wasm");
	path.push("cache");
	std::fs::create_dir_all(&path).context("Failed to create sourcemod/data/wasm/cache directory")?;
	path.set_file_name("plugins");
	std::fs::create_dir(&path).context("Failed to create sourcemod/data/wasm/plugins directory")?;

	let cache = format!(
		r#"
		# WARNING: Overwritten by srcwrwasm on extension load. Do not edit!
		# https://bytecodealliance.github.io/wasmtime/cli-cache.html
		[cache]
		enabled = true
		directory = "{}"
		cleanup-interval = "5h"
		files-total-size-soft-limit = "1Gi"
		"#,
		path.display()
	);

	path.set_file_name("cache.toml"); // sourcemod/data/wasm/cache.toml
	std::fs::write(&path, &cache).context("Failed to write sourcemod/data/wasm/cache.toml")?;
	let _ = CACHE_CONFIG_PATH.get_or_init(|| path);

	Ok(())
}
