use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Error};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiKeysData {
    app_name: String,
    api_key_data: HashMap<String, String>,
    pub rate_limit_alloc: u64,
    pub rate_limit_left: u64,
    pub rate_limit_reset_ts: u64,
}

impl ApiKeysData {
    pub fn rotate_api_keys(
        api_keys_data: &Vec<ApiKeysData>,
        mut current_pos: usize,
    ) -> Option<&ApiKeysData> {
        if api_keys_data.is_empty() {
            return None;
        }

        // iter next vector location
        current_pos += 1;

        // ensure the pointing location is always available otherwise reset to 0-th index
        if api_keys_data.len() - 1 <= current_pos {
            current_pos = 0;
        }

        // return the api key matching
        let data = api_keys_data.get(current_pos);
        data?;

        if data?.rate_limit_left < 1 {
            return None;
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_if_all_rate_limit() {
        let mock_api_key_a: Result<ApiKeysData, Error> = from_value::<ApiKeysData>(json!({
                "AppName": "research-lab-a",
                "ApiKeyData": {},
                "RateLimitAlloc": 0,
                "RateLimitLeft": 0,
                "RateLimitResetTs": 0
        }));
        let mock_api_key_a = vec![mock_api_key_a.unwrap()];

        let reducer = ApiKeysData::rotate_api_keys(&mock_api_key_a, 0);
        assert!(reducer.is_none());
    }

    #[test]
    fn should_return_true_if_not_rate_limit() {
        let mock_api_key_a: Result<ApiKeysData, Error> = from_value::<ApiKeysData>(json!({
                "AppName": "research-lab-a",
                "ApiKeyData": {"Key":"some-app-key"},
                "RateLimitAlloc": 100,
                "RateLimitLeft": 12,
                "RateLimitResetTs": 0
        }));
        let mock_api_key_a = vec![mock_api_key_a.unwrap()];

        let reducer = ApiKeysData::rotate_api_keys(&mock_api_key_a, 0);
        assert!(reducer.is_some());
    }
}
