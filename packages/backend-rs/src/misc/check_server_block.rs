use crate::misc::meta::fetch_meta;
use sea_orm::DbErr;

/**
 * @param host punycoded instance host
 * @returns whether the given host should be blocked
 */
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

/**
 * @param host punycoded instance host
 * @returns whether the given host should be limited
 */
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

/**
 * @param host punycoded instance host
 * @returns whether the given host is allowlisted (this is always true if private mode is disabled)
 */
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
