use crate::stacktrace::Frame;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception<'a> {
    error_class: &'a str,
    message: &'a str,
    stacktrace: &'a [Frame],
}

impl<'a> Exception<'a> {
    pub fn new(errorclass: &'a str, message: &'a str, stacktrace: &'a [Frame]) -> Exception<'a> {
        Exception {
            error_class: errorclass,
            message,
            stacktrace,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Exception;

    #[test]
    fn test_exception_to_json() {
        let empty_vec = Vec::new();
        let ex = Exception::new("Assert", "Assert", &empty_vec);

        assert_eq!(
            serde_json::to_value(&ex).unwrap(),
            json!({
                "errorClass": "Assert",
                "message": "Assert",
                "stacktrace": []
            })
        );
    }
}
