//! Entry point for the `ape-cli` binary.
//!
//! Reads arguments, writes output, and delegates every decision to [`ape_cli`]. Logic that
//! accumulates here is logic no test can reach.
//!
//! Its one job is to be the fresh process an experiment terminates into. It is given a
//! repository, a resource instance and an instant, and it is given nothing else — no value
//! the original process computed reaches it except through the repository it opens.
//!
//! It prints the whole lineage rather than its tip. Which world an application ended at is a
//! smaller question than which worlds it considered, and only the second is worth a
//! repository that keeps decisions.

use std::process::ExitCode;

use ape::kernel::entities::ResourceInstanceId;
use ape::kernel::value_objects::Date;

use ape_cli::reading;
use ape_cli::repository::Repository;

fn main() -> ExitCode {
    match run() {
        Ok(rendered) => {
            println!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("{reason}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<String, String> {
    let mut arguments = std::env::args().skip(1);

    let mut next = |name: &str| {
        arguments
            .next()
            .ok_or_else(|| format!("usage: ape-cli <repository> <instance> <date>\nmissing {name}"))
    };

    let repository = Repository::open(next("repository")?);
    let instance = identity(&next("instance")?).map(ResourceInstanceId::from)?;
    let effective_at =
        Date::parse(next("date")?).map_err(|_| "date is not YYYY-MM-DD".to_owned())?;

    let lineage = reading::reconstruct(&repository, instance, &effective_at)
        .map_err(|reason| reason.to_string())?;

    serde_json::to_string_pretty(&lineage).map_err(|reason| reason.to_string())
}

/// A 32-byte identity written as hex, which is the form every APE id renders itself in.
fn identity(value: &str) -> Result<[u8; 32], String> {
    let bytes: Vec<u8> = (0..value.len())
        .step_by(2)
        .map(|at| {
            value
                .get(at..at + 2)
                .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                .ok_or_else(|| format!("{value:?} is not hex"))
        })
        .collect::<Result<_, _>>()?;

    bytes
        .try_into()
        .map_err(|_| format!("{value:?} is not a 32-byte identity"))
}
