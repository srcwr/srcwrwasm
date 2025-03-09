// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2025 rtldg <rtldg@protonmail.com>

// https://docs.rs/wasmtime/latest/wasmtime/
// https://docs.rs/wasmtime-wasi/latest/wasmtime_wasi/

#![allow(non_snake_case)]

use std::{
	ffi::{c_char, c_void},
	path::PathBuf,
	ptr::NonNull,
	sync::{LazyLock, OnceLock},
};

use anyhow::Context;
use extshared::report_error;
use wasmtime::{Config, Engine, Module, Store, component::Linker};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub struct WasmModule {
	module: Module,
}

enum InstanceStatus {
	Building(Option<WasiCtxBuilder>),
	Built(Store<StoreState>),

}

pub struct WasmInstance {
	module: Module,
	linker: Linker<StoreState>,
	status: InstanceStatus,
}

struct StoreState {
	wasictx: Option<Box<WasiCtx>>,
	table: ResourceTable,
}
impl IoView for StoreState {
	fn table(&mut self) -> &mut ResourceTable {
		&mut self.table
	}
}
impl WasiView for StoreState {
	fn ctx(&mut self) -> &mut WasiCtx {
		self.wasictx.as_deref_mut().unwrap()
	}
}

static RT: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.thread_name("SRCWRWASM Tokio")
		.build()
		.unwrap()
});

static CACHE_CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

static ENGINE: LazyLock<Engine> = LazyLock::new(|| {
	Engine::new(
		Config::new()
			//.allocation_strategy(Pooling?)
			.async_support(true)
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

#[unsafe(no_mangle)]
pub extern "C" fn rust_WasmModule_WasmModule(ctx: NonNull<c_void>, path: *const c_char) -> Option<NonNull<WasmModule>> {
	let path = extshared::strxx(path, false, -1).unwrap();
	match rust_WasmModule_WasmModule_inner(path) {
		Ok(v) => Some(v),
		Err(e) => {
			// TODO:
			//report_error(ctx, &e);
			None
		}
	}
}

fn rust_WasmModule_WasmModule_inner(path: &str) -> anyhow::Result<NonNull<WasmModule>> {
	let boxed = Box::new(WasmModule {
		module: Module::from_file(&ENGINE, &path)?,
	});
	unsafe { Ok(NonNull::new_unchecked(Box::into_raw(boxed))) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_WasmModule_Instantiate(
	ctx: NonNull<c_void>,
	module: &WasmModule,
	enable_wasi: bool,
) -> Option<NonNull<WasmInstance>> {
	match rust_WasmModule_Instantiate_inner(module, enable_wasi) {
		Ok(v) => Some(v),
		Err(e) => {
			// TODO:
			//report_error(ctx, &e);
			None
		}
	}
}

fn rust_WasmModule_Instantiate_inner(module: &WasmModule, enable_wasi: bool) -> anyhow::Result<NonNull<WasmInstance>> {
	let mut linker = Linker::<StoreState>::new(&ENGINE);

	let wasibuilder = if enable_wasi {
		wasmtime_wasi::add_to_linker_async(&mut linker)?;
		let mut builder = WasiCtxBuilder::new();
		builder.allow_ip_name_lookup(true);
		Some(builder)
	} else {
		None
	};

	let boxed = Box::new(WasmInstance {
		module: module.module.clone(),
		linker,
		status: InstanceStatus::Building(wasibuilder),
	});
	unsafe { Ok(NonNull::new_unchecked(Box::into_raw(boxed))) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_WasmInstance_RunInBackground(instance: &mut WasmInstance) {
	todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_WasmInstance_Stop(instance: &mut WasmInstance) {
	todo!()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_WasmInstance_inherit_env(instance: &mut WasmInstance) {
	if let InstanceStatus::Building(Some(wasibuilder)) = &mut instance.status {
		wasibuilder.inherit_env();
	}
}
