use djanco::*;
use djanco::log::*;
use djanco::utils::*;
use clap::Clap;

use what_constitutes_software_query;

const PROJECT_NAME: &'static str = "what_constitutes_software_query";

pub fn main() {

    let options = CommandLineOptions::parse();
    let log = Log::new(options.verbosity);
    let dataset = options.dataset_path_as_str();
    let cache = options.cache_path_as_str();

    let repository = if let Some(repository) = options.repository.as_ref() {
        Some(create_project_archive(PROJECT_NAME, repository.as_str()))
    } else {
        None
    };

    macro_rules! execute_query {
        ($database:expr, $method:path) => {
            timed_query!($method[&$database, &log, &options.output_path]);
        }
    }

    macro_rules! prepare_database {
        ($savepoint:expr, $stores:expr) => {
            Djanco::from_spec(dataset, cache, $savepoint, $stores, log.clone())
                .expect("Error initializing Djanco!");
        }
    }

    //let database = prepare_database!(1619827200 /* = May 2021*/, stores!(Generic));
    let database = prepare_database!(1606780800 /* = Dec 2020*/, stores!(Generic));
    execute_query!(database, what_constitutes_software_query::sample_stars);
    execute_query!(database, what_constitutes_software_query::sample_all);
    execute_query!(database, what_constitutes_software_query::sample_100loc_7d_10c);
    execute_query!(database, what_constitutes_software_query::sample_1000loc_180d_100c);


    if options.repository.is_some() && !options.do_not_archive_results {
        add_results(PROJECT_NAME, &repository.unwrap(), &options.output_path, options.size_limit);
    }
}
