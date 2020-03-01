use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn silent_behave(silent: bool, total_bytes: usize, new_line: bool) {
    if !silent {
        if new_line {
            eprintln!("\r{}", total_bytes);
        } else {
            eprint!("\r{}", total_bytes);
        }
    }
}

fn main() -> Result<()> {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        silent_behave(silent, total_bytes, false);
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    silent_behave(silent, total_bytes, true);
    Ok(())
}
