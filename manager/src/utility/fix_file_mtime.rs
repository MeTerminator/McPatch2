use std::ops::Deref;
use std::path::Path;

use crate::diff::abstract_file::AbstractFile;
use crate::diff::diff::Diff;
use crate::utility::unix_timestamp::UnixTimestampExt;

pub fn fix_time_mtime<N, O>(diff: &Diff<N, O>, workspace_dir: &Path) where N: AbstractFile, O: AbstractFile {
    for (n, o) in &diff.mtime_fix {
        let path = workspace_dir.join(n.path().deref());
        println!("fix mtime: {:?} ({} => {})", path, n.modified().as_unix_seconds(), o.modified().as_unix_seconds());
        
        let open = std::fs::File::options()
            .write(true)
            .read(true)
            .open(path);

        let open = match open {
            Ok(open) => open,
            Err(e) => panic!("{}: {}", n.path().deref(), e.to_string()),
        };

        open.set_times(std::fs::FileTimes::new().set_modified(o.modified())).unwrap();
    }
}