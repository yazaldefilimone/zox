pub enum Command {
  Run,
}

#[allow(dead_code)]
pub enum FileType {
  TypeScript,
  JavaScript,
}

pub struct CliResult {
  pub extension: FileType,
  pub command: Command,
  pub file: String,
}
pub fn command_line(args: Vec<String>) -> Result<CliResult, String> {
  let command = match args.get(1) {
    Some(command) => command.to_string(),
    None => "run".to_string(),
  };
  // let extension = match args.get(2).and_then(|s| s.rfind('.')) {
  //   Some(extension) => match extension.to_string().as_str() {
  //     "ts" => FileType::TypeScript,
  //     "js" => FileType::JavaScript,
  //     _ => {
  //       return Err(format!("Unknown file extension: {}", extension.to_string().as_str()));
  //     }
  //   },
  //   None => {
  //     return Err(format!("No file extension"));
  //   }
  // };
  let command = match command.as_str() {
    "run" => Command::Run,
    _ => {
      return Err(format!("Unknown command: {}", command));
    }
  };

  Ok(CliResult { extension: FileType::JavaScript, command, file: args.get(2).unwrap().to_string() })
}
