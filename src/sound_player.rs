use gtk::prelude::*;
use std::path::PathBuf;

pub struct SoundPlayer {
    correct: gtk::MediaFile,
    wrong: gtk::MediaFile,
    settings: gio::Settings,
}

impl Default for SoundPlayer {
    fn default() -> Self {
        let sounds_dir = PathBuf::from(crate::config::SYSTEM_DATADIR)
            .join("sounds")
            .join("freedesktop")
            .join("stereo");
        Self {
            correct: gtk::MediaFile::for_filename(sounds_dir.join("complete.oga")),
            wrong: gtk::MediaFile::for_filename(sounds_dir.join("dialog-error.oga")),
            settings: gio::Settings::new("io.github.nacho.mundi"),
        }
    }
}

impl SoundPlayer {
    pub fn play_correct(&self) {
        self.play(&self.correct);
    }

    pub fn play_wrong(&self) {
        self.play(&self.wrong);
    }

    fn play(&self, media: &gtk::MediaFile) {
        if self.settings.boolean("sound-effects") {
            media.seek(0);
            media.play();
        }
    }
}
