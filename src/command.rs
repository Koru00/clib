
pub trait Command {
    fn name(&self) -> &'static str;
    fn syntax(&self) -> &'static str { "" }
    fn execute(&self, args: &[String]);
}
