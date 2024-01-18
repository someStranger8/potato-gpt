use tts_rust::{ GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
    };
    narrator.speak("Starting test?");
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..9 {
        narrator.volume += 1.0;
        let to_speak: String = String::from("Loop ") + &narrator.volume.to_string();
        narrator.speak(&to_speak);
        std::thread::sleep(ms);
    }
}
