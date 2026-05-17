#[cfg(windows)]
pub fn alert(body: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let body_owned = body.to_string();
    std::thread::spawn(move || {
        let title: Vec<u16> = OsStr::new("Deadlock RPC")
            .encode_wide()
            .chain(Some(0))
            .collect();
        let text: Vec<u16> = OsStr::new(&body_owned)
            .encode_wide()
            .chain(Some(0))
            .collect();
        unsafe {
            winapi::um::winuser::MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                winapi::um::winuser::MB_OK
                    | winapi::um::winuser::MB_ICONINFORMATION
                    | winapi::um::winuser::MB_TOPMOST,
            );
        }
    });
}

#[cfg(not(windows))]
pub fn alert(body: &str) {
    let _ = notify_rust::Notification::new()
        .appname("Deadlock RPC")
        .summary("Deadlock RPC")
        .body(body)
        .show();
}
