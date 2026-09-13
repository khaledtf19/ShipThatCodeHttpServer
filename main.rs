use std::io::{self, BufRead};

fn is_method(s: &str) -> bool {
    matches!(
        s,
        "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH"
    )
}

fn is_version(s: &str) -> bool {
    // TODO: return true iff s == "HTTP/<digit>.<digit>"
    let Some((http, num)) = s.split_once('/') else {
        return false;
    };

    let Some((a, b)) = num.split_once('.') else {
        return false;
    };
    let are_digits = a.parse::<i32>().is_ok() && b.parse::<i32>().is_ok();

    if http == "HTTP" && are_digits {
        return true;
    };
    false
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        // TODO: must be 3 parts; validate method, path, version
        if parts.len() != 3
            || !is_method(parts[0])
            || !parts[1].starts_with('/')
            || !is_version(parts[2])
        {
            println!("INVALID");
            continue;
        }
        println!("METHOD={} PATH={} VERSION={}", parts[0], parts[1], parts[2]);
    }
}
