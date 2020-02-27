#[cfg(any(target_arch = "armv7", target_arch = "arm"))]
extern crate blinkt;

extern crate ctrlc;
extern crate notify;
extern crate wasmi;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use wasm::Runtime;
use wasmi::RuntimeValue;

const MODULE_FILE: &'static str = "/home/kevin/indicators/indicator.wasm";
const MODULE_DIR: &'static str = "/home/kevin/indicators";

enum RunnerCommand {
    Reload,
    Stop,
}

fn watch(tx_wasm: Sender<RunnerCommand>) -> notify::Result<()> {
    let (tx, rx) = channel(); // (1) Creates a multi-producer, single-consumer communication channel

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(MODULE_DIR, RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv() { // (2) Block the receive channel until a message arrives
            Ok(event) => handle_event(event, &tx_wasm),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_event(event: DebouncedEvent, tx_wasm: &Sender<RunnerCommand>) {
    match event {
        DebouncedEvent::NoticeWrite(path) => {
            let path = Path::new(&path);
            let filename = path.file_name().unwrap();
            if filename == "indicator.wasm" {
                tx_wasm.send(RunnerCommand::Reload).unwrap(); // (3) Send a message on the channel, indicating that we should reload the WebAssembly module
            } else {
                println!("write (unexpected file): {:?}", path);
            }
        }
        _ => {}
    }
}

// 20 frames per second == ms delay of 50 per iteration
// 10 fps == 100ms delay

fn main() {
    let (tx_wasm, rx_wasm) = channel();
    let _indicator_runner = thread::spawn(move || {
        let mut runtime = Runtime::new();
        let mut module = wasm::get_module_instance(MODULE_FILE);
        println!("Starting wasm runner thread...");
        loop {
            match rx_wasm.recv_timeout(Duration::from_millis(100)) { // (4) Enforce the frame rate with a 100ms timeout value on receive
                Ok(RunnerCommand::Reload) => {
                    println!("Received a reload signal, sleeping for 2s");
                    thread::sleep(Duration::from_secs(2));
                    module = wasm::get_module_instance(MODULE_FILE);
                }
                Ok(RunnerCommand::Stop) => {
                    runtime.shutdown();
                    break;
                }
                Err(RecvTimeoutError::Timeout) => {
                    runtime.reduce_battery();
                    runtime.advance_frame();
                    module
                        .invoke_export(
                            "sensor_update",
                            &[
                                RuntimeValue::from(wasm::SENSOR_BATTERY),
                                RuntimeValue::F64(
                                 runtime.remaining_battery.into()),
                            ][..],
                            &mut runtime,
                        ).unwrap();

                    module
                        .invoke_export(
                            "apply",
                            &[RuntimeValue::from(runtime.frame)][..],
                            &mut runtime,
                        ).unwrap();
                }
                Err(_) => break,
            }
        }
    });

    let tx_wasm_sig = tx_wasm.clone();  // (5) Send channels can be cloned, hence their presence in the multi-producer module

    ctrlc::set_handler(move || { // (6) Use the ctrlc crate to trap SIGTERM and SIGINT, sending a Stop command in response
        tx_wasm_sig.send(RunnerCommand::Stop).unwrap();
    }).expect("Error setting Ctrl-C handler");

    if let Err(e) = watch(tx_wasm) { // (7) The watch function blocks the main thread with an infinite loop
        println!("error: {:?}", e)
    }
}

mod wasm;