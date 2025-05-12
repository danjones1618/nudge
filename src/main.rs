use oas3;

fn main() {
    let json = std::fs::read_to_string("petstore.openapi.json").unwrap();

    let spec = oas3::from_json(json).unwrap();
}
