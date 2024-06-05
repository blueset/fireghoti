//! This module is used in the TypeScript backend only.

#[crate::ts_export]
pub const SECOND: i32 = 1000;
#[crate::ts_export]
pub const MINUTE: i32 = 60 * SECOND;
#[crate::ts_export]
pub const HOUR: i32 = 60 * MINUTE;
#[crate::ts_export]
pub const DAY: i32 = 24 * HOUR;

#[crate::ts_export]
pub const USER_ONLINE_THRESHOLD: i32 = 10 * MINUTE;
#[crate::ts_export]
pub const USER_ACTIVE_THRESHOLD: i32 = 3 * DAY;

/// List of file types allowed to be viewed directly in the browser
///
/// Anything not included here will be responded as application/octet-stream
/// SVG is not allowed because it generates XSS (TODO: fix this and later allow it to be viewed directly)
/// * <https://github.com/sindresorhus/file-type/blob/main/supported.js>
/// * <https://github.com/sindresorhus/file-type/blob/main/core.js>
/// * <https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Containers>
#[crate::ts_export]
pub const FILE_TYPE_BROWSERSAFE: [&str; 41] = [
    // Images
    "image/png",
    "image/gif", // TODO: deprecated, but still used by old posts, new gifs should be converted to webp in the future
    "image/jpeg",
    "image/webp", // TODO: make this the default image format
    "image/apng",
    "image/bmp",
    "image/tiff",
    "image/x-icon",
    "image/avif", // not as good supported now, but its good to introduce initial support for the future
    // OggS
    "audio/opus",
    "video/ogg",
    "audio/ogg",
    "application/ogg",
    // ISO/IEC base media file format
    "video/quicktime",
    "video/mp4",     // TODO: we need to check for av1 later
    "video/vnd.avi", // also av1
    "audio/mp4",
    "video/x-m4v",
    "audio/x-m4a",
    "video/3gpp",
    "video/3gpp2",
    "video/3gp2",
    "audio/3gpp",
    "audio/3gpp2",
    "audio/3gp2",
    "video/mpeg",
    "audio/mpeg",
    "video/webm",
    "audio/webm",
    "audio/aac",
    "audio/x-flac",
    "audio/flac",
    "audio/vnd.wave",
    "audio/mod",
    "audio/x-mod",
    "audio/s3m",
    "audio/x-s3m",
    "audio/xm",
    "audio/x-xm",
    "audio/it",
    "audio/x-it",
];
