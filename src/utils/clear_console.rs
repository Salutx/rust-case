/**
 * Function to clear the console screen.
 */
pub fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
