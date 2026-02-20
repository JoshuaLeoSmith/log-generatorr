use std::fs::{self, File, rename};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};

use chrono::Utc;

/// A writer that automatically rotates log files when they exceed a size limit.
pub struct RotatingWriter {
    dir: PathBuf,
    max_bytes: u64,
    current_bytes: u64,
    writer: BufWriter<File>,
    current_path: PathBuf,
    file_index: u32,
}

impl RotatingWriter {
    /// Create a new RotatingWriter that writes into `dir` and rotates at `max_bytes`.
    pub fn new(dir: &Path, max_bytes: u64) -> io::Result<Self> {
        fs::create_dir_all(dir)?;

        let file_name = format!("{}.log", Utc::now().format("%Y-%m-%d_%H-%M-%S"));
        let path = dir.join(&file_name);
        let file = File::create(&path)?;

        Ok(Self {
            dir: dir.to_path_buf(),
            max_bytes,
            current_bytes: 0,
            writer: BufWriter::with_capacity(64 * 1024, file),
            current_path: path,
            file_index: 0,
        })
    }

    /// Write a log line. Rotates the file if the size limit is exceeded.
    /// Returns the number of bytes written.
    pub fn write_line(&mut self, line: &str) -> io::Result<usize> {
        let bytes = line.as_bytes();
        let newline = b"\n";
        let total = bytes.len() + newline.len();

        // Check if we need to rotate before writing
        if self.current_bytes + total as u64 > self.max_bytes && self.current_bytes > 0 {
            self.rotate()?;
        }

        self.writer.write_all(bytes)?;
        self.writer.write_all(newline)?;
        self.current_bytes += total as u64;

        // Flush periodically (every ~256KB) to avoid losing too much on crash
        if self.current_bytes % (256 * 1024) < total as u64 {
            self.writer.flush()?;
        }

        Ok(total)
    }

    fn rotate(&mut self) -> io::Result<()> {
        // Flush and drop the current writer
        self.writer.flush()?;

        // Rename current file to archived name
        self.file_index += 1;
        let archived_name = format!(
            "{}_{:04}.log",
            Utc::now().format("%Y-%m-%d_%H-%M-%S"),
            self.file_index
        );
        let archived_path = self.dir.join(&archived_name);
        rename(&self.current_path, &archived_path)?;

        // Open new file
        let new_name = format!("{}.log", Utc::now().format("%Y-%m-%d_%H-%M-%S%.f"));
        let new_path = self.dir.join(&new_name);
        let file = File::create(&new_path)?;
        self.writer = BufWriter::with_capacity(64 * 1024, file);
        self.current_path = new_path;
        self.current_bytes = 0;

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

