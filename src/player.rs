use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::thread::sleep;
use std::{fs, thread::JoinHandle};
use super_rodio::{self, Make, Player, SharedPlayer, Song};

#[derive(Clone)]
pub struct MusicPlayer {
    pub player: SharedPlayer,
}

impl MusicPlayer {
    // Create a new MusicPlayer instance
    pub fn new() -> Self {
        let player = SharedPlayer::make();
        MusicPlayer { player }
    }

    // Implement Default to satisfy clippy's `new_without_default` lint
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicPlayer {
    // Initialize the music player with shuffled songs from ./music
    pub fn init(&mut self) -> Result<()> {
        self.init_in("./music")
    }

    /// Variant of `init` that allows specifying the target directory (useful for tests)
    pub fn init_in(&mut self, dir: &str) -> Result<()> {
        // Use the bundled default music archive (if present)
        let default_zip: &[u8] = include_bytes!("../assets/music.zip");

        // Ensure music directory exists and contains songs
        let mut songs_dir = self.load_songs_from_directory(dir).context("Failed to read music directory")?;
        if songs_dir.is_empty() {
            fs::create_dir_all(dir).with_context(|| format!("Failed to create music directory: {}", dir))?;

            // unzip default songs to dir
            let cursor = std::io::Cursor::new(default_zip);
            let mut zip = zip::ZipArchive::new(cursor).context("Failed to read bundled music archive")?;
            for i in 0..zip.len() {
                let mut file = zip.by_index(i).context("Failed to access file in archive")?;
                if let Some(path_name) = file.enclosed_name()
                    && let Some(pn) = path_name.file_name() {
                        let name = pn.to_string_lossy().into_owned();
                        let outpath = PathBuf::from(dir).join(name);
                        let mut outfile = fs::File::create(&outpath).with_context(|| format!("Failed to create output file {:?}", outpath))?;
                        std::io::copy(&mut file, &mut outfile).with_context(|| format!("Failed to copy file contents to {:?}", outpath))?;
                    }
            }

            println!("No songs found in the music directory. Default songs have been extracted. Please restart the application after adding your own music files.");
            sleep(std::time::Duration::from_secs(3));

            // reload the directory after extraction
            songs_dir = self.load_songs_from_directory(dir).context("Failed to read music directory after extraction")?;
        }

        // Shuffle the song paths
        let mut rng = rand::rng();
        let mut songs: Vec<Song> = songs_dir
            .into_iter()
            .map(|path| {
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "<unknown>".into());
                Song::from(filename, path.to_string_lossy().into_owned())
            })
            .collect();
        songs.shuffle(&mut rng);

        // Add the shuffled songs to the player
        for song in songs {
            self.player.add(song);
        }

        Ok(())
    }

    // Load songs from a given directory
    pub(crate) fn load_songs_from_directory(&self, dir: &str) -> Result<Vec<PathBuf>> {
        let mut song_paths = Vec::new();
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(song_paths),
            Err(e) => return Err(e).context(format!("Failed to read directory: {}", dir)),
        };

        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ["mp3", "wav", "ogg", "flac", "aac"].contains(&ext_str.as_str()) {
                        song_paths.push(path);
                    }
                }
        }
        Ok(song_paths)
    }

    // Play music
    pub fn start_music(&self) {
        self.player.use_auto_play();
        self.player.play();
    }

    // Pause music
    pub fn stop_music(&self) {
        self.player.stop();
    }

    // Toggle music on/off
    pub fn toggle_music(&self) {
        self.player.toggle();
    }

    // Clear the player and ensure the waiting list is drained so the
    // operation is observed immediately by callers (useful for deterministic testing).
    pub fn clear(&mut self) {
        self.player.stop();
        self.player.clear();
        // Drain the waiting list to ensure any background state is synchronously
        // observed by the caller.
        let _ = self.player.waiting_list().join();
    }

    // Show the waiting list
    pub fn queue(&self) -> JoinHandle<Vec<Song>> {
        self.player.waiting_list()
    }

    // Add and play a single test song
    pub fn play_test_song(&self, filename: &str, path: &str) -> Result<()> {
        let song = Song::from(filename.into(), path.into());
        self.player.clear();
        self.player.add(song);
        self.player.use_auto_play();
        println!("Playing test song");
        println!("{:?}", self.player.waiting_list());

        self.player.play();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn load_songs_filters_by_extension() {
        let dir = tempdir().expect("tempdir");
        let file_mp3 = dir.path().join("song1.mp3");
        let mut f = File::create(&file_mp3).unwrap();
        writeln!(f, "dummy").unwrap();

        let file_txt = dir.path().join("ignore.txt");
        let mut f2 = File::create(&file_txt).unwrap();
        writeln!(f2, "dummy").unwrap();

        let player = MusicPlayer::new();
        let songs = player.load_songs_from_directory(dir.path().to_str().unwrap()).expect("read dir");
        assert_eq!(songs.len(), 1);
        assert!(songs.iter().any(|p| p.ends_with("song1.mp3")));
    }

    #[test]
    fn init_in_extracts_default_zip() {
        let dir = tempdir().expect("tempdir");
        let mut player = MusicPlayer::new();
        // Init in a fresh dir; it should extract bundled music.zip
        player.init_in(dir.path().to_str().unwrap()).expect("init_in");
        let songs = player.load_songs_from_directory(dir.path().to_str().unwrap()).expect("read dir after init");
        assert!(!songs.is_empty(), "Expected extracted songs in the directory");
    }
}
