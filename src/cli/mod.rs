#[allow(dead_code)]
pub enum Command {
  Run,
  Test,
}

#[allow(dead_code)]
pub enum FileType {
  TypeScript,
  TypeScriptReact,
  JavaScript,
  Vue,
}

pub struct CliResult {
  pub extension: FileType,
  pub command: Command,
  pub file: String,
}
pub fn command_line(args: Vec<String>) -> Result<CliResult, String> {
  let start_commands: usize = 1;
  // todo: verify if is development or production
  // let environment = env::var("CARGO");

  // zox [command] [file]
  match args.len() {
    2 => {
      let file = &args[start_commands];
      let extension = match file.split('.').last() {
        Some("ts") => FileType::TypeScript,
        Some("js") => FileType::JavaScript,
        _ => return Err("Invalid file extension".to_string()),
      };
      Ok(CliResult { extension, command: Command::Run, file: file.to_string() })
    }
    3 => {
      let command = match args[start_commands].as_str() {
        "run" => Command::Run,
        _ => return Err("Invalid command".to_string()),
      };
      let file = &args[start_commands + 1];
      let extension = match file.split('.').last() {
        Some("ts") => FileType::TypeScript,
        Some("js") => FileType::JavaScript,
        _ => return Err("Invalid file extension".to_string()),
      };
      Ok(CliResult { extension, command, file: file.to_string() })
    }
    _ => Err("Too many arguments provided".to_string()),
  }
}
