#[derive(Debug)]
pub struct Player {
    pub username: String,
    pub guid: u128,
}

pub fn format_guid(guid: u128) -> String {
    let entire = format!("{:x?}", guid);

    return format!(
        "{}-{}-{}-{}-{}",
        entire[..8].to_string(),
        entire[8..12].to_string(),
        entire[12..16].to_string(),
        entire[16..20].to_string(),
        entire[20..].to_string()
    );
}
