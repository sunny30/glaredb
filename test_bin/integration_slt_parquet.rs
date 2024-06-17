use rayexec_execution::{datasource::DataSourceRegistry, engine::Engine};
use rayexec_parquet::ParquetDataSource;
use std::path::Path;

pub fn main() {
    let paths = rayexec_slt::find_files(Path::new("../slt/parquet")).unwrap();
    rayexec_slt::run(
        paths,
        |rt| {
            Engine::new_with_registry(
                rt,
                DataSourceRegistry::default()
                    .with_datasource("parquet", Box::new(ParquetDataSource))?,
            )
        },
        "slt_datasource_parquet",
    )
    .unwrap();
}
