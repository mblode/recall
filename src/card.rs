/// The core struct of Recall representing a unit of learning or a so-called
/// flash-card.
pub struct Card {
    id: u64,
    question: String,
    answer: String,
}

impl Card {
    /// Parse ID of a `Card` from a HEX string.
    pub fn parse_id(id: &str) -> Result<u64, String> {
        u64::from_str_radix(id, 16).map_err(|r| format!("Failed to parse card ID: {}", r))
    }

    /// Serialize ID of a `Card` to a HEX string.
    pub fn serialize_id(id: u64) -> String {
        format!("{:016x}", id)
    }

    pub fn new(id: u64, question: String, answer: String) -> Card {
        Card {
            id,
            question,
            answer,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_id() {
        assert_eq![
            Card::parse_id("B000000000000001").unwrap(),
            12682136550675316737
        ];
        assert_eq![
            Card::parse_id("b000000000000001").unwrap(),
            12682136550675316737
        ];
        assert_eq![
            Card::parse_id("xxx").err().unwrap(),
            String::from("Failed to parse card ID: invalid digit found in string")
        ];
    }

    #[test]
    fn test_serialize_id() {
        assert_eq![Card::serialize_id(49154), String::from("000000000000c002")];
    }

    #[test]
    fn test_card() {
        let card = Card::new(123, String::from("What?"), String::from("Something!"));
        assert_eq!(card.id(), 123);
        assert_eq!(card.question(), "What?");
        assert_eq!(card.answer(), "Something!");
    }
}
