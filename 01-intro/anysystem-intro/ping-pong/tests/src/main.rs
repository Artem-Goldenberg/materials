mod common;
mod tests;
mod tests_mc;

use std::env;
use std::io::Write;

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

use anysystem::test::TestSuite;

use crate::common::TestConfig;
use crate::tests::*;
use crate::tests_mc::*;

/// Ping-Pong Tests
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Path to Python file with PingClient and PingServer implementations
    #[clap(long = "impl", short)]
    impl_path: String,

    /// Test to run (optional)
    #[clap(long = "test", short)]
    test: Option<String>,

    /// Random seed used in tests
    #[clap(long, short, default_value = "123")]
    seed: u64,
}

fn main() {
    let args = Args::parse();
    Builder::new()
        .filter(Some("anysystem"), LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    append_to_python_path("../../../anysystem/python".to_string());
    env::set_var("PYTHONHASHSEED", args.seed.to_string());
    let config = TestConfig {
        impl_path: args.impl_path,
        seed: args.seed,
    };
    let mut tests = TestSuite::new();

    tests.add("RUN", test_run, config.clone());
    tests.add("RESULT", test_result, config.clone());
    tests.add("RESULT UNRELIABLE", test_result_unreliable, config.clone());
    tests.add(
        "10 RESULTS UNRELIABLE",
        test_10_results_unreliable,
        config.clone(),
    );
    tests.add("DROP PING", test_drop_ping, config.clone());
    tests.add("DROP PONG", test_drop_pong, config.clone());
    tests.add("DROP PING 2", test_drop_ping_2, config.clone());
    tests.add("DROP PONG 2", test_drop_pong_2, config.clone());
    tests.add("10 UNIQUE RESULTS", test_10_unique_results, config.clone());
    tests.add(
        "10 UNIQUE RESULTS UNRELIABLE",
        test_10_unique_results_unreliable,
        config.clone(),
    );
    tests.add(
        "MC RELIABLE NETWORK",
        test_mc_reliable_network,
        config.clone(),
    );
    tests.add(
        "MC UNRELIABLE NETWORK",
        test_mc_unreliable_network,
        config.clone(),
    );
    tests.add(
        "MC LIMITED MESSAGE DROPS",
        test_mc_limited_message_drops,
        config.clone(),
    );
    tests.add(
        "MC CONSECUTIVE MESSAGES",
        test_mc_consecutive_messages,
        config,
    );

    if args.test.is_none() {
        tests.run();
    } else {
        tests.run_test(&args.test.unwrap().to_uppercase().replace("_", " "));
    }
}

fn append_to_python_path(entry: String) {
    let path_separator = if cfg!(windows) { ";" } else { ":" };
    let current_path = env::var("PYTHONPATH").unwrap_or_default();
    let updated_path = if current_path.is_empty() {
        entry
    } else {
        format!("{current_path}{path_separator}{entry}")
    };
    env::set_var("PYTHONPATH", updated_path);
}
