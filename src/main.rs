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

    const NULL_TAG_VALUE_STR: &str = "unknown";

    //function to be called in order to stop the current song and play a new one from the given file
    let fast_forward_to_new_song = |gui: MusicPlayerGUI, song_location, sink: &Arc<RwLock<Sink>>| {

        //update GUI song info
        let tagged_file = read_from_path(&song_location, true).expect(format!("failed to read file from path! File: {:?}", &song_location).as_str());
        let tag = tagged_file.primary_tag().unwrap_or_else(|| {
            eprintln!("no primary tag found for tagged_file! Attempting to get first tag instead");
            tagged_file.first_tag().unwrap()
        });

        // display the current song's tag's artist
        //TODO: if not found in this tag, maybe look for other tags in the file and source from there?? Also check user-defined db when that's implemented (will override the rest if found)
            match tag.artist() {
                Some(artist_name) => {
                    gui.set_CurSongArtist(artist_name.into());
                }
                //TODO: log this, maybe
                None => {
                gui.set_CurSongArtist(NULL_TAG_VALUE_STR.into());
                }
            }

        // display the current song's tag's title
            match tag.title() {
                Some(title_name) => {
                    gui.set_CurSongTitle(title_name.into());
                }
                //TODO: log this, maybe
                None => {
                gui.set_CurSongTitle(NULL_TAG_VALUE_STR.into());
                }
            }

        // display the current song's tag's genre in the GUI
            match tag.genre() {
                Some(genre_name) => {
                    gui.set_CurSongGenre(genre_name.into());
                }
                //TODO: log this, maybe
                None => {
                gui.set_CurSongGenre(NULL_TAG_VALUE_STR.into());
                }
            }

        // display the current song's tag's album in the GUI
            match tag.album() {
                Some(album_name) => {
                    gui.set_CurSongAlbum(album_name.into());
                }
                //TODO: log this, maybe
                None => {
                gui.set_CurSongAlbum(NULL_TAG_VALUE_STR.into());
                }
            }


        //TODO: actually display the length nicely
        // get the total length of the current song from the Tag and display it in the GUI
            let song_length = tagged_file.properties().duration();
            let raw_secs = song_length.as_secs();
            
            let mut secs = raw_secs % 60;
            // because the `as_secs()` method returns only the whole seconds without rounding, we use this to round the song length to the nearest second
            if song_length.subsec_millis() >= 500 {
                secs += 1;
            }
            let secs = secs;

            let mins = raw_secs / 60;
            //NOTE: songs over an hour long will display the minutes of the song instead of hours plus the minutes mod 60. This is easy to change if deemed better to display the hours too
            // nitpick: very minor, but I dislike the wasteful allocations here
            let length_string = mins.to_string() + ":" + &secs.to_string();
            gui.set_TotalLength(length_string.into());



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