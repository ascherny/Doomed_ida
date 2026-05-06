pub struct Metadata {
    fields: Vec<(String, String)>,
}

impl Metadata {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add(&mut self, key: &str, value: impl ToString) -> &mut Self {
        self.fields.push((key.to_string(), value.to_string()));
        self
    }

    pub fn fields(&self) -> &[(String, String)] {
        &self.fields
    }
}
