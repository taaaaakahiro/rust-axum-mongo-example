use rust_axum_adapter::modules::{RepositoriesModule, RepositoriesModuleExt};
use rust_axum_adapter::persistence::mongodb::Db;
use rust_axum_adapter::repository::health_check::HealthCheckRepository;
use rust_axum_app::usecase::health_check::HealthCheckUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new(db));

        Self {
            health_check_use_case,
        }
    }
}
