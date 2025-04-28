use std::panic::PanicHookInfo;

pub fn to_message(info: &PanicHookInfo) -> String {
    if let Some(data) = info.payload().downcast_ref::<String>() {
        data.to_owned()
    } else if let Some(data) = info.payload().downcast_ref::<&str>() {
        (*data).to_owned()
    } else {
        format!("Error: {:?}", info.payload())
    }
}
