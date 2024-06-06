use std::env::consts::OS;
use std::process::Command;

pub fn flush_macos_dns_cache() {
    Command::new("dscacheutil")
        .arg("-flushcache")
        .output()
        .unwrap();
}

pub fn flush_windows_dns_cache() {
    Command::new("ipconfig").arg("/flushdns").output().unwrap();
}

pub fn hup_dns_responder() {
    Command::new("killall")
        .arg("-HUP")
        .arg("mDNSResponder")
        .output()
        .unwrap();
}

pub fn flush_dns_cache() {
    if OS == "windows" {
        flush_windows_dns_cache();
        return;
    }

    if OS == "macos" {
        flush_macos_dns_cache();
        hup_dns_responder();
        return;
    }
}
