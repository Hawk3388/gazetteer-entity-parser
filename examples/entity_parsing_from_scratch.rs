extern crate gazetteer_entity_parser;

use gazetteer_entity_parser::*;

fn main() {
    let gazetteer = gazetteer!(
        ("king of pop", "Michael Jackson"),
        ("the rolling stones", "The Rolling Stones"),
        ("the crying stones", "The Crying Stones"),
        ("the fab four", "The Beatles"),
        ("queen of soul", "Aretha Franklin"),
    );
    let parser = ParserBuilder::default()
        .gazetteer(gazetteer)
        .minimum_tokens_ratio(2. / 3.)
        .build()
        .unwrap();

    let sentence = "My favourite artists are the stones and fab four";
    let extracted_entities = parser.run(sentence, 5).unwrap();
    assert_eq!(
        extracted_entities,
        vec![
            ParsedValue {
                matched_value: "the stones".to_string(),
                resolved_value: ResolvedValue {
                    resolved: "The Rolling Stones".to_string(),
                    raw_value: "the rolling stones".to_string(),
                },
                alternatives: vec![ResolvedValue {
                    resolved: "The Crying Stones".to_string(),
                    raw_value: "the crying stones".to_string(),
                }],
                range: 25..35,
            },
            ParsedValue {
                matched_value: "fab four".to_string(),
                resolved_value: ResolvedValue {
                    resolved: "The Beatles".to_string(),
                    raw_value: "the fab four".to_string(),
                },
                alternatives: vec![],
                range: 40..48,
            }
        ]
    );
}
