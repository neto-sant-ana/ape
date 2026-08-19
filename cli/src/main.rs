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
//!
//! Earned by: 00-reconstruction (Confirmed), 03-convergence (Confirmed), 05-coordination (Confirmed)

use std::process::ExitCode;

use ape::engine::thesis::ThesisId;
use ape::kernel::entities::{AgentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_cli::transfer;

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

const USAGE: &str = "usage: ape-cli <repository> <instance> <date>\n   or: ape-cli <repository> transfer <base> <source> <target>\n   or: ape-cli <repository> decided <party>";

/// Three questions, and they take different arguments because they need different things.
///
/// Reading the worlds needs an instance and an instant, because a world is read *of* something
/// *at* some time. A transfer needs neither and needs three identities instead: it is asked
/// about worlds rather than about a resource, and the question is not in the repository, so it
/// arrives here. Making one form carry the other's arguments unused would be dishonest about
/// which answer depends on what.
///
/// Asking whose a line is needs one identity and answers with identities, which is the shape of
/// addressing rather than of reading: what comes back is what the other two forms can be asked
/// about next.
fn run() -> Result<String, String> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let [root, rest @ ..] = arguments.as_slice() else {
        return Err(USAGE.to_owned());
    };

    let repository = Repository::open(root);

    match rest {
        [verb, party] if verb == "decided" => rendered(
            &reading::decided_by(&repository, identity(party).map(AgentId::from)?)
                .map_err(|reason| reason.to_string())?,
        ),

        [instance, date] => {
            let instance = identity(instance).map(ResourceInstanceId::from)?;
            let effective_at =
                Date::parse(date).map_err(|_| "date is not YYYY-MM-DD".to_owned())?;

            rendered(
                &reading::reconstruct(&repository, instance, &effective_at)
                    .map_err(|reason| reason.to_string())?,
            )
        }

        [verb, base, source, target] if verb == "transfer" => rendered(
            &transfer::reconstruct(
                &repository,
                identity(base).map(ThesisId::from)?,
                identity(source).map(ThesisId::from)?,
                identity(target).map(ThesisId::from)?,
            )
            .map_err(|reason| reason.to_string())?,
        ),

        _ => Err(USAGE.to_owned()),
    }
}

fn rendered<T: serde::Serialize>(answer: &T) -> Result<String, String> {
    serde_json::to_string_pretty(answer).map_err(|reason| reason.to_string())
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
