use log::{error, info};

use reqwest::StatusCode;
cfg_if::cfg_if! {
    if #[cfg(test)]{
        use mockall::automock;
        use core::fmt::Formatter;
        use std::fmt::Display;

        struct Client{
            url_passed_to_post: String,
            headers: Vec<(String, String)>,
            body: HashMap<String, f32>,
        }

        struct Response {
            status_code: StatusCode,
        }

        #[automock]
        impl Response {
            pub fn status(&self) -> StatusCode {
                self.status_code
            }
        }

        impl Client{
            pub fn new() -> Self {
                Client{
                    url_passed_to_post: "".to_string(),
                    headers: Vec::new(),
                    body: HashMap::new(),
                }
            }

            pub fn post(&self, url: String) -> Self {
                Client{
                    url_passed_to_post: url,
                    headers: self.headers.clone(),
                    body: self.body.clone(),

                }
            }

            pub fn header(&self, header_name: &str, header_value: &str) -> Self {
                let mut headers = self.headers.clone();
                headers.push((header_name.to_string(), header_value.to_string()));
                Client { url_passed_to_post: self.url_passed_to_post.clone(), headers: self.headers.clone(), body: self.body.clone() }
            }

            pub fn json<'a>(&self, json_body: &HashMap<&'a str, f32>) -> Self {
                Client {
                    url_passed_to_post: self.url_passed_to_post.clone(),
                    headers: self.headers.clone(),
                    body: json_body.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
                }
            }

            pub async fn send(&self) -> Result<Response, String> {
                Ok(Response { status_code: StatusCode::CREATED })
            }
        }

        impl Display for Response {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Status Code: {}", self.status_code)
            }
        }

    }else {
        use reqwest::Client;
    }
}

use std::{collections::HashMap, env};

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

static DATA_COLLECTION_URL_KEY: &str = "DATA_COLLECTION_URL";
static DATA_COLLECTION_ENABLED_KEY: &str = "DATA_COLLECTION_ENABLED";
static DATA_COLLECTION_SECRET_KEY: &str = "DATA_COLLECTION_SECRET";

pub enum DataCollectionError {
    DataCollectionDisabled,
    ValueHasNotChanged,
    DataCollectionError(StatusCode),
    SystemError(String),
}

pub async fn collect_data(
    water_temperature_sensor: &WaterTemperatureSensor,
) -> Result<StatusCode, DataCollectionError> {
    let collection_enabled: bool = env::var(DATA_COLLECTION_ENABLED_KEY.to_string())
        .expect("DATA_COLLECTION_ENABLED must be set")
        .trim()
        .parse()
        .unwrap();

    if !collection_enabled {
        return Err(DataCollectionError::DataCollectionDisabled);
    }

    if water_temperature_sensor.should_collect_data() {
        let mut json_body = HashMap::new();
        json_body.insert(
            "temperature_in_celcius",
            water_temperature_sensor.current_temperature,
        );

        let url =
            env::var(DATA_COLLECTION_URL_KEY.to_string()).expect("DATA_COLLECTION_URL must be set");

        let data_collection_auth = env::var(DATA_COLLECTION_SECRET_KEY.to_string())
            .expect("DATA_COLLECTION_SECRET must be set");

        let result_query = Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Require-Whisk-Auth", data_collection_auth.as_str())
            .json(&json_body)
            .send()
            .await;

        match result_query {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    info!("Data collected successfully");
                    return Ok(response.status());
                }
                _ => {
                    error!("{}", response.status());
                    return Err(DataCollectionError::DataCollectionError(response.status()));
                }
            },
            Err(e) => {
                error!("Error: {}", e);
                return Err(DataCollectionError::SystemError(e.to_string()));
            }
        }
    } else {
        return Err(DataCollectionError::ValueHasNotChanged);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

    fn mock_env_variable(key_value_variables: HashMap<String, String>) {
        for key_value_variable in key_value_variables {
            env::set_var(key_value_variable.0, key_value_variable.1);
        }
    }

    #[tokio::test]
    async fn collect_data_should_send_data_to_the_server() {
        let key_value_variables = HashMap::from([
            (
                DATA_COLLECTION_URL_KEY.to_string(),
                "http://127.0.0.1".to_string(),
            ),
            (DATA_COLLECTION_SECRET_KEY.to_string(), "Nope".to_string()),
            (DATA_COLLECTION_ENABLED_KEY.to_string(), "true".to_string()),
        ]);

        mock_env_variable(key_value_variables);

        let mut water_temperature_sensor = WaterTemperatureSensor::new();
        water_temperature_sensor.current_temperature = 10.0;

        collect_data(&water_temperature_sensor).await;
    }
}
