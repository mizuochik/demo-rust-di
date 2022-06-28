pub trait Database {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {}

pub trait UseCase {
    type Database: Database;

    fn database(&self) -> &Self::Database;

    fn run(&self) {
        println!("selected {}", self.database().select());
    }
}

pub struct UseCaseImpl {
    database: DatabaseImpl,
}

impl UseCase for UseCaseImpl {
    type Database = DatabaseImpl;
    fn database(&self) -> &Self::Database {
        &self.database
    }
}

pub trait Handler {
    type UseCase: UseCase;

    fn use_case(&self) -> &Self::UseCase;

    fn handle(&self) {
        self.use_case().run();
    }
}

pub struct HandlerImpl {
    use_case: UseCaseImpl,
}

impl Handler for HandlerImpl {
    type UseCase = UseCaseImpl;

    fn use_case(&self) -> &Self::UseCase {
        &self.use_case
    }
}

pub struct Container {}

pub fn new_container() -> Container {
    Container {}
}

impl Container {
    pub fn handler(&self) -> HandlerImpl {
        HandlerImpl {
            use_case: self.use_case(),
        }
    }

    pub fn use_case(&self) -> UseCaseImpl {
        UseCaseImpl {
            database: self.database(),
        }
    }

    pub fn database(&self) -> DatabaseImpl {
        DatabaseImpl {}
    }
}
