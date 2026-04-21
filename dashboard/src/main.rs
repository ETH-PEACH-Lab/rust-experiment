// dashboard/src/main.rs

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const HTML: &str = include_str!("../static/index.html");

// ── Test result types ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct TestResult {
    name:   String,
    passed: bool,
    suite:  String,
}

#[derive(Clone, Debug)]
struct TestRun {
    results:     Vec<TestResult>,
    running:     bool,
    last_run_ms: u64,
}

impl TestRun {
    fn new() -> Self { TestRun { results: vec![], running: false, last_run_ms: 0 } }

    fn to_json(&self) -> String {
        let items: Vec<String> = self.results.iter().map(|r| {
            format!(r#"{{"name":{},"passed":{},"suite":{}}}"#,
                json_str(&r.name), r.passed, json_str(&r.suite))
        }).collect();
        format!(r#"{{"running":{},"last_run_ms":{},"results":[{}]}}"#,
            self.running, self.last_run_ms, items.join(","))
    }
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO).as_millis() as u64
}

// ── Source-file reader — extracts values live without compilation ─────────────

/// Read `garden.rs` and extract the numeric value that follows `key:` on any line,
/// skipping lines where the value is a type annotation (i.e. no digit follows).
fn read_garden_value(workspace: &str, key: &str, fallback: f64) -> f64 {
    let path = format!("{}/sim/src/garden.rs", workspace);
    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return fallback,
    };
    // Scan every occurrence of key, return the first one whose value is numeric
    let mut search = src.as_str();
    while let Some(pos) = search.find(key) {
        let rest = search[pos + key.len()..].trim_start_matches(|c: char| c == ' ' || c == '\t');
        // Must start with a digit or '-' (not a type like `f64`)
        if rest.starts_with(|c: char| c.is_ascii_digit() || c == '-') {
            let end = rest.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                .unwrap_or(rest.len());
            if let Ok(v) = rest[..end].parse() { return v; }
        }
        search = &search[pos + key.len()..];
    }
    fallback
}

/// Returns true if the given code snippet exists anywhere in garden.rs.
fn garden_has_code(workspace: &str, snippet: &str) -> bool {
    let path = format!("{}/sim/src/garden.rs", workspace);
    std::fs::read_to_string(&path)
        .map(|src| src.contains(snippet))
        .unwrap_or(false)
}

fn garden_state_json(workspace: &str) -> String {
    let can_x      = read_garden_value(workspace, "can_x:",      240.0);
    let can_y      = read_garden_value(workspace, "can_y:",       20.0);
    let can_angle  = read_garden_value(workspace, "can_angle:",  -30.0);
    let c1d = read_garden_value(workspace, "c1_day_h:",   8.0);
    let c1k = read_garden_value(workspace, "c1_dark_h:", 10.0);
    let c1l = read_garden_value(workspace, "c1_lux:",     0.6);
    let c2d = read_garden_value(workspace, "c2_day_h:", 10.0);
    let c2k = read_garden_value(workspace, "c2_dark_h:", 12.0);
    let c2l = read_garden_value(workspace, "c2_lux:",     0.5);
    let c3d = read_garden_value(workspace, "c3_day_h:",   7.0);
    let c3k = read_garden_value(workspace, "c3_dark_h:", 11.0);
    let c3l = read_garden_value(workspace, "c3_lux:",     0.7);
    let c4d = read_garden_value(workspace, "c4_day_h:", 12.0);
    let c4k = read_garden_value(workspace, "c4_dark_h:",  9.0);
    let c4l = read_garden_value(workspace, "c4_lux:",     0.4);
    let c5d = read_garden_value(workspace, "c5_day_h:",   9.0);
    let c5k = read_garden_value(workspace, "c5_dark_h:", 13.0);
    let c5l = read_garden_value(workspace, "c5_lux:",     0.6);
    // Bug 3 is fixed when the Kelvin offset is removed from the temperature formula
    let temp_fixed = !garden_has_code(workspace, "273.15");
    format!(
        r#"{{"can_x":{},"can_y":{},"can_angle":{},"c1":[{},{},{}],"c2":[{},{},{}],"c3":[{},{},{}],"c4":[{},{},{}],"c5":[{},{},{}],"temp_fixed":{}}}"#,
        can_x, can_y, can_angle,
        c1d, c1k, c1l, c2d, c2k, c2l, c3d, c3k, c3l, c4d, c4k, c4l, c5d, c5k, c5l,
        temp_fixed
    )
}

// ── Test runner ───────────────────────────────────────────────────────────────

fn run_tests(state: Arc<Mutex<TestRun>>, workspace: String) {
    {
        let mut s = state.lock().unwrap();
        if s.running { return; }
        s.running = true;
    }
    thread::spawn(move || {
        let output = Command::new("cargo")
            .args(["test", "-p", "garden-sim", "--no-fail-fast"])
            .current_dir(&workspace)
            .output();

        let mut results: Vec<TestResult> = vec![];
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            let clean: String = {
                let mut s = String::with_capacity(text.len());
                let mut chars = text.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '\x1b' { for nc in chars.by_ref() { if nc == 'm' { break; } } }
                    else { s.push(c); }
                }
                s
            };
            for line in clean.lines() {
                let t = line.trim();
                if !t.starts_with("test ") { continue; }
                let after = &t[5..];
                let Some(sep) = after.find(" ... ") else { continue };
                let name    = &after[..sep];
                let outcome = &after[sep + 5..];
                if name.starts_with("result:") { continue; }
                let passed  = outcome.trim_start_matches(|c: char|
                    c == '\x1b' || c.is_ascii_digit() || c == '[' || c == ';') == "ok"
                    || outcome == "ok";
                let failed  = outcome.starts_with("FAILED") || outcome.contains("FAILED");
                if !passed && !failed { continue; }
                let suite = if name.contains("feature_") { "task2" } else { "task1" };
                results.push(TestResult { name: name.to_string(), passed, suite: suite.to_string() });
            }
        }
        let mut s = state.lock().unwrap();
        s.results     = results;
        s.running     = false;
        s.last_run_ms = now_ms();
    });
}

// ── Plant Lab runner ──────────────────────────────────────────────────────────

/// Parse static config flags from garden_lab.rs source — fast enough to poll.
fn run_lab_config(workspace: &str) -> String {
    let path = format!("{}/sim/src/systems/garden_lab.rs", workspace);
    let src = std::fs::read_to_string(&path).unwrap_or_default();

    // Look for `CAN_INTERACTIVE: bool = <true|false>`
    let interactive = src.find("CAN_INTERACTIVE")
        .and_then(|pos| {
            let rest = &src[pos..];
            rest.find('=').map(|eq| &rest[eq + 1..])
        })
        .map(|rest| {
            let trimmed = rest.trim_start();
            if trimmed.starts_with("true") { true }
            else if trimmed.starts_with("false") { false }
            else { true }
        })
        .unwrap_or(true);

    format!(r#"{{"interactive":{}}}"#, interactive)
}

/// Run `cargo run --example lab -- check <dist>` and return physics JSON.
fn run_lab_check(workspace: &str, dist: f64) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--example", "lab", "--", "check", &dist.to_string()])
        .current_dir(workspace)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if s.is_empty() { r#"{"tilt":0,"reaches":false}"#.to_string() } else { s }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let first = stderr.lines()
                .find(|l| l.contains("error"))
                .unwrap_or("compile error")
                .replace('"', "'");
            format!(r#"{{"tilt":0,"reaches":false,"error":"{}"}}"#, &first[..first.len().min(120)])
        }
        Err(e) => format!(r#"{{"tilt":0,"reaches":false,"error":"{}"}}"#, e),
    }
}

/// Run `cargo run --example lab -- <m> <f> <t>` and return the JSON output.
/// On compile/runtime error, returns a JSON object with color:null and an error field.
fn run_lab(workspace: &str, m: f64, f: f64, t: f64) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--example", "lab", "--", &m.to_string(), &f.to_string(), &t.to_string()])
        .current_dir(workspace)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.is_empty() { r#"{"color":null}"#.to_string() } else { stdout }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let first = stderr.lines()
                .find(|l| l.contains("error"))
                .unwrap_or("compile error")
                .replace('"', "'");
            format!(r#"{{"color":null,"error":"{}"}}"#, &first[..first.len().min(120)])
        }
        Err(e) => format!(r#"{{"color":null,"error":"{}"}}"#, e),
    }
}

// ── HTTP handler ──────────────────────────────────────────────────────────────

fn handle(mut stream: TcpStream, state: Arc<Mutex<TestRun>>, workspace: String) {
    let mut buf = [0u8; 1024];
    let n = match stream.read(&mut buf) { Ok(n) => n, Err(_) => return };
    let request    = String::from_utf8_lossy(&buf[..n]);
    let first_line = request.lines().next().unwrap_or("");

    if first_line.starts_with("GET /api/state") {
        // Reads garden.rs source directly — live, no recompile needed
        let body = garden_state_json(&workspace);
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else if first_line.starts_with("GET /api/results") {
        let body = state.lock().unwrap().to_json();
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else if first_line.starts_with("GET /api/lab/config") {
        let body = run_lab_config(&workspace);
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else if first_line.starts_with("GET /api/lab/check") {
        let dist = {
            let needle = "dist=";
            first_line.find(needle)
                .and_then(|pos| {
                    let rest = &first_line[pos + needle.len()..];
                    let end  = rest.find(|c: char| c == '&' || c == ' ').unwrap_or(rest.len());
                    rest[..end].parse().ok()
                })
                .unwrap_or(100.0_f64)
        };
        let body = run_lab_check(&workspace, dist);
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else if first_line.starts_with("GET /api/lab") {
        // Parse ?m=..&f=..&t=.. from the request line
        let parse_param = |key: &str, default: f64| -> f64 {
            let needle = format!("{}=", key);
            first_line.find(&needle)
                .and_then(|pos| {
                    let rest = &first_line[pos + needle.len()..];
                    let end  = rest.find(|c: char| c == '&' || c == ' ').unwrap_or(rest.len());
                    rest[..end].parse().ok()
                })
                .unwrap_or(default)
        };
        let m = parse_param("m", 0.5);
        let f = parse_param("f", 1.0);
        let t = parse_param("t", 22.0);
        let body = run_lab(&workspace, m, f, t);
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else if first_line.starts_with("POST /api/run") {
        run_tests(Arc::clone(&state), workspace);
        let body = r#"{"status":"started"}"#;
        let resp = format!(
            "HTTP/1.1 202 Accepted\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body);
        let _ = stream.write_all(resp.as_bytes());

    } else {
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            HTML.len(), HTML);
        let _ = stream.write_all(resp.as_bytes());
    }
}

// ── HTTP server ───────────────────────────────────────────────────────────────

fn start_server(state: Arc<Mutex<TestRun>>, workspace: String) {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("Could not bind to port 3030");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
            let state     = Arc::clone(&state);
            let workspace = workspace.clone();
            thread::spawn(move || handle(stream, state, workspace));
        }
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    use tao::dpi::LogicalSize;
    use tao::event::{Event, WindowEvent};
    use tao::event_loop::{ControlFlow, EventLoop};
    use tao::window::WindowBuilder;
    use wry::WebViewBuilder;

    let workspace = std::env::current_dir().unwrap_or_default()
        .to_string_lossy().to_string();

    let state: Arc<Mutex<TestRun>> = Arc::new(Mutex::new(TestRun::new()));

    let server_state     = Arc::clone(&state);
    let server_workspace = workspace.clone();
    thread::spawn(move || start_server(server_state, server_workspace));
    run_tests(Arc::clone(&state), workspace.clone());

    thread::sleep(Duration::from_millis(150));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wholesome Garden")
        .with_inner_size(LogicalSize::new(300_u32, 920_u32))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new()
        .with_url("http://localhost:3030")
        .build(&window)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
