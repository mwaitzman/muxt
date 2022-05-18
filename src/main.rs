#![allow(unused_imports)]
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, thread};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use lofty::{read_from, Accessor, read_from_path, Tag, AudioFile};
use std::path::Path;
use parking_lot::RwLock;


slint::include_modules!();


fn main() {
    println!("Starting the program...");


    //CAUTION: stream must not be dropped or the program will panic!
    //TODO: allow changing the OutputStreamHandle (i.e. the device output is going through)
    //TODO: display the output+device info
    let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();



    let music_player_gui = MusicPlayerGUI::new();
    let cloner_sink = Arc::new(RwLock::new(Sink::try_new(&output_stream_handle).unwrap()));


    //play/pause GUI handler code
    //TODO: revamp to actually use the new sinks for new songs or whatever
    let sink = Arc::clone(&cloner_sink);
    music_player_gui.on_toggle_pause(move || {
        let sink = sink.read();
        if sink.is_paused() {
            sink.play();
        }
        else {
            sink.pause();
        }
    });


    //function to be called in order to stop the current song and play a new one from the given file
    let fast_forward_to_new_song = |gui: MusicPlayerGUI, song_location, sink: &Arc<RwLock<Sink>>| {

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



    // finally, discard the old song and play the new one
    let file = BufReader::new(File::open(song_location).unwrap());
    let src = Decoder::new(file).unwrap();

    let old_sink = sink.read();

    let is_paused = old_sink.is_paused();
    sink.read().stop();
    drop(old_sink);
    let new_sink = Sink::try_new(&output_stream_handle).unwrap();
    
    // keep playback paused
    //NOTE: intentionally done before the new song is appended so there's no chance the new song plays a frame or two before the pause call is executed
    if is_paused {
        new_sink.pause();
    }

    new_sink.append(src);

    *sink.write() = new_sink;
    };




    let music_player_gui_clone = music_player_gui.clone_strong();
    let path = Path::new("/mnt/win10/Users/kon-boot/Music/SoundCloud_Favorites/Retrograde & Xomu - Valhalla.mp3");
    let sink = Arc::clone(&cloner_sink);
    fast_forward_to_new_song(music_player_gui_clone, path, &sink);
    drop(sink);


    music_player_gui.run();

    println!("end of the main function");
}