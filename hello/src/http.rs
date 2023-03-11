use std::{collections::HashMap, fmt::Display};

#[derive(PartialEq)]
pub struct RequestHeader {
    pub method: Method,
    pub path: String,
    pub version: Version,
}

impl TryFrom<&str> for RequestHeader {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut components = string.split_whitespace();

        let method = Method::try_from(components.next().ok_or("No method found.")?)?;
        let path = components.next().ok_or("No path found.")?.to_string();
        let version = Version::try_from(components.next().ok_or("No version found.")?)?;

        Ok(Self {
            method,
            path,
            version,
        })
    }
}

#[derive(PartialEq)]
pub enum Method {
    Get,
}

impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "GET" => Ok(Self::Get),
            _ => Err("Method not implemented."),
        }
    }
}

#[derive(PartialEq)]
pub enum Version {
    H1,
    H2,
}

impl TryFrom<&str> for Version {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "HTTP/1.1" => Ok(Self::H1),
            "HTTP/2.0" => Ok(Self::H2),
            _ => Err("HTTP version not implemented."),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/{}",
            match self {
                Self::H1 => "1.1",
                Self::H2 => "2.0",
            }
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct StatusCode(pub u16);

#[allow(dead_code)]
impl StatusCode {
    pub const CONTINUE: StatusCode = StatusCode(100);
    pub const SWITCHINGPROTOCOLS: StatusCode = StatusCode(101);
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const ACCEPTED: StatusCode = StatusCode(202);
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode = StatusCode(203);
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    pub const RESETCONTENT: StatusCode = StatusCode(205);
    pub const PARTIALCONTENT: StatusCode = StatusCode(206);
    pub const MULTIPLECHOICES: StatusCode = StatusCode(300);
    pub const MOVEDPERMANENTLY: StatusCode = StatusCode(301);
    pub const FOUND: StatusCode = StatusCode(302);
    pub const SEEOTHER: StatusCode = StatusCode(303);
    pub const NOTMODIFIED: StatusCode = StatusCode(304);
    pub const USEPROXY: StatusCode = StatusCode(305);
    pub const TEMPORARYREDIRECT: StatusCode = StatusCode(307);
    pub const BADREQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const PAYMENTREQUIRED: StatusCode = StatusCode(402);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOTFOUND: StatusCode = StatusCode(404);
    pub const METHODNOTALLOWED: StatusCode = StatusCode(405);
    pub const NOTACCEPTABLE: StatusCode = StatusCode(406);
    pub const PROXYAUTHENTICATIONREQUIRED: StatusCode = StatusCode(407);
    pub const REQUESTTIMEOUT: StatusCode = StatusCode(408);
    pub const CONFLICT: StatusCode = StatusCode(409);
    pub const GONE: StatusCode = StatusCode(410);
    pub const LENGTHREQUIRED: StatusCode = StatusCode(411);
    pub const PRECONDITIONFAILED: StatusCode = StatusCode(412);
    pub const REQUESTENTITYTOOLARGE: StatusCode = StatusCode(413);
    pub const REQUESTURITOOLARGE: StatusCode = StatusCode(414);
    pub const UNSUPPORTEDMEDIATYPE: StatusCode = StatusCode(415);
    pub const REQUESTEDRANGENOTSATISFIABLE: StatusCode = StatusCode(416);
    pub const EXPECTATIONFAILED: StatusCode = StatusCode(417);
    pub const INTERNALSERVERERROR: StatusCode = StatusCode(500);
    pub const NOTIMPLEMENTED: StatusCode = StatusCode(501);
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    pub const SERVICEUNAVAILABLE: StatusCode = StatusCode(503);
    pub const GATEWAYTIMEOUT: StatusCode = StatusCode(504);
    pub const HTTPVERSIONNOTSUPPORTED: StatusCode = StatusCode(505);

    pub fn canonical_reason(num: u16) -> Option<&'static str> {
        match num {
            100 => Some("Continue"),
            101 => Some("Switching Protocols"),
            200 => Some("OK"),
            201 => Some("Created"),
            202 => Some("Accepted"),
            203 => Some("Non-Authoritative Information"),
            204 => Some("No Content"),
            205 => Some("Reset Content"),
            206 => Some("Partial Content"),
            300 => Some("Multiple Choices"),
            301 => Some("Moved Permanently"),
            302 => Some("Found"),
            303 => Some("See Other"),
            304 => Some("Not Modified"),
            305 => Some("Use Proxy"),
            307 => Some("Temporary Redirect"),
            400 => Some("Bad Request"),
            401 => Some("Unauthorized"),
            402 => Some("Payment Required"),
            403 => Some("Forbidden"),
            404 => Some("Not Found"),
            405 => Some("Method Not Allowed"),
            406 => Some("Not Acceptable"),
            407 => Some("Proxy Authentication Required"),
            408 => Some("Request Time-out"),
            409 => Some("Conflict"),
            410 => Some("Gone"),
            411 => Some("Length Required"),
            412 => Some("Precondition Failed"),
            413 => Some("Request Entity Too Large"),
            414 => Some("Request-URI Too Large"),
            415 => Some("Unsupported Media Type"),
            416 => Some("Requested range not satisfiable"),
            417 => Some("Expectation Failed"),
            500 => Some("Internal Server Error"),
            501 => Some("Not Implemented"),
            502 => Some("Bad Gateway"),
            503 => Some("Service Unavailable"),
            504 => Some("Gateway Time-out"),
            505 => Some("HTTP Version not supported"),
            _ => None,
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(val: StatusCode) -> Self {
        val.0
    }
}

impl From<StatusCode> for String {
    fn from(val: StatusCode) -> Self {
        StatusCode::canonical_reason(val.into())
            .unwrap_or("")
            .to_string()
    }
}

#[derive(PartialEq)]
pub struct Response {
    pub version: Version,
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<Response> for String {
    fn from(mut value: Response) -> Self {
        let version = value.version;

        let status_code: u16 = value.status.into();
        let status_reason: String = value.status.into();

        let body = value.body;
        let length = body.len();

        let mut headers = String::new();
        value.headers.remove("Content-Length");

        for (key, value) in value.headers.iter() {
            headers += format!("{key}: {value}\r\n").as_str();
        }

        format!(
            "{version} {status_code} {status_reason}\r\n\
            {headers}\
            Content-Length: {length}\r\n\r\n\
            {body}"
        )
    }
}
