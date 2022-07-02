use std::sync::Arc;

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
    fn database(&self) -> Arc<Self::Database>;
}

pub struct UseCaseImpl<D> {
    di: D,
}

impl<D: UseCaseDI> UseCase for UseCaseImpl<D> {
    fn run(&self) {
        println!("selected {}", self.di.database().select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub trait HandlerDI {
    type UseCase: UseCase;
    fn use_case(&self) -> Arc<Self::UseCase>;
}

pub struct HandlerImpl<D> {
    di: D,
}

impl<D: HandlerDI> Handler for HandlerImpl<D> {
    fn handle(&self) {
        self.di.use_case().run();
    }
}

pub struct DI {
}

pub fn new_di() -> Arc<DI> {
    Arc::new(DI{})
}

impl UseCaseDI for Arc<DI> {
    type Database = DatabaseImpl;
    fn database(&self) -> Arc<Self::Database> {
        Arc::new(DatabaseImpl {})
    }
}

impl HandlerDI for Arc<DI> {
    type UseCase = UseCaseImpl<Self>;
    fn use_case(&self) -> Arc<Self::UseCase> {
        Arc::new(UseCaseImpl { di: self.clone() })
    }
}

pub trait MainDI {
    type Handler : Handler;
    fn handler(&self) -> Arc<Self::Handler>;
}

impl MainDI for Arc<DI> {
    type Handler = HandlerImpl<Self>;
    fn handler(&self) -> Arc<Self::Handler> {
        Arc::new(HandlerImpl { di: self.clone() })
    }
}
