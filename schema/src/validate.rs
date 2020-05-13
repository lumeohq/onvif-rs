pub trait Validate {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
