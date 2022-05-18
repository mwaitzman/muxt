#![allow(unused_imports)]
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, thread};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use lofty::{read_from, Accessor, read_from_path, Tag, AudioFile};
use std::path::Path;

slint::include_modules!();


fn main() {
    println!("Starting the program...");


    //CAUTION: stream must not be dropped or the program will panic!
    //TODO: allow changing the OutputStreamHandle (i.e. the device output is going through)
    let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();



    let music_player_gui = MusicPlayerGUI::new();
    let cloner_sink = Arc::new(Sink::try_new(&output_stream_handle).unwrap());


    //play/pause GUI handler code
    //TODO: revamp to actually use the new sinks for new songs or whatever
    let sink = Arc::clone(&cloner_sink);
    music_player_gui.on_toggle_pause(move || {
        if sink.is_paused() {
            sink.play();
        }
        else {
            sink.pause();
        }
    });


    //function to be called in order to play a new song from the given file
    let play_new_song = |gui: MusicPlayerGUI, song_location, player: &Arc<Sink>| {

        //update GUI song info
        let tagged_file = read_from_path(&song_location, true).expect(format!("failed to read file from path! File: {:?}", &song_location).as_str());
        let tag = tagged_file.primary_tag().unwrap_or_else(|| {
            eprintln!("no primary tag found for tagged_file! Attempting to get first tag instead");
            tagged_file.first_tag().unwrap()
        });
    gui.set_CurSongArtist(tag.artist().unwrap().into());
    gui.set_CurSongTitle(tag.title().unwrap().into());
    //TODO: actually handle the absence of specific tags properties and send <appropriately to display in the GUI
    // music_player_gui.set_CurSongGenre(tag.genre().unwrap().into());
    // music_player_gui.set_CurSongAlbum(tag.album().unwrap().into());

    //TODO: actually display the length nicely
    let song_length = tagged_file.properties().duration();
    gui.set_TotalLength(song_length.as_secs().to_string().into());



    //finally, load the song into the sink over the old one
    let file = BufReader::new(File::open(song_location).unwrap());
    let src = Decoder::new(file).unwrap();
    //NOTE: this approach doesn't allow queues *in the sink* - TODO: use external list structure for that or something?
    // player.stop();
    player.append(src);
    };




    let music_player_gui_clone = music_player_gui.clone_strong();
    let path = Path::new("/mnt/win10/Users/kon-boot/Music/SoundCloud_Favorites/Retrograde & Xomu - Valhalla.mp3");
    let sink = Arc::clone(&cloner_sink);
    println!("{}|{}|{}", sink.is_paused(), sink.volume(), sink.len());
    play_new_song(music_player_gui_clone, path, &sink);
    println!("{}|{}|{}", sink.is_paused(), sink.volume(), sink.len());



    music_player_gui.run();
    sink.play();
}