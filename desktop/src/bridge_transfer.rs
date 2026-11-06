//! Preserve the actual PU FirmwareTransfer GET/HEAD/Range/ETag contract and
//! Pakar's existing guarded updater/AVR coordinator, rather than a second flasher.
use super::*;

pub(super) const MAX_IMAGE_BODY: usize = 4 * 1024 * 1024 + 65536;
const MAX_EXPORT: u64 = 4 * 1024 * 1024;
const RESPONSE_HEADERS: &[&str] = &[
    "content-type",
    "content-range",
    "accept-ranges",
    "etag",
    "content-md5",
    "digest",
    "content-disposition",
    "x-firmware-mode",
    "x-firmware-size",
    "x-firmware-md5",
    "x-product-id",
    "x-product-model",
    "location",
];

fn feature(request: &Request) -> Option<&'static str> {
    let path = request.path.split('?').next()?;
    match (request.method.as_str(), path) {
        ("GET","/api/peers/discovered")|("POST","/api/peers/discover"|"/api/peers/policy"|"/api/peers/update")=>Some("peer-exchange"),
        ("GET" | "HEAD", "/firmware/download") | ("POST", "/update") => Some("ota"),
        ("GET" | "HEAD", "/file/download") => Some("artifacts"),
        ("GET", "/api/firmware/remote")
        | ("POST", "/api/firmware/fetch" | "/api/firmware/apply" | "/api/firmware/cancel") => {
            Some("ota")
        }
        ("GET", "/api/avr/probe" | "/api/avr/backup/metadata")
        | ("GET" | "HEAD", "/api/avr/backup/flash" | "/api/avr/backup/eeprom")
        | ("POST", "/api/avr/backup" | "/api/avr/image" | "/api/avr/program") => Some("isp"),
        ("GET" | "POST", "/api/avr/contracts")
        | ("GET" | "DELETE", "/api/avr/contracts/result") => Some("settings"),
        ("GET", "/info" | "/api/telemetry") | ("POST", "/api/clients/report") => Some("telemetry"),
        ("GET" | "POST", "/api/cloud") | ("POST", "/api/cloud/reconnect") => Some("network"),
        _ => None,
    }
}
pub(super) fn handles(request: &Request) -> bool {
    feature(request).is_some()
}

pub(super) struct Response<'a> {
    response: reqwest::blocking::Response,
    shared: &'a Shared,
    stamp: connection::Stamp,
    _lease: connection::Lease,
    head: bool,
    maximum: u64,
}
pub(super) fn open<'a>(shared: &'a Shared, request: &Request) -> Result<Response<'a>, String> {
    let o = route(shared)?;
    let requested = feature(request).ok_or("Transfer API unavailable")?;
    let path = request.path.split('?').next().unwrap_or("");
    if request.path.len() > 2048
        || request.path.contains('#')
        || request.path.contains('%')
        || !request.path.is_ascii()
        || request.path.bytes().any(|b| b < 32 || b == 127)
    {
        // Query values can be URL encoded; path separators cannot be disguised.
        if path.contains('%')
            || request.path.len() > 2048
            || request.path.contains('#')
            || !request.path.is_ascii()
            || request.path.bytes().any(|b| b < 32 || b == 127)
        {
            return Err("Invalid transfer target".into());
        }
    }
    let has = |feature: &str| {
        o.status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.has(feature))
    };
    let allowed = if requested == "artifacts" {
        has("ota") || has("isp")
    } else {
        has(requested)
    };
    if !allowed {
        return Err("Selected board has not acknowledged this capability".into());
    }
    // Probe is a resetting ISP operation even though its existing route is GET.
    let mutation = !matches!(request.method.as_str(), "GET" | "HEAD") || path == "/api/avr/probe";
    if mutation {
        require_pin(&o, &request.headers)?;
    }
    let Route::Network {
        endpoint,
        stamp,
        gate,
    } = &o.route
    else {
        return Err("This management transfer requires the selected network route; USB never opens a Wi-Fi fallback".into());
    };
    let lease = gate
        .acquire(Transport::WiFi, stamp.epoch)
        .ok_or("Selected network owner retired")?;
    let client = peer_http::Pin::from_status(&o.status)
        .ok_or("Peer fencing unavailable")?
        .client()?;
    let method =
        reqwest::Method::from_bytes(request.method.as_bytes()).map_err(|_| "Invalid method")?;
    let mut upstream = client
        .request(
            method,
            format!("{}{}", endpoint.base.trim_end_matches('/'), request.path),
        )
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .timeout(Duration::from_secs(
            if mutation || requested == "artifacts" || path == "/firmware/download" {
                180
            } else {
                8
            },
        ))
        .header(
            "X-Pakar-Route",
            trace(
                request.headers.get("x-pakar-route").map(String::as_str),
                &shared.settings.instance_id,
            )?,
        );
    for key in [
        "content-type",
        "range",
        "if-none-match",
        "if-match",
        "x-pakar-firmware-size",
        "x-pakar-firmware-md5",
    ] {
        if let Some(value) = request.headers.get(key) {
            upstream = upstream.header(key, value);
        }
    }
    if !request.body.is_empty() {
        upstream = upstream.body(request.body.clone());
    }
    if shared.stop.load(Ordering::Acquire) || !stamp.current() {
        return Err("Owner retired before transfer; nothing sent".into());
    }
    let response = upstream
        .send()
        .map_err(|_| "Transfer outcome uncertain; not replayed")?;
    if shared.stop.load(Ordering::Acquire) || !stamp.current() {
        return Err("Owner retired after dispatch; no replay".into());
    }
    // Preserve actual status/body:202 queued is not a verified flash, and201
    // upload staging is not programming. AVR coordinator keeps its own receipts.
    Ok(Response {
        response,
        shared,
        stamp: stamp.clone(),
        _lease: lease,
        head: request.method == "HEAD",
        maximum: if matches!(
            path,
            "/file/download"
                | "/firmware/download"
                | "/api/avr/backup/flash"
                | "/api/avr/backup/eeprom"
        ) {
            MAX_EXPORT
        } else {
            65536
        },
    })
}
impl Response<'_> {
    pub(super) fn write(mut self, socket: &mut TcpStream) -> Result<(), String> {
        let status = self.response.status().as_u16();
        let length = self
            .response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());
        if length.is_some_and(|n| n > self.maximum) {
            return Err("Upstream export exceeds bounded limit".into());
        }
        let no_body = self.head || status == 304 || status == 204;
        let mut buffered = Vec::new();
        if length.is_none() && !no_body {
            if self.maximum == MAX_EXPORT {
                return Err("Export requires exact Content-Length".into());
            }
            self.response
                .by_ref()
                .take(self.maximum + 1)
                .read_to_end(&mut buffered)
                .map_err(|_| "Incomplete upstream response; no replay")?;
            if buffered.len() as u64 > self.maximum {
                return Err("Upstream response exceeds limit".into());
            }
        }
        if self.shared.stop.load(Ordering::Acquire) || !self.stamp.current() {
            return Err("Transfer owner retired".into());
        }
        socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| e.to_string())?;
        write!(socket,"HTTP/1.1 {status} Result\r\n{}Content-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n",crate::peer_identity::server_headers(),length.unwrap_or(buffered.len() as u64)).map_err(|e|e.to_string())?;
        for key in RESPONSE_HEADERS {
            if let Some(value) = self
                .response
                .headers()
                .get(*key)
                .and_then(|v| v.to_str().ok())
            {
                // A redirect remains relative to this server, never a new host.
                if *key == "location" && (!value.starts_with('/') || value.starts_with("//")) {
                    continue;
                }
                write!(socket, "{key}: {value}\r\n").map_err(|e| e.to_string())?;
            }
        }
        socket.write_all(b"\r\n").map_err(|e| e.to_string())?;
        if no_body {
            return Ok(());
        }
        if length.is_none() {
            return socket.write_all(&buffered).map_err(|e| e.to_string());
        }
        let mut remaining = length.unwrap();
        let mut chunk = [0; 8192];
        while remaining > 0 {
            if self.shared.stop.load(Ordering::Acquire) || !self.stamp.current() {
                return Err("Transfer owner retired; partial response closed".into());
            }
            let count = self
                .response
                .read(&mut chunk[..(remaining as usize).min(8192)])
                .map_err(|_| "Upstream transfer interrupted; not replayed")?;
            if count == 0 {
                return Err("Truncated upstream transfer; no successful receipt".into());
            }
            socket
                .write_all(&chunk[..count])
                .map_err(|e| e.to_string())?;
            remaining -= count as u64;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(method: &str, path: &str) -> Request {
        Request {
            method: method.into(),
            path: path.into(),
            headers: BTreeMap::new(),
            body: Vec::new(),
        }
    }
    #[test]
    fn bridge_transfer_usb_never_opens_implicit_network_management() {
        let (shared, _, _) = super::super::tests::fixture();
        assert!(open(&shared, &request("GET", "/api/telemetry")).is_err());
        assert!(open(&shared, &request("GET", "/api/avr/probe")).is_err());
        assert!(!handles(&request("POST", "/arbitrary")));
        assert!(!handles(&request("POST", "/api/avr/erase-all")));
        assert!(handles(&request(
            "DELETE",
            "/api/avr/contracts/result?id=1"
        )));
    }
    #[test]
    fn bridge_transfer_real_binary_range_and_multipart_are_not_json_reparsed() {
        for (method, path, body, code, response, content_type) in [
            (
                "GET",
                "/firmware/download",
                vec![],
                206,
                vec![0xe9, 0, 0xff, 0x80],
                "application/octet-stream",
            ),
            (
                "POST",
                "/api/avr/image",
                [b"--test\r\nContent-Disposition: form-data; name=\"firmware\"; filename=\"avr.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n".as_slice(), &vec![0xff; 32768], b"\r\n--test--\r\n".as_slice()].concat(),
                201,
                b"AVR image stored".to_vec(),
                "text/plain",
            ),
            (
                "POST",
                "/api/avr/contracts",
                b"opcode=53&payload_hex=00".to_vec(),
                202,
                b"{\"state\":\"queued\",\"request_id\":5}".to_vec(),
                "application/json",
            ),
            (
                "GET",
                "/api/telemetry",
                vec![],
                200,
                b"{\"esp\":{\"flash_bytes\":16777216},\"avr\":{},\"clients\":[]}".to_vec(),
                "application/json",
            ),
        ] {
            let (shared, _, _) = super::super::tests::fixture();
            let upstream = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = upstream.local_addr().unwrap();
            let reply = response.clone();
            let expected = body.clone();
            let remote = thread::spawn(move || {
                let mut s = upstream.accept().unwrap().0;
                // Request parser has a larger limit only for known uploads.
                let r = read_request(&mut s).unwrap();
                assert_eq!(r.body, expected);
                assert_eq!(r.headers.get("x-pakar-board").unwrap(), "fixture-board");
                if method == "GET" && path == "/firmware/download" {
                    assert_eq!(r.headers.get("range").unwrap(), "bytes=0-3");
                }
                write!(s,"HTTP/1.1 {code} Result\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nETag: \"actual\"\r\nContent-Range: bytes 0-3/32768\r\nConnection: close\r\n\r\n",reply.len()).unwrap();
                s.write_all(&reply).unwrap();
            });
            let gate = connection::Gate::default();
            let epoch = gate.select(Some(Transport::WiFi));
            {
                let mut o = shared.observation.lock().unwrap();
                let o = o.as_mut().unwrap();
                o.route = Route::Network {
                    endpoint: Endpoint {
                        base: format!("http://{addr}"),
                        user: String::new(),
                        password: String::new(),
                    },
                    stamp: gate.stamp(Transport::WiFi, epoch),
                    gate: gate.clone(),
                };
                o.status
                    .capabilities
                    .as_mut()
                    .unwrap()
                    .acknowledged_features
                    .extend(["isp".into(), "settings".into(), "telemetry".into()]);
            }
            let mut r = request(method, path);
            r.body = body;
            r.headers.insert(
                "content-type".into(),
                if path=="/api/avr/image" {"multipart/form-data; boundary=test"}else{"application/x-www-form-urlencoded"}.into(),
            );
            r.headers.insert("range".into(), "bytes=0-3".into());
            r.headers
                .insert("x-pakar-board".into(), "fixture-board".into());
            r.headers.insert("x-pakar-boot".into(), "a".repeat(32));
            let opened = open(&shared, &r).unwrap();
            let output = TcpListener::bind("127.0.0.1:0").unwrap();
            let peer = output.local_addr().unwrap();
            let consumer = thread::spawn(move || {
                let mut s = TcpStream::connect(peer).unwrap();
                let mut bytes = Vec::new();
                s.read_to_end(&mut bytes).unwrap();
                bytes
            });
            opened.write(&mut output.accept().unwrap().0).unwrap();
            let bytes = consumer.join().unwrap();
            let split = bytes.windows(4).position(|v| v == b"\r\n\r\n").unwrap() + 4;
            assert_eq!(&bytes[split..], response);
            assert!(String::from_utf8_lossy(&bytes[..split]).contains(&format!("HTTP/1.1 {code}")));
            remote.join().unwrap();
        }
    }

    #[test]
    fn bridge_transfer_head_truncation_and_stale_mutation_preserve_truth() {
        for truncated in [false, true] {
            let (shared, _, _) = super::super::tests::fixture();
            let upstream = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = upstream.local_addr().unwrap();
            let gate = connection::Gate::default();
            let epoch = gate.select(Some(Transport::WiFi));
            {
                let mut o = shared.observation.lock().unwrap();
                o.as_mut().unwrap().route = Route::Network {
                    endpoint: Endpoint {
                        base: format!("http://{address}"),
                        user: String::new(),
                        password: String::new(),
                    },
                    stamp: gate.stamp(Transport::WiFi, epoch),
                    gate: gate.clone(),
                };
            }
            // A stale boot must fail before a single upstream byte is sent.
            let mut stale = request("POST", "/update");
            stale
                .headers
                .insert("x-pakar-board".into(), "fixture-board".into());
            stale.headers.insert("x-pakar-boot".into(), "b".repeat(32));
            assert!(open(&shared, &stale).is_err());
            upstream.set_nonblocking(true).unwrap();
            assert_eq!(
                upstream.accept().unwrap_err().kind(),
                std::io::ErrorKind::WouldBlock
            );
            upstream.set_nonblocking(false).unwrap();
            let remote = thread::spawn(move || {
                let mut socket = upstream.accept().unwrap().0;
                let r = read_request(&mut socket).unwrap();
                assert_eq!(r.method, if truncated { "GET" } else { "HEAD" });
                socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nContent-Type: application/octet-stream\r\nETag: \"exact\"\r\nConnection: close\r\n\r\n").unwrap();
                if truncated {
                    socket.write_all(&[0xe9, 0]).unwrap();
                }
            });
            let opened = open(
                &shared,
                &request(if truncated { "GET" } else { "HEAD" }, "/firmware/download"),
            )
            .unwrap();
            let output = TcpListener::bind("127.0.0.1:0").unwrap();
            let target = output.local_addr().unwrap();
            let consumer = thread::spawn(move || {
                let mut socket = TcpStream::connect(target).unwrap();
                let mut bytes = Vec::new();
                socket.read_to_end(&mut bytes).unwrap();
                bytes
            });
            let result = opened.write(&mut output.accept().unwrap().0);
            assert_eq!(result.is_err(), truncated);
            let bytes = consumer.join().unwrap();
            let split = bytes.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
            let headers = String::from_utf8_lossy(&bytes[..split]);
            assert!(headers.contains("Content-Length: 4") && headers.contains("etag: \"exact\""));
            assert_eq!(bytes.len() - split, if truncated { 2 } else { 0 });
            remote.join().unwrap();
        }
    }
}
