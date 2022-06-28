pub trait Database {
    fn select(&self) -> String;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub trait UseCase {
    fn run(&self);
}

pub struct UseCaseImpl<D: Database> {
    database: D,
}

impl<D: Database> UseCase for UseCaseImpl<D> {
    fn run(&self) {
        println!("selected {}", self.database.select());
    }
}

pub struct Handler<U: UseCase> {
    use_case: U,
}

impl<U: UseCase> Handler<U> {
    pub fn handle(&self) {
        self.use_case.run();
    }
}

pub struct Container {}

pub fn new_container() -> Container {
    Container {}
}

impl Container {
    pub fn database(&self) -> DatabaseImpl {
        DatabaseImpl {}
    }

    pub fn use_case(&self) -> UseCaseImpl<DatabaseImpl> {
        UseCaseImpl {
            database: self.database(),
        }
    }

    pub fn handler(&self) -> Handler<UseCaseImpl<DatabaseImpl>> {
        Handler {
            use_case: self.use_case(),
        }
    }
}
