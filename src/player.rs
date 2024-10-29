use rand::seq::SliceRandom;
use rand::thread_rng;
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

    // Initialize the music player with shuffled songs from ./music
    pub fn init(&mut self) {
        let songs_dir = self.load_songs_from_directory("./music");
        if songs_dir.is_empty() {
            fs::create_dir_all("./music").expect("Failed to create music directory");
            println!("No songs found in the music directory. Please add some songs.");
            sleep(std::time::Duration::from_secs(3));
        }

        // Shuffle the song paths
        let mut rng = thread_rng();
        let mut songs: Vec<Song> = songs_dir
            .iter()
            .map(|path| {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                Song::from(filename.into(), path.to_str().unwrap().into())
            })
            .collect();
        songs.shuffle(&mut rng);

        // Add the shuffled songs to the player
        for song in songs {
            self.player.add(song);
        }
    }

    // Load songs from a given directory
    fn load_songs_from_directory(&self, dir: &str) -> Vec<PathBuf> {
        let mut song_paths = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ["mp3", "wav", "ogg", "flac", "aac"].contains(&ext_str.as_str()) {
                            song_paths.push(path);
                        }
                    }
                }
            }
        }
        song_paths
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

    // Clear the player
    pub fn clear(&mut self) {
        self.player.stop();
        self.player.clear();
    }

    // Show the waiting list
    pub fn queue(&self) -> JoinHandle<Vec<Song>> {
        self.player.waiting_list()
    }

    // Add and play a single test song
    pub fn play_test_song(
        &self,
        filename: &str,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
