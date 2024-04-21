#[cfg(test)]
mod tests;

mod path {}

pub mod stash {
	use std::fs;
	use std::io::ErrorKind;
	use std::path::{Path, PathBuf};

	use crate::item_stash::segment::StashSegment;

	#[derive(Debug)]
	pub struct ItemStash {
		stash_path: PathBuf,
		segment_0: StashSegment,
	}

	impl ItemStash {
		pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
			let stash_path = path.as_ref().to_path_buf();
			let segment_0 = StashSegment::open(&segments_path(path), 0)?;
			let stash = Self { stash_path, segment_0 };
			Ok(stash)
		}
		pub fn create(path: impl AsRef<Path>) -> std::io::Result<()> {
			let stash_dir = path.as_ref();
			if stash_dir.exists() {
				return Err(std::io::Error::from(ErrorKind::AlreadyExists));
			}
			fs::create_dir(stash_dir)?;
			fs::create_dir(segments_path(stash_dir))?;
			StashSegment::create(&segments_path(stash_dir), 0)?;
			Ok(())
		}
	}

	fn segments_path(stash_path: impl AsRef<Path>) -> PathBuf {
		let path = stash_path.as_ref().join("segments");
		path
	}
}

pub mod segment {
	use std::fs::{File, OpenOptions};
	use std::os::unix::fs::OpenOptionsExt;
	use std::path::{Path, PathBuf};

	#[derive(Debug)]
	pub struct StashSegment {
		file_path: PathBuf,
		file: File,
	}

	impl StashSegment {
		pub fn open(segments_path: impl AsRef<Path>, index: u32) -> std::io::Result<Self> {
			let file_path = segment_path(segments_path, index);
			let file = OpenOptions::new().read(true).append(true).open(&file_path)?;
			let segment = Self { file_path, file };
			Ok(segment)
		}
		pub fn create(segments_path: impl AsRef<Path>, index: u32) -> std::io::Result<()> {
			let segment_path = segment_path(segments_path, index);
			OpenOptions::new()
				.write(true)
				.create(true)
				.mode(0o600)
				.open(&segment_path)?;
			Ok(())
		}
	}

	fn segment_path(stash_segments_dir: impl AsRef<Path>, index: u32) -> PathBuf {
		let segment_dir = stash_segments_dir.as_ref().join(&format!("segment-{}", index));
		segment_dir
	}
}