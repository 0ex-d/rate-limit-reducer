# rate-limit-reducer

A lib to help reduce your app API keys from being rate-limited when processing big data

### Reducer Vector
| name | Type | Description |
|---|---|---|
| app_name | String | The api key username in your 3rd-party dashboard |
| api_key_data | {} | The 3rd-party API key data structure. This could vary depending on the 3rd-party app<br>example: `{"api_key": "some-app-key"}` <br>twitter example: `{"consumer_key": "some-app-key", "consumer_secret": "some-app-key","bearer_token": "some-app-key"}` |
| rate_limit_alloc | Integer | The total rate-limit available per 3rd-party API key. |
| rate_limit_left | Integer | The remaining calls available per 3rd-party API key, this decreases with the number of API calls made. |
| rate_limit_reset_ts | Integer | The timestamp window for a rate-limit to reset after. <br>Some API services reset hourly or x number of minutes on the hour. |

### Unit Tests

`cargo run --workspace -- --nocapture`

### Run the program

`cargo run`

### Use clippy to lint and clean rust code

`cargo clippy --fix --allow-dirty`
