use std::panic::PanicInfo;

pub fn to_message(info: &PanicInfo) -> String {
    if let Some(data) = info.payload().downcast_ref::<String>() {
        data.to_owned()
    } else if let Some(data) = info.payload().downcast_ref::<&str>() {
        (*data).to_owned()
    } else {
        format!("Error: {:?}", info.payload())
    }
}
