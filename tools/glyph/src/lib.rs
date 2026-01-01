use nom::{
    IResult,
    bytes::complete::{tag, take_while1, take_while},
    character::complete::{space0, multispace0, line_ending},
    branch::alt,
    combinator::{opt, map},
    multi::many0,
    sequence::{preceded, terminated},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub name: String,
    pub intent: String,
    pub entities: Vec<Entity>,
    pub dependencies: Vec<String>,
    pub questions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub contract: Option<String>,
    pub guarantees: Vec<String>,
    pub exclusions: Vec<String>,
    pub assumptions: Vec<String>,
}

pub fn parse_glyph(input: &str) -> IResult<&str, Spec> {
    let (input, _) = multispace0(input)?;
    let (input, name) = preceded(tag("#"), parse_name)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, intent) = preceded(tag("^"), parse_text)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, (entities, dependencies, questions)) = parse_body(input)?;
    Ok((input, Spec { name, intent, entities, dependencies, questions }))
}

fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, name) = take_while1(|c: char| {
        c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
    })(input)?;
    Ok((input, name.to_string()))
}

fn parse_text(input: &str) -> IResult<&str, String> {
    let (input, text) = take_while(|c| c != '\n' && c != '\r')(input)?;
    Ok((input, text.trim().to_string()))
}

fn parse_body(input: &str) -> IResult<&str, (Vec<Entity>, Vec<String>, Vec<String>)> {
    let mut entities = Vec::new();
    let mut dependencies = Vec::new();
    let mut questions = Vec::new();
    let mut remaining = input;

    loop {
        // Skip any blank space/newlines between top-level items.
        let (rem, _) = multispace0(remaining)?;
        remaining = rem;

        // Try parse one top-level line/block.
        let (rem, line) = opt(parse_line)(remaining)?;
        remaining = rem;

        match line {
            Some(Line::Entity(e)) => entities.push(e),
            Some(Line::Dep(d)) => dependencies.push(d),
            Some(Line::Question(q)) => questions.push(q),
            None => break,
        }
    }

    Ok((remaining, (entities, dependencies, questions)))
}

#[derive(Debug)]
enum Line {
    Entity(Entity),
    Dep(String),
    Question(String),
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_entity_block, Line::Entity),
        map(
            terminated(preceded(tag(">"), preceded(space0, parse_name)), opt(line_ending)),
            Line::Dep
        ),
        map(
            terminated(preceded(tag("?"), preceded(space0, parse_text)), opt(line_ending)),
            Line::Question
        ),
    ))(input)
}

#[derive(Debug)]
enum IndentedLine {
    Contract(String),
    Guarantee(String),
    Exclusion(String),
    Assumption(String),
}

fn parse_indented_line(input: &str) -> IResult<&str, IndentedLine> {
    let (input, _) = tag("  ")(input)?;

    // Parse the content marker+text
    let (input, item) = alt((
        map(preceded(tag(":"), preceded(space0, parse_text)), IndentedLine::Contract),
        map(preceded(tag("!"), preceded(space0, parse_text)), IndentedLine::Guarantee),
        map(preceded(tag("-"), preceded(space0, parse_text)), IndentedLine::Exclusion),
        map(preceded(tag("~"), preceded(space0, parse_text)), IndentedLine::Assumption),
    ))(input)?;

    // Consume optional line ending so many0 continues properly
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, item))
}

fn parse_entity_block(input: &str) -> IResult<&str, Entity> {
    let (input, name) = preceded(tag("@"), parse_name)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, lines) = many0(parse_indented_line)(input)?;
    let mut contract = None;
    let mut guarantees = Vec::new();
    let mut exclusions = Vec::new();
    let mut assumptions = Vec::new();
    for line in lines {
        match line {
            IndentedLine::Contract(c) => contract = Some(c),
            IndentedLine::Guarantee(g) => guarantees.push(g),
            IndentedLine::Exclusion(e) => exclusions.push(e),
            IndentedLine::Assumption(a) => assumptions.push(a),
        }
    }
    Ok((input, Entity { name, contract, guarantees, exclusions, assumptions }))
}

pub fn verify(spec: &Spec, code_path: &str) -> Result<String, String> {
    let code = std::fs::read_to_string(code_path).map_err(|e| e.to_string())?;

    // Parse to ensure it's syntactically valid Rust (keeps us honest).
    let _ast = syn::parse_file(&code).map_err(|e| e.to_string())?;

    // Collect exclusions from ALL entities.
    let mut exclusions: Vec<String> = Vec::new();
    for e in &spec.entities {
        for ex in &e.exclusions {
            exclusions.push(ex.trim().to_lowercase());
        }
    }

    // Supported exclusions (v0).
    let supported = ["network", "filesystem_writes", "nondeterminism"];

    // Fail-closed: any exclusion present that isn't supported => FAIL.
    for ex in &exclusions {
        if !supported.contains(&ex.as_str()) {
            return Err(format!("Unsupported exclusion in spec (fail-closed): {}", ex));
        }
    }

    // Apply checks.
    for ex in &exclusions {
        match ex.as_str() {
            "network" => {
                let banned = [
                    "std::net",
                    "reqwest",
                    "hyper",
                    "ureq",
                    "tokio::net",
                ];
                if banned.iter().any(|s| code.contains(s)) {
                    return Err("Exclusion violated: network".to_string());
                }
            }
            "filesystem_writes" => {
                let banned = [
                    "std::fs::write",
                    "File::create",
                    "std::fs::File::create",
                    "OpenOptions",
                    "std::fs::OpenOptions",
                    "remove_file",
                    "create_dir",
                    "create_dir_all",
                ];
                if banned.iter().any(|s| code.contains(s)) {
                    return Err("Exclusion violated: filesystem_writes".to_string());
                }
            }
            "nondeterminism" => {
                let banned = [
                    "rand::",
                    "thread_rng",
                    "SystemTime::now",
                    "Instant::now",
                    "Uuid::new_v4",
                ];
                if banned.iter().any(|s| code.contains(s)) {
                    return Err("Exclusion violated: nondeterminism".to_string());
                }
            }
            _ => {
                // Already fail-closed above, but keep defensive.
                return Err(format!("Unsupported exclusion in spec (fail-closed): {}", ex));
            }
        }
    }

    Ok("PASS".to_string())
}
