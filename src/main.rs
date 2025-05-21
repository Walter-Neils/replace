use anyhow::Result;
use std::{
    collections::HashMap,
    env::args,
    io::{Read, Write},
};

struct OutputManager {
    buffer: Vec<u8>,
    max_buffer_size: usize,
    output_files: Vec<String>,
    output_writers: HashMap<String, std::fs::File>,
}

impl OutputManager {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_buffer_size,
            output_files: Vec::new(),
            output_writers: HashMap::new(),
        }
    }

    pub fn add_files(&mut self, files: Vec<String>) {
        for file in files {
            self.output_files.push(file);
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        if self.buffer.len() >= self.max_buffer_size {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        for file in &self.output_files {
            if let Some(writer) = self.output_writers.get_mut(file) {
                writer
                    .write_all(&self.buffer)
                    .expect("Failed to write data");
            } else {
                let mut writer = std::fs::File::create(file).expect("Failed to create file");
                writer
                    .write_all(&self.buffer)
                    .expect("Failed to write data");
                self.output_writers.insert(file.clone(), writer);
            }
        }
        self.buffer.clear();
    }
}

fn main() -> Result<()> {
    let target_files: Vec<String> = args().skip(1).collect();
    if target_files.is_empty() {
        eprintln!("Replace: No target files provided.");
        std::process::exit(1);
    }

    let mut output_manager = OutputManager::new(1024 * 1024 * 256); // 256 MB buffer size
    output_manager.add_files(target_files);

    let mut stdin = std::io::stdin();
    let mut local_buffer = [0; 1024];
    while let Ok(read_result) = stdin.read(&mut local_buffer) {
        if read_result == 0 {
            // EOF reached
            break;
        } else {
            output_manager.add_data(&local_buffer[..read_result]);
        }
    }

    output_manager.flush();

    Ok(())
}
