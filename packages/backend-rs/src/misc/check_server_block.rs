use crate::misc::meta::fetch_meta;
use sea_orm::DbErr;

/// Checks if a server is blocked.
///
/// ## Argument
/// `host` - punycoded instance host
#[crate::export]
pub async fn is_blocked_server(host: &str) -> Result<bool, DbErr> {
    Ok(fetch_meta(true)
        .await?
        .blocked_hosts
        .iter()
        .any(|blocked_host| {
            host == blocked_host || host.ends_with(format!(".{}", blocked_host).as_str())
        }))
}

/// Checks if a server is silenced.
///
/// ## Argument
/// `host` - punycoded instance host
#[crate::export]
pub async fn is_silenced_server(host: &str) -> Result<bool, DbErr> {
    Ok(fetch_meta(true)
        .await?
        .silenced_hosts
        .iter()
        .any(|silenced_host| {
            host == silenced_host || host.ends_with(format!(".{}", silenced_host).as_str())
        }))
}

/// Checks if a server is allowlisted.
/// Returns `Ok(true)` if private mode is disabled.
///
/// ## Argument
/// `host` - punycoded instance host
#[crate::export]
pub async fn is_allowed_server(host: &str) -> Result<bool, DbErr> {
    let meta = fetch_meta(true).await?;

    if !meta.private_mode.unwrap_or(false) {
        return Ok(true);
    }
    if let Some(allowed_hosts) = meta.allowed_hosts {
        return Ok(allowed_hosts.contains(&host.to_string()));
    }
    Ok(false)
}
