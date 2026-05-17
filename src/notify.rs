pub fn alert(body: &str) {
    let _ = notify_rust::Notification::new()
        .appname("Deadlock RPC")
        .summary("Deadlock RPC")
        .body(body)
        .show();
}
