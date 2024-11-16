use git2::{Repository, DiffOptions, DiffFormat};

fn main() -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    let index = repo.index()?;

    let head_commit = repo.head()?.peel_to_commit()?;
    let head_tree = head_commit.tree()?;

    let mut diff_options = DiffOptions::new();

    let diff = repo.diff_tree_to_index(
        Some(&head_tree),
        Some(&index),
        Some(&mut diff_options),
    )?;

    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        print!("{}", std::str::from_utf8(line.content()).unwrap());
        true
    })?;

    Ok(())
}
