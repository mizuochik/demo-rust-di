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

pub struct Container {
    database: Arc<DatabaseImpl>,
    use_case: Arc<UseCaseImpl<DatabaseImpl>>,
    handler: Arc<HandlerImpl<UseCaseImpl<DatabaseImpl>>>,
}

pub fn new_container() -> Container {
    let db = Arc::new(DatabaseImpl {});
    let uc = Arc::new(UseCaseImpl {
        database: db.clone(),
    });
    let h = Arc::new(HandlerImpl {
        use_case: uc.clone(),
    });
    Container {
        database: db.clone(),
        use_case: uc.clone(),
        handler: h.clone(),
    }
}

impl Container {
    pub fn database(&self) -> Arc<impl Database> {
        self.database.clone()
    }

    pub fn use_case(&self) -> Arc<impl UseCase> {
        self.use_case.clone()
    }

    pub fn handler(&self) -> Arc<impl Handler> {
        self.handler.clone()
    }
}
