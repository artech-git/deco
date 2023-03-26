
use syn::TypeBareFn;
use syn::StmtMacro;

#[derive(Debug)]
pub struct Function {
    pub module: PathBuf,
    pub functions: Vec<TypeBareFn>,
    pub decorator: Option<String>
}

impl Function {
    fn new(module: PathBuf) -> Self {
        Self {
            module: module,
            functions: vec![],
            decorator: None
        }
    }

    fn set_function(&mut self, method: TypeBareFn) {
        self.functions.push(method);
    }

    fn set_decorator(&mut self, decorator: String) {
        self.decorator = decorator
    }

    fn return_decorator(&self) -> Result<syn::StmtMacro, BackendError> {
        self.decorator.
    }
}
