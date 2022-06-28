pub trait Database {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub trait UseCase {
    type Database: Database;

    fn database(&self) -> &Self::Database;

    fn run(&self) {
        println!("selected {}", self.database().select());
    }
}

pub trait Handler {
    type UseCase: UseCase;

    fn use_case(&self) -> &Self::UseCase;

    fn handle(&self) {
        self.use_case().run();
    }
}

pub struct Container {}

pub fn new_container() -> Container {
    Container {}
}

impl Database for Container {}

impl UseCase for Container {
    type Database = Self;
    fn database(&self) -> &Self::Database {
        &self
    }
}

impl Handler for Container {
    type UseCase = Self;
    fn use_case(&self) -> &Self::UseCase {
        &self
    }
}

impl Container {
    pub fn handler(&self) -> &Self {
        &self
    }
}
