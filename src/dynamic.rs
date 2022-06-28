use std::sync::Arc;

pub trait Database {
    fn select(&self) -> String;
}

pub struct DBImpl {}

impl Database for DBImpl {
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

pub struct Container {
    database_cache: Option<Arc<dyn Database>>,
}

pub fn new_container() -> Container {
    Container {
        database_cache: None,
    }
}

impl Container {
    pub fn handler(&mut self) -> Arc<dyn Handler> {
        Arc::new(HandlerImpl {
            use_case: self.use_case(),
        })
    }

    pub fn use_case(&mut self) -> Arc<dyn UseCase> {
        Arc::new(UseCaseImpl {
            database: self.database(),
        })
    }

    pub fn database(&mut self) -> Arc<dyn Database> {
        match &self.database_cache {
            None => {
                self.database_cache = Some(Arc::new(DBImpl {}));
                self.database()
            }
            Some(db) => db.clone(),
        }
    }
}
