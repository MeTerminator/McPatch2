use std::time::{SystemTime, Duration, UNIX_EPOCH};

/// 将 SystemTime 转换为 Unix 时间戳的 trait
pub trait UnixTimestampExt {
    /// 转换为秒级 Unix 时间戳
    fn as_unix_seconds(&self) -> u64;
}

impl UnixTimestampExt for SystemTime {
    fn as_unix_seconds(&self) -> u64 {
        self.duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }
}