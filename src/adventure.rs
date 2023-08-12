use std::collections::VecDeque;

use crate::int_program::ProgramASCII;

fn add_command_to_program(program: &mut ProgramASCII, command:&str, item: &str)  {
    let s  = format!("{}{}",command, item);
    let mut str_chars:VecDeque<char> = s.trim_end().chars().collect();
    str_chars.push_back('\n');
    program.str_chars.extend(str_chars);
}

pub fn advanture(commands: Vec<String>)-> Vec<String> {
    let mut program_acsii = ProgramASCII::new("".to_string());
    
    for command in commands {
        add_command_to_program(&mut program_acsii, "", &command);    
    }

    let mut msg = "".to_string();
    let mut text_help = vec![];
    while let Some(output) = program_acsii.run_program(0) {
        let c = output as u8 as char;
        if c == '\n' {
            if !msg.is_empty() {
                text_help.push(msg.clone());
            } 
            if msg.contains("take the infinite loop") {
                break
            }
            if msg.contains("giant electromagnet is stuck to you") {
                break
            }
            msg.clear();
        } else {
            msg.push(c);
        }
    }
    let text = text_help.iter().cloned().enumerate().filter(|(_, txt)| *txt == "Command?".to_string()).collect::<Vec<_>>();
    if text.len() < 2 {
        return text_help;
    }
    let mut last_msg = text_help[text[text.len()-2].0+1..=text[text.len()-1].0].to_vec();

    let game_ends = [
        "take the infinite loop",
        "take the escape pod",
        "giant electromagnet is stuck to you",
        "take the photons",
        "take the molten lava",
        "You may proceed",
    ];

    let end_condition = game_ends.iter().any(|msg| {
        text_help.join("").contains(&msg.to_string())
    });

    if end_condition {
        last_msg = text_help[text[text.len()-1].0+1..].to_vec();
        last_msg.push("Try Again!".to_string());
        last_msg
    } else {
        last_msg
    }
}