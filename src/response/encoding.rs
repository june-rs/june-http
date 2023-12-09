use strum::{AsRefStr, Display, EnumString};

// CONTENT ENCODING

#[derive(Debug, AsRefStr, Display, EnumString, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "lowercase")]
pub enum ContentEncoding {
    Gzip = 3,
    Compress = 5,
    Deflate = 4,
    #[strum(serialize = "br")]
    Brotli = 2,
    Zstd = 1,
    Identity = 6,
}

// TRANSFER ENCODING

#[derive(Debug, AsRefStr, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum TransferEncoding {
    Chunked,
    Compress,
    Deflate,
    Gzip,
}

// BODY ENCODING

pub struct BodyEncoding {
    pub content: ContentEncoding,
    pub transfer: Option<TransferEncoding>,
}

impl Default for BodyEncoding {
    fn default() -> Self {
        Self {
            content: ContentEncoding::Identity,
            transfer: None,
        }
    }
}