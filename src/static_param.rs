use std::sync::Arc;

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

pub struct UseCaseImpl<D> {
    database: Arc<D>,
}

impl<D: Database> UseCase for UseCaseImpl<D> {
    fn run(&self) {
        println!("selected {}", self.database.select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub struct HandlerImpl<U> {
    use_case: Arc<U>,
}

impl<U: UseCase> Handler for HandlerImpl<U> {
    fn handle(&self) {
        self.use_case.run();
    }
}

pub struct Container {}

pub fn new_container() -> Container {
    Container {}
}

impl Container {
    pub fn database(&self) -> Arc<DatabaseImpl> {
        Arc::new(DatabaseImpl {})
    }

    pub fn use_case(&self) -> Arc<impl UseCase> {
        Arc::new(UseCaseImpl {
            database: self.database(),
        })
    }

    pub fn handler(&self) -> Arc<impl Handler> {
        Arc::new(HandlerImpl {
            use_case: self.use_case(),
        })
    }
}
