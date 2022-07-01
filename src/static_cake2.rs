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

pub trait UseCaseDep {
    type Database: Database;
    fn database(&self) -> Arc<Self::Database>;
}

pub struct UseCaseImpl<D> {
    dep: D,
}

impl<D: UseCaseDep> UseCase for UseCaseImpl<D> {
    fn run(&self) {
        println!("selected {}", self.dep.database().select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub trait HandlerDep {
    type UseCase: UseCase;
    fn use_case(&self) -> Arc<Self::UseCase>;
}

pub struct HandlerImpl<D> {
    dep: D,
}

impl<D: HandlerDep> Handler for HandlerImpl<D> {
    fn handle(&self) {
        self.dep.use_case().run();
    }
}

pub struct Dep {}

pub fn new_dep() -> Arc<Dep> {
    Arc::new(Dep {})
}

impl UseCaseDep for Arc<Dep> {
    type Database = DatabaseImpl;
    fn database(&self) -> Arc<Self::Database> {
        Arc::new(DatabaseImpl {})
    }
}

impl HandlerDep for Arc<Dep> {
    type UseCase = UseCaseImpl<Self>;
    fn use_case(&self) -> Arc<Self::UseCase> {
        Arc::new(UseCaseImpl { dep: self.clone() })
    }
}

pub trait MainDep {
    type Handler : Handler;
    fn handler(&self) -> Arc<Self::Handler>;
}

impl MainDep for Arc<Dep> {
    type Handler = HandlerImpl<Self>;
    fn handler(&self) -> Arc<Self::Handler> {
        Arc::new(HandlerImpl { dep: self.clone() })
    }
}
