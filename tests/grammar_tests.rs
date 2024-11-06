use anyhow::anyhow;
use pest::Parser;
use JSON_parser::*;

#[test]
fn parse_object_without_whitespace() -> anyhow::Result<()> {
    // Тест на створення дужок без пробілів
    let pair = Grammar::parse(Rule::MAIN_BRACKETS, "{}")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "{}");
    Ok(())
}

#[test]
fn parse_object_with_whitespace() -> anyhow::Result<()> {
    // Тест на створення дужок з пробілами в середині
    let pair = Grammar::parse(Rule::MAIN_BRACKETS, "{   }")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "{   }");
    Ok(())
}

#[test]
fn parse_invalid_object() {
    // Тест на неповний об'єкт
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{");
    assert!(result.is_err());

    // Тест на зайві дужки відкривання
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{{}");
    assert!(result.is_err());

    // Тест на зайві дужки закривання
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{}}");
    assert!(result.is_err());

    // Тест на двійні дужки
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{{}}");
    assert!(result.is_err());

    // Тест на неправильно введені дужки
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "}{");
    assert!(result.is_err());
}

#[test]
fn parse_non_empty_object() {
    // Тест на введення рядка всередині дужок
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{abc}");
    assert!(result.is_err());

    // Тест на введення чисел всередині дужок
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{43}");
    assert!(result.is_err());

    // Тест на введення символів пунктуації всередині дужок
    let result = Grammar::parse(Rule::MAIN_BRACKETS, "{.}");
    assert!(result.is_err());
}