#[derive(Debug)]
pub struct Message {
    pub source: Option<String>,
    pub command: String,
    pub parameters: Vec<String>,
}

pub enum ParsingStage {
    Beggining,
    Tags,
    Source,
    Command,
    Parameters,
}

pub enum ParameterParsingStage {
    String,
    General,
}

pub fn parse_message(raw: &[u8]) -> Result<Message, &'static str> {
    let mut source: String = "".to_string();
    let mut command: String = "".to_string();
    let mut raw_parameters = "".to_string();
    let mut parsing_stage = ParsingStage::Beggining;
    let vectorized_raw = raw.to_vec();
    let mut peekable_raw = vectorized_raw.iter().peekable();

    while peekable_raw.peek().is_some() {
        let current_char = peekable_raw.next().unwrap();
        match *current_char as char {
            ':' => match parsing_stage {
                ParsingStage::Beggining => {
                    parsing_stage = ParsingStage::Source;
                    let (s, p) = consume_attribute(source, peekable_raw, false);
                    source = s;
                    peekable_raw = p;
                }
                _ => {}
            },
            _ => match parsing_stage {
                ParsingStage::Beggining | ParsingStage::Source | ParsingStage::Tags => {
                    parsing_stage = ParsingStage::Command;
                    command += &(*current_char as char).to_string();
                    let (c, p) = consume_attribute(command, peekable_raw, false);
                    command = c;
                    peekable_raw = p;
                }
                ParsingStage::Command => {
                    parsing_stage = ParsingStage::Parameters;
                    raw_parameters += &(*current_char as char).to_string();
                    let (pa, p) = consume_attribute(raw_parameters, peekable_raw, true);
                    raw_parameters = pa;
                    peekable_raw = p;
                }
                _ => {}
            },
        }
    }

    let parameters: Vec<String> = parse_parameters(raw_parameters);

    let message = Message {
        source: Some(source),
        command,
        parameters,
    };

    Ok(message)
}

pub fn consume_attribute(
    mut source: String,
    mut peekable_raw: std::iter::Peekable<std::slice::Iter<u8>>,
    all: bool,
) -> (String, std::iter::Peekable<std::slice::Iter<u8>>) {
    if !all {
        while peekable_raw.peek().is_some() && (**peekable_raw.peek().unwrap() as char) != ':' {
            let current_char = peekable_raw.next().unwrap();
            match *current_char as char {
                ' ' => {
                    break;
                }
                _ => {
                    source += &(*current_char as char).to_string();
                }
            }
        }
    } else {
        while peekable_raw.peek().is_some() {
            let current_char = peekable_raw.next().unwrap();
            source += &(*current_char as char).to_string();
        }
    }
    (source, peekable_raw)
}

pub fn parse_parameters(parameters: String) -> Vec<String> {
    let bytes_parameters = parameters.into_bytes();
    let peekable_parameters = bytes_parameters.iter();
    let mut peekable_parameters = peekable_parameters.peekable();
    // let mut stage = ParameterParsingStage::General;
    let mut parameters: Vec<String> = Vec::<String>::new();
    while peekable_parameters.peek().is_some() {
        let current_char = peekable_parameters.next().unwrap();
        match *current_char as char {
            ' '|'\r'|'\n' => continue,
            ':' => {
                let (s, p) = consume_string(peekable_parameters, false);
                peekable_parameters = p;
                parameters.push(s);
            }
            _ => {
                let (mut s, p) = consume_string(peekable_parameters, true);
                s = format!("{}{}", &(*current_char as char).to_string(), s);
                peekable_parameters = p;
                parameters.push(s);
            }
        }
    }

    parameters
}

pub fn consume_string(
    mut peekable_raw: std::iter::Peekable<std::slice::Iter<u8>>,
    stop_on_space: bool,
) -> (String, std::iter::Peekable<std::slice::Iter<u8>>) {
    let mut string = String::from("");
    while peekable_raw.peek().is_some() && (**peekable_raw.peek().unwrap() as char) != ':' {
        let current_char = peekable_raw.next().unwrap();
        if stop_on_space {
            match *current_char as char {
                ' '|'\r' => {
                    break;
                }
                _ => {
                    string += &(*current_char as char).to_string();
                }
            }
        } else {
            match *current_char as char {
                '\r' => {
                    break;
                }
                _ => {
                    string += &(*current_char as char).to_string();
                }
            }
        }
    }
    (string, peekable_raw)
}
