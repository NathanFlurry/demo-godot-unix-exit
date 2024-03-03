use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use godot::engine::{INode, Node};
use godot::prelude::*;

struct UnixExit;

#[gdextension]
unsafe impl ExtensionLibrary for UnixExit {}

#[derive(GodotClass)]
#[class(base=Node)]
struct UnixExitNode {
    base: Base<Node>,
    has_exited: Arc<AtomicBool>,
}

#[godot_api]
impl INode for UnixExitNode {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        let has_exited = Arc::new(AtomicBool::new(false));

        let has_exited_clone = has_exited.clone();
        ctrlc::set_handler(move || {
            has_exited_clone.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");

        Self { base, has_exited }
    }

    fn process(&mut self, _delta: f64) {
        if self.has_exited.load(Ordering::Relaxed) {
            godot_print!("Exiting...");
            self.base().get_tree().as_mut().expect("no tree").quit();
        }
    }
}
