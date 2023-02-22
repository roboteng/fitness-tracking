use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (value, _context) = event.into_parts();
    let first_name = get_first_name(value);

    Ok(json!({ "message": make_message(first_name) }))
}

fn get_first_name(value: Value) -> Option<String> {
    value["firstName"].as_str().map(|s| s.to_owned())
}

fn make_message(name: Option<String>) -> String {
    format!("Hello, {}", name.unwrap_or(String::from("world")))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn message_says_hello_world_if_no_name_is_given() {
        let actual = make_message(None);
        let expected = String::from("Hello, world");

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn message_says_hello_trevor_if_given_trevor() {
        let actual = make_message(Some(String::from("Trevor")));
        let expected = String::from("Hello, Trevor");

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn we_can_pull_the_first_name_out_of_the_event() {
        let value = json!({"firstName": "Trevor"});

        let first_name = get_first_name(value);

        assert_eq!(first_name, Some(String::from("Trevor")));
    }
}
