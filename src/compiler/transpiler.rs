use crate::utils::get_transpiler_options;
use std::sync::Arc;
use swc::config::Config;
use swc::Compiler;
use swc_common::errors::{ColorConfig, Handler};

use swc_common::{FileName, SourceMap};
pub struct Transpiler {
  pub source_map: Arc<SourceMap>,
  swc_compiler: Compiler,
}

pub struct TranspilerOptions {}

impl Transpiler {
  pub fn new() -> Self {
    let source_map = Arc::new(SourceMap::default());
    let swc_compiler = Compiler::new(source_map.clone());

    Self { source_map, swc_compiler }
  }

  pub fn transpile(&self, code: String, filename: String, is_module: bool) -> Result<String, String> {
    let globals = swc_common::Globals::new();
    swc_common::GLOBALS.set(&globals, || {
      let options_str = get_transpiler_options(is_module);
      let config: Config = serde_json::from_str(&options_str).unwrap();

      let format_source_map = self
        .source_map
        .new_source_file(FileName::Custom(filename.into()), code.into());
      let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(self.source_map.clone()));
      let options = swc::config::Options { config, ..Default::default() };

      let output = self.swc_compiler.process_js_file(format_source_map, &handler, &options);
      match output {
        Ok(output) => Ok(output.code),
        Err(error) => Err(error.to_string()),
      }
    })
  }
}
