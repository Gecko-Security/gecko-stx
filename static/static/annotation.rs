use clarity::vm::representations::Span;
use clarity::vm::ClarityName;
use regex::Regex;

#[derive(Debug)]
pub enum AnnotationKind {
    Allow(WarningKind),
    Filter(Vec<ClarityName>),
    FilterAll,
}

impl std::str::FromStr for AnnotationKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([[:word:]]+)(\(([^)]+)\))?").unwrap();
        if let Some(captures) = re.captures(s) {
            let (base, value) = if captures.get(1).is_some() && captures.get(3).is_some() {
                (&captures[1], &captures[3])
            } else {
                (&captures[1], "")
            };
            match base {
                "allow" => match value.parse() {
                    Ok(value) => Ok(AnnotationKind::Allow(value)),
                    Err(_) => Err("missing value for 'allow' annotation".to_string()),
                },
                "filter" => {
                    if value == "*" {
                        Ok(AnnotationKind::FilterAll)
                    } else {
                        let params: Vec<ClarityName> = value
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| ClarityName::from(s.trim()))
                            .collect();
                        if params.is_empty() {
                            Err("missing value for 'filter' annotation".to_string())
                        } else {
                            Ok(AnnotationKind::Filter(params))
                        }
                    }
                }
                _ => Err("unrecognized annotation".to_string()),
            }
        } else {
            Err("malformed annotation".to_string())
        }
    }
}
