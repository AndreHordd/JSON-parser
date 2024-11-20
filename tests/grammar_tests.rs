use anyhow::{anyhow, Result};
use pest::Parser;
use JSON_parser::{JSONParser, Rule};

#[test]
fn test_parse_json() -> Result<()> {
    let pair = JSONParser::parse(Rule::JSON, r#"{"key": "value"}"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити JSON"))?;
    assert_eq!(pair.as_rule(), Rule::JSON);
    Ok(())
}

#[test]
fn test_parse_value_string() -> Result<()> {
    let pair = JSONParser::parse(Rule::VALUE, r#""Hello""#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити VALUE"))?;
    assert_eq!(pair.as_rule(), Rule::VALUE);
    Ok(())
}

#[test]
fn test_parse_object() -> Result<()> {
    let pair = JSONParser::parse(Rule::OBJECT, r#"{"key": "value"}"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити OBJECT"))?;
    assert_eq!(pair.as_rule(), Rule::OBJECT);
    Ok(())
}

#[test]
fn test_parse_members() -> Result<()> {
    let pair = JSONParser::parse(Rule::MEMBERS, r#""key1": "value1", "key2": "value2""#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити MEMBERS"))?;
    assert_eq!(pair.as_rule(), Rule::MEMBERS);
    Ok(())
}

#[test]
fn test_parse_pair() -> Result<()> {
    let pair = JSONParser::parse(Rule::PAIR, r#""key": "value""#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити PAIR"))?;
    assert_eq!(pair.as_rule(), Rule::PAIR);
    Ok(())
}

#[test]
fn test_parse_array() -> Result<()> {
    let pair = JSONParser::parse(Rule::ARRAY, r#"[1, 2, 3]"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити ARRAY"))?;
    assert_eq!(pair.as_rule(), Rule::ARRAY);
    Ok(())
}

#[test]
fn test_parse_elements() -> Result<()> {
    let pair = JSONParser::parse(Rule::ELEMENTS, r#"1, 2, 3"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити ELEMENTS"))?;
    assert_eq!(pair.as_rule(), Rule::ELEMENTS);
    Ok(())
}

#[test]
fn test_parse_string() -> Result<()> {
    let pair = JSONParser::parse(Rule::STRING, r#""Hello""#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити STRING"))?;
    assert_eq!(pair.as_rule(), Rule::STRING);
    Ok(())
}

#[test]
fn test_parse_escaped_char() -> Result<()> {
    let pair = JSONParser::parse(Rule::ESCAPED_CHAR, r#"\""#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити ESCAPED_CHAR"))?;
    assert_eq!(pair.as_rule(), Rule::ESCAPED_CHAR);
    Ok(())
}

#[test]
fn test_parse_non_escaped_char() -> Result<()> {
    let pair = JSONParser::parse(Rule::NON_ESCAPED_CHAR, r#"a"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити NON_ESCAPED_CHAR"))?;
    assert_eq!(pair.as_rule(), Rule::NON_ESCAPED_CHAR);
    Ok(())
}

#[test]
fn test_parse_number() -> Result<()> {
    let pair = JSONParser::parse(Rule::NUMBER, r#"123"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити NUMBER"))?;
    assert_eq!(pair.as_rule(), Rule::NUMBER);
    Ok(())
}

#[test]
fn test_parse_true() -> Result<()> {
    let pair = JSONParser::parse(Rule::TRUE, r#"true"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити TRUE"))?;
    assert_eq!(pair.as_rule(), Rule::TRUE);
    Ok(())
}

#[test]
fn test_parse_false() -> Result<()> {
    let pair = JSONParser::parse(Rule::FALSE, r#"false"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити FALSE"))?;
    assert_eq!(pair.as_rule(), Rule::FALSE);
    Ok(())
}

#[test]
fn test_parse_null() -> Result<()> {
    let pair = JSONParser::parse(Rule::NULL, r#"null"#)?
        .next()
        .ok_or_else(|| anyhow!("Не вдалося розпарсити NULL"))?;
    assert_eq!(pair.as_rule(), Rule::NULL);
    Ok(())
}
