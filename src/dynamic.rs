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

pub struct UseCaseImpl {
    database: Arc<dyn Database>,
}

impl UseCase for UseCaseImpl {
    fn run(&self) {
        let msg = self.database.select();
        println!("selected {}", msg);
    }
}

pub trait Handler {
    fn handle(&self);
}

pub struct HandlerImpl {
    use_case: Arc<dyn UseCase>,
}

impl Handler for HandlerImpl {
    fn handle(&self) {
        self.use_case.run();
    }
}

pub struct DI {
    pub database: Arc<dyn Database>,
    pub use_case: Arc<dyn UseCase>,
    pub handler: Arc<dyn Handler>,
}

pub fn new_di() -> DI {
    let db = Arc::new(DatabaseImpl {});
    let uc = Arc::new(UseCaseImpl {
        database: db.clone(),
    });
    let h = Arc::new(HandlerImpl {
        use_case: uc.clone(),
    });
    DI {
        database: db.clone(),
        use_case: uc.clone(),
        handler: h.clone(),
    }
}
