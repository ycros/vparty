#[macro_use]
extern crate vst2;
extern crate glutin;
extern crate conrod;
extern crate libc;
extern crate gl;

use std::mem;
use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use libc::c_void;

use vst2::plugin::{Info, Plugin};
use vst2::editor::Editor;
use glutin::{WindowBuilder, WindowHandle};

#[derive(Default)]
struct BasicPlugin {
    editor: MyEditor,
}

struct MyEditor {
    is_open: bool,
    log: File,
}

impl Default for MyEditor {
    fn default() -> Self {
        let path = Path::new("C:\\tmp\\vparty.log");

        let mut file = File::create(&path).unwrap();

        MyEditor {
            is_open: false,
            log: file,
        }
    }
}

impl Editor for MyEditor {
    fn size(&self) -> (i32, i32) {
        (500, 500)
    }

    fn position(&self) -> (i32, i32) {
        (500, 500)
    }

    fn open(&mut self, window: *mut c_void) {
        self.log.write_all("editor: open\n".as_bytes());
        self.log.flush();
        let mut log = self.log.try_clone().unwrap();

        let handle = unsafe {
            WindowHandle(mem::transmute(window))
        };

        // TODO: need to kill this from close?
        thread::spawn(move || {
            log.write_all("editor: thread start".as_bytes());
            log.flush();

            let builder = WindowBuilder::new().with_parent(handle);
            let window = builder.build().unwrap();

            unsafe { window.make_current(); };

            unsafe {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
                gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            }

            for event in window.wait_events() {
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
                window.swap_buffers();

                match event {
                    glutin::Event::Closed => break,
                    _ => ()
                }
            }

            log.write_all("editor: thread stop".as_bytes());
            log.flush();
        });

        self.is_open = true;
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }
}

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(&mut self.editor)
    }
}

plugin_main!(BasicPlugin); // Important!