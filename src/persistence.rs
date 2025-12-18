use std::process::Command;
use std::env;
use std::io;

pub fn install_sentinel() -> io::Result<()> {
    let current_exe = env::current_exe()?;
    let exe_path = current_exe.to_str().unwrap();
    let command_args = format!("\"{}\" lock", exe_path);

    let output = Command::new("schtasks")
        .args(&[
            "/CREATE",
            "/F",
            "/TN", "AmnesiaSentinel",
            "/SC", "ONLOGON",
            "/RL", "HIGHEST",
            "/TR", &command_args
        ])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        // Si falla, convierte el error de Windows a string para verlo
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err_msg))
    }
}

pub fn uninstall_sentinel() -> io::Result<()> {
    let output = Command::new("schtasks")
        .args(&["/DELETE", "/F", "/TN", "AmnesiaSentinel"])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err_msg))
    }
}