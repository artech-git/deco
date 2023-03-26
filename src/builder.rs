

pub trait Signature {
    type Output;

    fn component(&self) -> Self::Output;
    fn access_scope(&self) -> Self::Output;
    fn module(&self, module: impl AsRef<str>) -> Self::Output;
    fn build(&self, ) -> Self::Output;
    fn set_component(&self, component: impl Operation) -> Self::Output;

}


pub trait Operation {
    type Output; 
    
    fn decorate(&self, Macro: &str) -> Self::Output; 
    fn match_function(&self, sign: Sign) -> Self::Output;
    fn match_module(&self, ) -> Self::Output;
} 