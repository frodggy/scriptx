use std::process::Command;

use crate::utils::{open, toggle_audio, Config};

// * Line 1:

// switch profiles
pub fn one() {
    let mut cfg = Config::load();

    cfg.next();

    cfg.save();
}

// prev workspace
pub fn two() {
    Command::new("qtile")
        .args(["cmd-obj", "-o", "screen", "-f", "prev_group"])
        .spawn()
        .expect("failed to go to prev group");
}

// next workspace
pub fn three() {
    Command::new("qtile")
        .args(["cmd-obj", "-o", "screen", "-f", "next_group"])
        .spawn()
        .expect("failed to go to next group");
}

// next window
pub fn four() {
    Command::new("qtile")
        .args(["cmd-obj", "-o", "layout", "-f", "next"])
        .spawn()
        .expect("failed to go to prev group");
}

// * Line 2:

// Open terminal
pub fn five() {
    open("kitty")
}

// Open browser
pub fn six() {
    open("brave")
}

// Open discord
pub fn seven() {
    open("discord")
}

// Open steam
pub fn eight() {
    open("steam")
}

// * Line 3:

// Brightness up
pub fn nine() {
    Command::new("xbacklight")
        .args(["-inc", "5"])
        .spawn()
        .unwrap();
}

// Brightness down
pub fn ten() {
    Command::new("xbacklight")
        .args(["-dec", "5"])
        .spawn()
        .unwrap();
}

// Volume up
pub fn eleven() {
    Command::new("amixer")
        .args(["-c", "0", "-q", "set", "Master", "2dB+"])
        .spawn()
        .unwrap();
}

// Volume down
pub fn twelve() {
    Command::new("amixer")
        .args(["-c", "0", "-q", "set", "Master", "2dB-"])
        .spawn()
        .unwrap();
}

// * Line 4:

// Run app menu
pub fn thirteen() {
    Command::new("rofi")
        .args(["-show", "drun"])
        .spawn()
        .unwrap();
}

// Show open apps
pub fn fourteen() {
    Command::new("rofi").args(["-show"]).spawn().unwrap();
}

// Open bluetooth settings
pub fn fifteen() {
    Command::new("rofi-bluetooth").spawn().unwrap();
}

// Screenshot
pub fn sixteen() {
    Command::new("maim")
        .args([
            "-s",
            "~/Pictures/Screenshots/screen_$(date +%Y-%m-%d-%T).png",
        ])
        .spawn()
        .unwrap();
}

// * Line 5:

// Play pause
pub fn seventeen() {
    Command::new("playerctl").arg("play-pause").spawn().unwrap();
}

// Next song
pub fn eighteen() {
    Command::new("playerctl").arg("next").spawn().unwrap();
}

// Prev song
pub fn nineteen() {
    Command::new("playerctl").arg("pervious").spawn().unwrap();
}

// Mute output
pub fn twenty() {
    toggle_audio("audio")
}

// * Long keys

// Open vscode
pub fn twenty_one() {
    Command::new("code").args(["-w"]).spawn().unwrap();
}

// Mute mic
pub fn twenty_two() {
    toggle_audio("mic")
}
