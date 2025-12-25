use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the compiled WASI module (.wasm)
    #[arg(long, default_value = "demo/wasi/target/wasm32-wasip1/release/wasi_demo.wasm")]
    wasi: PathBuf,

    /// Save artifacts (JSON timing, logs)
    #[arg(long)]
    save: bool,
}

#[derive(Serialize, Debug)]
struct Timing {
    cold_compile_ms: f64,
    warm_compile_ms: f64,
    instantiate_ms: f64,
    run_ms: f64,
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn run_wat_add(engine: &Engine) -> Result<i32> {
    // Simple WAT module exporting (func (export "add") (param i32 i32) (result i32) ...)
    let wat_src = include_str!("../../modules/add.wat");
    let wasm = wat::parse_str(wat_src).context("Failed to parse WAT")?;
    let module = Module::new(engine, &wasm).context("Compile add.wat")?;
    let mut store = Store::new(engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[]).context("Instantiate add.wat")?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let res = add.call(&mut store, (7, 35)).context("Invoke add")?;
    Ok(res)
}

fn run_wasi(engine: &Engine, wasm_path: &PathBuf) -> Result<()> {
    // Prepare WASI context: inherit stdio, preopen demo/data
    let wasi = WasiCtx::builder()
        .inherit_stdio()
        .inherit_env()
        .preopened_dir("demo/data", "/data", DirPerms::all(), FilePerms::all())?
        .build_p1();

    let mut store = Store::new(engine, wasi);
    let module = Module::from_file(engine, wasm_path).context("Load WASI module")?;

    let mut linker = Linker::new(engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |ctx| ctx)?;
    let instance = linker.instantiate(&mut store, &module).context("Instantiate WASI module")?;

    // WASI modules conventionally export `_start`
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start.call(&mut store, ())?;
    Ok(())
}

fn run_host_import(engine: &Engine) -> Result<()> {
    // Demonstrates linking a host function usable from Wasm
    let wat_src = include_str!("../../modules/uses_import.wat");
    let wasm = wat::parse_str(wat_src)?;
    let module = Module::new(engine, &wasm)?;

    let mut store = Store::new(engine, ());
    let mut linker = Linker::new(engine);

    linker.func_wrap("env", "log", |mut caller: wasmtime::Caller<'_, ()>, ptr: i32, len: i32| {
        // Read memory and print to host stdout
        let mem = caller
            .get_export("memory")
            .and_then(|e| e.into_memory())
            .expect("memory export");
        let data = mem
            .data(&caller)
            .get(ptr as usize..(ptr + len) as usize)
            .expect("range");
        let s = std::str::from_utf8(data).expect("utf8");
        println!("[host log] {}", s);
    })?;

    let instance = linker.instantiate(&mut store, &module)?;
    // Call the exported function that uses the host import
    let run = instance.get_func(&mut store, "run").expect("export run");
    run.call(&mut store, &[], &mut [])?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    fs::create_dir_all("demo/out")?;

    // Engine setup (default; could enable caches or tunables here)
    let engine = Engine::new(&wasmtime::Config::new()).context("Create Engine")?;

    // Measure cold vs warm compilation on WAT module
    let t0 = Instant::now();
    let _cold = run_wat_add(&engine)?;
    let cold_compile = t0.elapsed();

    let t1 = Instant::now();
    let _warm = run_wat_add(&engine)?;
    let warm_compile = t1.elapsed();

    // Build & run WASI module
    let t2 = Instant::now();
    run_wasi(&engine, &args.wasi)?;
    let run_time = t2.elapsed();

    // Host import demo
    run_host_import(&engine)?;

    // Measure instantiate time separately
    let t3 = Instant::now();
    let module = Module::from_file(&engine, &args.wasi)?;
    let wasi = WasiCtx::builder()
        .inherit_stdio()
        .inherit_env()
        .preopened_dir("demo/data", "/data", DirPerms::all(), FilePerms::all())?
        .build_p1();
    let mut store = Store::new(&engine, wasi);
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |ctx| ctx)?;
    let _ = linker.instantiate(&mut store, &module)?;
    let instantiate_time = t3.elapsed();

    let timing = Timing {
        cold_compile_ms: ms(cold_compile),
        warm_compile_ms: ms(warm_compile),
        instantiate_ms: ms(instantiate_time),
        run_ms: ms(run_time),
    };

    println!("Timing: {:?}", timing);

    if args.save {
        let json_path = "demo/out/timing.json";
        let json = serde_json::to_string_pretty(&timing)?;
        fs::write(json_path, json)?;
        println!("[*] Saved {}", json_path);
    }

    Ok(())
}
