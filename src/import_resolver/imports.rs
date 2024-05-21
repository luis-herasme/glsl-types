use glsl::syntax::TranslationUnit;
use glsl::visitor::{Host, Visit, Visitor};

use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_file_imports(
  ast: &mut TranslationUnit,
  file_path: &PathBuf,
) -> HashMap<String, PathBuf> {
  let mut visitor = FileImports::new(file_path);
  ast.visit(&mut visitor);
  return visitor.imports;
}

#[derive(Clone, Debug)]
pub struct FileImports {
  base_path: PathBuf,
  imports: HashMap<String, PathBuf>,
}

impl FileImports {
  fn new(file_path: &PathBuf) -> Self {
    Self {
      base_path: file_path.parent().unwrap().to_path_buf(),
      imports: HashMap::new(),
    }
  }
}

impl Visitor for FileImports {
  fn visit_import(&mut self, import: &glsl::syntax::Import) -> Visit {
    let path = match &import.path {
      glsl::syntax::Path::Absolute(path) => PathBuf::from(path),
      glsl::syntax::Path::Relative(path) => self.base_path.join(path).canonicalize().unwrap(),
    };

    self.imports.insert(import.identifier.to_string(), path);
    Visit::Parent
  }
}
