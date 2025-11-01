mod types;

pub use types::*;

pub use bitburner_bindings_macros::{bb_bindgen, from_dts};

pub fn v4uuid() -> String {
    let my_uuid = uuid::Uuid::new_v4()
        .as_hyphenated()
        .to_string()
        .as_str()
        .to_owned();
    my_uuid.into()
}

pub use ns::NS;

pub mod ns {
    use super::*;
    from_dts!("./NetscriptDefinitions.d.ts");
}
