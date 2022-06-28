pub trait Database {
    fn select(&self) -> String;
}

pub trait HaveDatabase {
    type Database: Database;
    fn database(&self) -> &Self::Database;
}

pub trait DatabaseImpl {}

impl<T: DatabaseImpl> Database for T {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub trait UseCase {
    fn run(&self);
}

pub trait HaveUseCase {
    type UseCase: UseCase;
    fn use_case(&self) -> &Self::UseCase;
}

pub trait UseCaseImpl: HaveDatabase {}

impl<T: UseCaseImpl> UseCase for T {
    fn run(&self) {
        println!("selected {}", self.database().select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub trait HaveHandler {
    type Handler: Handler;
    fn handler(&self) -> &Self::Handler;
}

pub trait HandlerImpl: HaveUseCase {}

impl<T: HandlerImpl> Handler for T {
    fn handle(&self) {
        self.use_case().run();
    }
}

pub struct Container {}

pub fn new_container() -> Container {
    Container {}
}

impl DatabaseImpl for Container {}
impl UseCaseImpl for Container {}
impl HandlerImpl for Container {}
impl HaveDatabase for Container {
    type Database = Self;
    fn database(&self) -> &Self::Database {
        self
    }
}
impl HaveUseCase for Container {
    type UseCase = Self;
    fn use_case(&self) -> &Self::UseCase {
        self
    }
}
impl HaveHandler for Container {
    type Handler = Self;
    fn handler(&self) -> &Self::Handler {
        self
    }
}
