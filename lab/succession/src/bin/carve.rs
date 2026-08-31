//! Emit the three carvings, and print what each one placed and what it cost.
//!
//! ```text
//! cargo run -p ape-succession --bin carve
//! ```
//!
//! Writes to `lab/succession/01-articulation/carvings/{a-flat,b-per-entity,c-per-decision}`, which
//! is committed: the three directories are what three agents are given, so pinning them is what lets
//! a later reader check that the agents were asked about the record this experiment describes.
//!
//! **It refuses to run outside the workspace**, because a generator that silently wrote a carving of
//! nothing would produce three empty directories and a table of zeroes.

use std::path::{Path, PathBuf};

use ape_succession::articulation::carving::{self, Carving};
use ape_succession::articulation::record::{Record, SOURCE};
use ape_succession::testimony::reconciliation;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the crate is inside the workspace")
}

fn main() -> Result<(), String> {
    let root = root();
    let record = Record::open(&root.join(SOURCE)).map_err(|why| why.to_string())?;
    let into = root.join("lab/succession/01-articulation/carvings");
    let claims = reconciliation::CLAIMS;

    println!(
        "{:<16} {:>6} {:>9} {:>8} {:>10}",
        "carving", "pages", "bytes", "placed", "overflowed"
    );

    for carving in Carving::ALL {
        let pages = carving::carve(&record, claims, carving);
        let directory = into.join(carving.directory());

        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).map_err(|why| why.to_string())?;

        for page in &pages {
            std::fs::write(directory.join(format!("{}.md", page.name)), page.rendered())
                .map_err(|why| why.to_string())?;
        }

        let at = carving::placement(&record, claims, carving);
        println!(
            "{:<16} {:>6} {:>9} {:>8} {:>10}",
            carving.directory(),
            at.pages,
            at.bytes,
            at.placed,
            at.overflowed
        );
    }

    println!(
        "\n{} claims to place, of {}",
        carving::placement(&record, claims, Carving::Flat).to_place,
        claims.len()
    );

    Ok(())
}
