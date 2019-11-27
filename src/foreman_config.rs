use crate::dashboard::Team;
use short_crypt::ShortCrypt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

const SECRET: &str = "LACLEFMEGALONGUEPOURCRYPTERLESTRUC";

#[derive(Serialize, Deserialize, Debug)]
pub struct ForemanConfig {
    pub id: i32,
    pub name: String,
    pub step: i32,
}

pub fn write(team: &Team) -> std::io::Result<()> {
    let _ = fs::remove_file("config");

    let config = ForemanConfig {
        id: team.id,
        name: team.name.clone(),
        step: team.current_step_id,
    };

    let serialized = serde_json::to_string(&config).unwrap();

    let sc = ShortCrypt::new(SECRET);
    let encrypted_file = sc.encrypt_to_qr_code_alphanumeric(&serialized);
    let mut file = File::create("config")?;

    file.write_all(encrypted_file.as_bytes())?;

    Ok(())
}

pub fn get() -> std::io::Result<ForemanConfig> {
    let sc = ShortCrypt::new(SECRET);

    let mut file = File::open("config")?;
    let mut encrypted_config = String::new();

    file.read_to_string(&mut encrypted_config)?;

    let decripted_config = sc.decrypt_qr_code_alphanumeric(&encrypted_config).unwrap();
    let decripted_config = String::from_utf8(decripted_config).unwrap();
    let config: ForemanConfig = serde_json::from_str(&decripted_config).unwrap();

    Ok(config)
}
