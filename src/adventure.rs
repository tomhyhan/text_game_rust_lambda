use std::collections::VecDeque;

use crate::int_program::ProgramASCII;



// fn find_password() {
    // auto : "north\ntake sand\nnorth\nnorth\ntake astrolabe\nsouth\nsouth\nwest\nwest\ntake mutex\neast\neast\nsouth\neast\ntake klein bottle\neast\ntake semiconductor\nwest\nnorth\nnorth\nnorth\ntake dehydrated water\nsouth\nsouth\nsouth\nwest\nwest\nnorth\ntake shell\nsouth\nsouth\nwest\ntake ornament\nwest\nsouth\n".to_string()
    // let items = vec!["mutex", "ornament", "sand", "astrolabe", "klein bottle", "semiconductor", "dehydrated water", "shell"];
    // for item in items.iter() {
    //     add_command_to_program(&mut program_acsii, "drop ", item)
    // }
    // let command_sets = generate_powerset(items);
    // for commands in command_sets.iter() {
    //     for command in commands.iter() {
    //         add_command_to_program(&mut program_acsii, "take ", command)
    //     }    
    //     add_command_to_program(&mut program_acsii, "", "south");
    //     for command in commands.iter() {
    //         add_command_to_program(&mut program_acsii, "drop ", command)
    //     }  
    // }
// }

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
                msg.clear();
            } 
        } else {
            msg.push(c);
        }
    }
    let text = text_help.iter().cloned().enumerate().filter(|(_, txt)| *txt == "Command?".to_string()).collect::<Vec<_>>();
    if text.len() < 2 {
        return text_help;
    }
    text_help[text[text.len()-2].0+1..=text[text.len()-1].0].to_vec()
}