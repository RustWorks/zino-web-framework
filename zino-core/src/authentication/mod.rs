use crate::{DateTime, Map, Validation};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::Duration;

mod access_key;
mod security_token;

// Rexports.
pub use access_key::{AccessKeyId, SecretAccessKey};
pub(crate) use security_token::ParseTokenError;
pub use security_token::SecurityToken;

/// HTTP signature using RFC 2104 HMAC-SHA1.
pub struct Authentication {
    /// Service name.
    service_name: String,
    /// Access key ID.
    access_key_id: AccessKeyId,
    /// Signature.
    signature: String,
    /// HTTP method.
    method: String,
    /// Accept header value.
    accept: Option<String>,
    /// Content-MD5 header value.
    content_md5: Option<String>,
    /// Content-Type header value.
    content_type: Option<String>,
    /// Date header.
    date_header: (String, DateTime),
    /// Expires.
    expires: Option<DateTime>,
    /// Canonicalized headers.
    headers: Vec<(String, String)>,
    /// Canonicalized resource.
    resource: String,
}

impl Authentication {
    /// Creates a new instance.
    #[inline]
    pub fn new(method: &str) -> Self {
        Self {
            service_name: String::new(),
            access_key_id: AccessKeyId::default(),
            signature: String::new(),
            method: method.to_ascii_uppercase(),
            accept: None,
            content_md5: None,
            content_type: None,
            date_header: ("Date".to_string(), DateTime::now()),
            expires: None,
            headers: Vec::new(),
            resource: String::new(),
        }
    }

    /// Sets the service name.
    #[inline]
    pub fn set_service_name(&mut self, service_name: &str) {
        self.service_name = service_name.to_ascii_uppercase();
    }

    /// Sets the access key ID.
    #[inline]
    pub fn set_access_key_id(&mut self, access_key_id: impl Into<AccessKeyId>) {
        self.access_key_id = access_key_id.into();
    }

    /// Sets the signature.
    #[inline]
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    /// Sets the `Accept` header value.
    #[inline]
    pub fn set_accept(&mut self, accept: impl Into<Option<String>>) {
        self.accept = accept.into();
    }

    /// Sets the `Content-MD5` header value.
    #[inline]
    pub fn set_content_md5(&mut self, content_md5: String) {
        self.content_md5 = Some(content_md5);
    }

    /// Sets the `Content-Type` header value.
    #[inline]
    pub fn set_content_type(&mut self, content_type: impl Into<Option<String>>) {
        self.content_type = content_type.into();
    }

    /// Sets the `Date` header value.
    #[inline]
    pub fn set_date_header(&mut self, header_name: String, date: DateTime) {
        self.date_header = (header_name, date);
    }

    /// Sets the expires timestamp.
    #[inline]
    pub fn set_expires(&mut self, expires: impl Into<Option<DateTime>>) {
        self.expires = expires.into();
    }

    /// Sets the canonicalized headers.
    /// The header is matched if it has a prefix in the filter list.
    #[inline]
    pub fn set_headers(
        &mut self,
        headers: impl Iterator<Item = (String, String)>,
        filter: &[&'static str],
    ) {
        let mut headers = headers
            .filter_map(|(name, values)| {
                let key = name.as_str();
                filter
                    .iter()
                    .any(|&s| key.starts_with(s))
                    .then(|| (key.to_ascii_lowercase(), values.to_string()))
            })
            .collect::<Vec<_>>();
        headers.sort_by(|a, b| a.0.cmp(&b.0));
        self.headers = headers;
    }

    /// Sets the canonicalized resource.
    #[inline]
    pub fn set_resource(&mut self, path: String, query: impl Into<Option<Map>>) {
        if let Some(query) = query.into() {
            if query.is_empty() {
                self.resource = path;
            } else {
                let mut query_pairs = query.iter().collect::<Vec<_>>();
                query_pairs.sort_by(|a, b| a.0.cmp(b.0));

                let query = query_pairs
                    .iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>();
                self.resource = path + "?" + &query.join("&");
            }
        } else {
            self.resource = path;
        }
    }

    /// Returns the service name.
    #[inline]
    pub fn service_name(&self) -> &str {
        self.service_name.as_str()
    }

    /// Returns the access key ID.
    #[inline]
    pub fn access_key_id(&self) -> &str {
        self.access_key_id.as_str()
    }

    /// Returns the signature.
    #[inline]
    pub fn signature(&self) -> &str {
        self.signature.as_str()
    }

    /// Returns an `Authorization` header value.
    #[inline]
    pub fn authorization(&self) -> String {
        let service_name = self.service_name();
        let access_key_id = self.access_key_id();
        let signature = self.signature();
        if service_name.is_empty() {
            format!("{access_key_id}:{signature}")
        } else {
            format!("{service_name} {access_key_id}:{signature}")
        }
    }

    /// Returns the string to sign.
    pub fn string_to_sign(&self) -> String {
        let mut sign_parts = Vec::new();

        // HTTP verb
        let method = self.method.to_string();
        sign_parts.push(method);

        // Accept
        if let Some(accept) = self.accept.as_ref() {
            let accept = accept.to_string();
            sign_parts.push(accept);
        }

        // Content-MD5
        let content_md5 = self
            .content_md5
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_default();
        sign_parts.push(content_md5);

        // Content-Type
        let content_type = self
            .content_type
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_default();
        sign_parts.push(content_type);

        // Expires.
        if let Some(expires) = self.expires.as_ref() {
            let expires = expires.timestamp().to_string();
            sign_parts.push(expires);
        } else {
            // Date
            let date_header = &self.date_header;
            let date = if date_header.0.eq_ignore_ascii_case("Date") {
                date_header.1.to_utc_string()
            } else {
                "".to_string()
            };
            sign_parts.push(date);
        }

        // Canonicalized headers
        let headers = self
            .headers
            .iter()
            .map(|(name, values)| format!("{}:{}", name, values.trim()))
            .collect::<Vec<_>>();
        sign_parts.extend(headers);

        // Canonicalized resource
        let resource = self.resource.to_string();
        sign_parts.push(resource);

        sign_parts.join("\n")
    }

    /// Generates a signature with the secret access key.
    pub fn sign_with(&self, secret_access_key: SecretAccessKey) -> String {
        let string_to_sign = self.string_to_sign();
        let mut mac = Hmac::<Sha1>::new_from_slice(secret_access_key.as_ref())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_ref());
        base64::encode(mac.finalize().into_bytes())
    }

    /// Validates the signature using the secret access key.
    pub fn validate_with(&self, secret_access_key: SecretAccessKey) -> Validation {
        let mut validation = Validation::new();
        let current = DateTime::now();
        let date = self.date_header.1;
        let max_tolerance = Duration::from_secs(900);
        if date < current && date < current - max_tolerance
            || date > current && date > current + max_tolerance
        {
            validation.record_fail("date", "untrusted date");
        }
        if let Some(expires) = self.expires {
            if current > expires {
                validation.record_fail("expires", "valid period has expired");
            }
        }

        let signature = self.signature();
        if signature.is_empty() || self.sign_with(secret_access_key) == signature {
            validation.record_fail("signature", "invalid signature");
        }
        validation
    }
}