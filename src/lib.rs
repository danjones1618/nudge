use oas3;
pub mod cli;

pub fn load_spec() -> oas3::Spec {
    let json = std::fs::read_to_string("petstore.openapi.json").unwrap();
    let spec = oas3::from_json(json).unwrap();

    spec
}
