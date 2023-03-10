use git2::{Commit, Oid, Repository};

/// All commit related stuff.
pub mod commit;
/// Repository management.
pub mod repository;

// Other stuff.
pub mod merge;
pub mod pr;
pub mod push;

/// Wrap a ```git2::Commit<'static>``` as a ```RawCommit```.
/// This is done so that we can distinguish between a ```Commit``` meant
/// to be stored in the database and a ```RawCommit``` that is used for git operations.
pub type RawCommit<'rawcommit> = Commit<'rawcommit>;

/// Wrap a ```git2::Repository``` as a ```RawRepository```.
pub type RawRepository = Repository;
