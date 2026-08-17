use desktop::solver_config::{CUBE_MOVES, FILENAME};
use solver_2x2::solver;

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")).init();
    let (table, complete) = solver::build_move_table(CUBE_MOVES);
    if complete {
        log::info!("Table is complete");
    } else {
        log::warn!("Table is not complete");
    }
    let w = std::fs::File::options()
        .write(true)
        .create(true)
        .open(FILENAME)?;
    table.save(w)
}
