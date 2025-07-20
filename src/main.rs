use std::env;         // for command-line arguments
use std::fs::File;     use std::io::BufReader;
// to open the file
use std::path::Path;   // for handling paths
use walkdir::WalkDir;  // for recursive directory walking
use rodio;             // for audio playback

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_name = &args[1];
    
    let found_path = find_audio_file(target_name);

    if let Some(path) = found_path {
        play_audio_file(&path);
    } else {
            println!("No audio file by this name found")
    }

}

fn is_supported_file(path: &Path) -> bool {
    let supported_extensions = ["mp3", "wav", "flac", "mp4"];

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        supported_extensions.contains(&ext.to_ascii_lowercase().as_str())
    } else {
        false
    }
}

fn find_audio_file(target_name: &str) -> Option<std::path::PathBuf> {
    for entry in WalkDir::new(".") {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_file() && is_supported_file(path) {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.eq_ignore_ascii_case(target_name) {
                    return Some(path.to_path_buf())
                }
            }
        }
    }
    None
}

fn play_audio_file(path: &Path) {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    let file = BufReader::new(File::open(path).unwrap());

    let sink = rodio::play(stream_handle.mixer(), file).unwrap();

    sink.sleep_until_end();
}


#[test]
fn test_audio_file_filter() {
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && is_supported_file(path) {
            println!("supported file: {}", path.display());
        }
    }
}

