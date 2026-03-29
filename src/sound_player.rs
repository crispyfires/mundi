use gtk::prelude::*;

pub struct SoundPlayer {
    correct: gtk::MediaFile,
    wrong: gtk::MediaFile,
}

impl Default for SoundPlayer {
    fn default() -> Self {
        Self {
            correct: gtk::MediaFile::for_resource("/io/github/nacho/mundi/sounds/correct.oga"),
            wrong: gtk::MediaFile::for_resource("/io/github/nacho/mundi/sounds/wrong.oga"),
        }
    }
}

impl SoundPlayer {
    pub fn play_correct(&self) {
        self.correct.seek(0);
        self.correct.play();
    }

    pub fn play_wrong(&self) {
        self.wrong.seek(0);
        self.wrong.play();
    }
}
