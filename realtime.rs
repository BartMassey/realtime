use howlong::HighResolutionTimer;

fn main() {
    let argv: Vec<std::ffi::OsString> = std::env::args_os().collect();
    let exe = std::path::PathBuf::from(&argv[1]);
    let mut cmd = std::process::Command::new(&exe);
    let cmd = cmd.args(&argv[2..]);

    let timer = HighResolutionTimer::new();

    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("{}: {e}", exe.display());
        std::process::exit(1);
    });

    let time = timer.elapsed().as_secs_f64();

    if !status.success() {
        eprintln!("{}: failed exit status", exe.display());
    }

    eprintln!();
    eprintln!("time: {time}");
}
