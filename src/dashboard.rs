use crate::foreman_config::ForemanConfig;
use reqwest::Error;

const AUTH: &str = "xFeGdHjhqsixiZBUntXvSTGGMfsIzOcshZIVxWVoxnjWyzDXhTxHyiwbnoZnTVttFgJFCglnmHYyLhWSverHWCPMGSsumXPkyuWV";

#[derive(Serialize, Deserialize)]
struct NewTeam {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub current_step_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StepRequest {
    group_id: i32,
    step_id: i32,
}

pub fn create_team(name: &str) -> Result<Team, reqwest::Error> {
    let client = reqwest::Client::new();
    let team = NewTeam {
        name: name.to_owned(),
    };

    client
        .post(&format!("{}/teams", super::DASHBOARD_URL))
        .header("auth", AUTH)
        .json(&team)
        .send()?
        .json()
}

pub fn step_forward(config: &ForemanConfig) -> Result<Team, reqwest::Error> {
    let client = reqwest::Client::new();

    let step_request = StepRequest {
        group_id: config.id,
        step_id: config.step + 1,
    };

    client
        .post(&format!(
            "{}/teams/{}/completeStep",
            super::DASHBOARD_URL,
            config.id,
        ))
        .header("auth", AUTH)
        .json(&step_request)
        .send()?
        .json()
}

pub fn step_failed(config: &ForemanConfig) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let step_request = StepRequest {
        group_id: config.id,
        step_id: config.step,
    };

    client
        .post(&format!(
            "{}/teams/{}/failStep",
            super::DASHBOARD_URL,
            config.id,
        ))
        .header("auth", AUTH)
        .json(&step_request)
        .send()?;

    Ok(())
}
