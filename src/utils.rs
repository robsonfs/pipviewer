pub fn silent_behave(silent: bool, total_bytes: usize, new_line: bool) {
    if !silent {
        if new_line {
            eprintln!("\r{}", total_bytes);
        } else {
            eprint!("\r{}", total_bytes);
        }
    }
}
