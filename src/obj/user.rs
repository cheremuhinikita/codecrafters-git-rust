pub struct User {
    pub name: String,
    pub email: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::from("Nikita Cheremuhin"),
            email: String::from("nikitadf68@gmail.com"),
        }
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}
