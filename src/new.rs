use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn new_deck (name: Option<&str>) -> Result<(), ExitFailure> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    Ok(())
}
