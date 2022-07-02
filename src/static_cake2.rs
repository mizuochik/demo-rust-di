use std::sync::{Arc, Weak};

pub trait Database {
    fn select(&self) -> String;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    fn select(&self) -> String {
        String::from("selected")
    }
}

pub trait UseCase {
    fn run(&self);
}

pub trait UseCaseDI {
    type Database: Database;
    fn get_database(&self) -> Arc<Self::Database>;
}

pub struct UseCaseImpl<D> {
    di: D,
}

impl<D: UseCaseDI> UseCase for UseCaseImpl<D> {
    fn run(&self) {
        println!("selected {}", self.di.get_database().select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub trait HandlerDI {
    type UseCase: UseCase;
    fn get_use_case(&self) -> Arc<Self::UseCase>;
}

pub struct HandlerImpl<D> {
    di: D,
}

impl<D: HandlerDI> Handler for HandlerImpl<D> {
    fn handle(&self) {
        self.di.get_use_case().run();
    }
}

pub struct DI {
    pub database: Arc<DatabaseImpl>,
    pub use_case: Arc<UseCaseImpl<Weak<Self>>>,
    pub handler: Arc<HandlerImpl<Weak<Self>>>
}

pub fn new_di() -> Arc<DI> {
    Arc::new_cyclic(|di| {
        DI {
            database: Arc::new(DatabaseImpl{}),
            use_case: Arc::new(UseCaseImpl { di: di.clone() }),
            handler: Arc::new(HandlerImpl { di: di.clone() }),
        }
    })
}

impl UseCaseDI for Weak<DI> {
    type Database = DatabaseImpl;
    fn get_database(&self) -> Arc<Self::Database> {
        self.upgrade().unwrap().database.clone()
    }
}

impl HandlerDI for Weak<DI> {
    type UseCase = UseCaseImpl<Self>;
    fn get_use_case(&self) -> Arc<Self::UseCase> {
        self.upgrade().unwrap().use_case.clone()
    }
}
