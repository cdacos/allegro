pub fn record_type_must_be(expected: &'static str) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if value == expected {
            Ok(())
        } else {
            Err(format!("Expected record type '{}', found '{}'", expected, value))
        }
    }
}

pub fn date_yyyymmdd(value: &str) -> Result<(), String> {
    if value.len() != 8 {
        return Err("Date must be 8 characters (YYYYMMDD)".to_string());
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err("Date must contain only digits".to_string());
    }
    
    let year: u32 = value[0..4].parse().map_err(|_| "Invalid year")?;
    let month: u32 = value[4..6].parse().map_err(|_| "Invalid month")?;
    let day: u32 = value[6..8].parse().map_err(|_| "Invalid day")?;
    
    if year < 1900 || year > 2100 {
        return Err("Year must be between 1900 and 2100".to_string());
    }
    
    if month < 1 || month > 12 {
        return Err("Month must be between 01 and 12".to_string());
    }
    
    if day < 1 || day > 31 {
        return Err("Day must be between 01 and 31".to_string());
    }
    
    Ok(())
}

pub fn one_of(allowed: &'static [&'static str]) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if allowed.contains(&value) {
            Ok(())
        } else {
            Err(format!("Value '{}' not in allowed list: {:?}", value, allowed))
        }
    }
}

pub fn numeric_range(min: u32, max: u32) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        let num: u32 = value.parse().map_err(|_| format!("'{}' is not a valid number", value))?;
        if num >= min && num <= max {
            Ok(())
        } else {
            Err(format!("Number {} must be between {} and {}", num, min, max))
        }
    }
}

pub fn alphanumeric(value: &str) -> Result<(), String> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err("Value must contain only alphanumeric characters".to_string())
    }
}

pub fn yes_no(value: &str) -> Result<(), String> {
    if value == "Y" || value == "N" {
        Ok(())
    } else {
        Err("Value must be 'Y' or 'N'".to_string())
    }
}

pub fn works_count(value: &str) -> Result<(), String> {
    let num: u32 = value.parse().map_err(|_| format!("'{}' is not a valid number", value))?;
    if num >= 1 && num <= 99999 {
        Ok(())
    } else {
        Err(format!("Number of works {} must be between 1 and 99999", num))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_type_must_be() {
        let validator = record_type_must_be("AGR");
        assert!(validator("AGR").is_ok());
        assert!(validator("ARI").is_err());
    }

    #[test]
    fn test_date_yyyymmdd() {
        assert!(date_yyyymmdd("20231201").is_ok());
        assert!(date_yyyymmdd("2023120").is_err()); // too short
        assert!(date_yyyymmdd("2023121").is_err()); // too short
        assert!(date_yyyymmdd("202312011").is_err()); // too long
        assert!(date_yyyymmdd("20231301").is_err()); // invalid month
        assert!(date_yyyymmdd("20230001").is_err()); // invalid day
        assert!(date_yyyymmdd("20230132").is_err()); // invalid day
        assert!(date_yyyymmdd("abcd1201").is_err()); // non-numeric
    }

    #[test]
    fn test_one_of() {
        let validator = one_of(&["Y", "N"]);
        assert!(validator("Y").is_ok());
        assert!(validator("N").is_ok());
        assert!(validator("X").is_err());
        assert!(validator("").is_err());
    }

    #[test]
    fn test_numeric_range() {
        let validator = numeric_range(1, 100);
        assert!(validator("1").is_ok());
        assert!(validator("50").is_ok());
        assert!(validator("100").is_ok());
        assert!(validator("0").is_err());
        assert!(validator("101").is_err());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_alphanumeric() {
        assert!(alphanumeric("ABC123").is_ok());
        assert!(alphanumeric("abc").is_ok());
        assert!(alphanumeric("123").is_ok());
        assert!(alphanumeric("ABC-123").is_err()); // hyphen not allowed
        assert!(alphanumeric("ABC 123").is_err()); // space not allowed
        assert!(alphanumeric("").is_ok()); // empty is ok
    }

    #[test]
    fn test_yes_no() {
        assert!(yes_no("Y").is_ok());
        assert!(yes_no("N").is_ok());
        assert!(yes_no("X").is_err());
        assert!(yes_no("").is_err());
        assert!(yes_no("y").is_err()); // lowercase not allowed
        assert!(yes_no("YES").is_err()); // full word not allowed
    }

    #[test]
    fn test_works_count() {
        assert!(works_count("1").is_ok());
        assert!(works_count("50").is_ok());
        assert!(works_count("99999").is_ok());
        assert!(works_count("0").is_err()); // below minimum
        assert!(works_count("100000").is_err()); // above maximum
        assert!(works_count("abc").is_err()); // non-numeric
        assert!(works_count("").is_err()); // empty
    }

}