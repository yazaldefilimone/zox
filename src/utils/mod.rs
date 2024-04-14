pub mod constants;
pub mod primitives;
use std::io::Read;

pub fn get_read_file(path: &str) -> String {
  let mut file = std::fs::File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

pub fn get_transpiler_options(is_module: bool) -> String {
  let module = if is_module {
    r#"
    "module": {
        "type": "es6",
        "strict": true,
        "strictMode": true,
        "lazy": false,
        "noInterop": false,
        "ignoreDynamic": true
    },
    "#
  } else {
    ""
  };
  let options = format!(
    r#"{{
      "minify": {{}},
      "sourceMaps": true,
      {}
      "jsc": {{
        "externalHelpers": {{}},
        "parser": {{
          "syntax": "typescript",
          "jsx": true,
          "tsx": true,
          "decorators": true,
          "decoratorsBeforeExport": true,
          "dynamicImport": true,
          "preserveAllComments": false
        }},
        "transform": {{
          "legacyDecorator": true,
          "decoratorMetadata": true,
          "react": {{
              "runtime": "classic",
              "useBuiltins": true,
              "refresh": true
          }}
        }},
        "target": {{
          "type": "es6",
          "loose": false,
          "spec": false,
          "decorators": true,
          "decoratorsBeforeExport": true,
          "jsx": true,
          "tsx": true,
          "dynamicImport": true,
          "useDefineForClassFields": true,
          "useDefineForFunctionFields": true,
          "useDefineForClassFields": true,
          "useDefineForFunctionFields": true,
          "useDefineForClassFields": true,
          "useDefineForFunctionFields": true
        }},
        "keepClassNames": true
      }}
    }}"#,
    module
  );
  options.to_string()
}

// !todo: improve this function, maybe use  low level bindings to check if the code is a module
pub fn get_detect_module(code: &str) -> bool {
  code.contains("import") || code.contains("export")
}
