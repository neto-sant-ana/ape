//! Stage the three briefings **outside** the repository, so `lab/` can be shut for the run.
//!
//! ```text
//! cargo run -p ape-succession --bin stage -- /path/outside/the/repo
//! ```
//!
//! Each briefing gets its own directory holding `instructions.md`, `questions.md` and `record/`.
//! Nothing else: no protocol, no classification, no baseline, no sight of another carving.
//!
//! **Staging outside is what makes the isolation enforceable.** With the briefings inside `lab/`,
//! shutting `lab/` would take the agent's own material with it, and leaving it open would put the
//! protocol one `cat` away. The procedure is `agents/05-reconciliation`'s: stage, `chmod 000 lab`,
//! run, `chmod 755 lab`, and record what came back before judging any of it.
//!
//! It refuses to stage into the repository, because a briefing written inside it would defeat the
//! only mechanism the isolation has.

use std::path::{Path, PathBuf};

use ape_succession::articulation::briefing;
use ape_succession::articulation::carving::{self, Carving};
use ape_succession::articulation::record::{Run, SOURCE};
use ape_succession::testimony::reconciliation;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the crate is inside the workspace")
}

fn main() -> Result<(), String> {
    let into = std::env::args()
        .nth(1)
        .ok_or("usage: stage <directory outside the repository>")?;
    let into = PathBuf::from(into);

    std::fs::create_dir_all(&into).map_err(|why| why.to_string())?;

    let into = into.canonicalize().map_err(|why| why.to_string())?;
    let root = root();

    if into.starts_with(&root) {
        return Err(format!(
            "{} is inside {} — a briefing staged in the repository cannot be isolated from it",
            into.display(),
            root.display()
        ));
    }

    let record = Run::open(&root.join(SOURCE)).map_err(|why| why.to_string())?;
    let claims = reconciliation::CLAIMS;

    for carving in Carving::ALL {
        let pages = carving::carve(&record, claims, carving);
        let brief = briefing::brief(pages, claims);
        let directory = into.join(carving.directory());

        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(directory.join("record")).map_err(|why| why.to_string())?;

        std::fs::write(directory.join("instructions.md"), &brief.instructions)
            .map_err(|why| why.to_string())?;
        std::fs::write(directory.join("questions.md"), &brief.questions)
            .map_err(|why| why.to_string())?;

        for page in &brief.pages {
            std::fs::write(
                directory.join("record").join(format!("{}.md", page.name)),
                page.rendered(),
            )
            .map_err(|why| why.to_string())?;
        }

        println!(
            "{:<16} {} pages, {} questions -> {}",
            carving.directory(),
            brief.pages.len(),
            claims.len(),
            directory.display()
        );
    }

    Ok(())
}
