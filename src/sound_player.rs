use gtk::prelude::*;

pub struct SoundPlayer {
    correct: gtk::MediaFile,
    wrong: gtk::MediaFile,
    settings: gio::Settings,
}

fn media_file_for_resource(resource_path: &str) -> gtk::MediaFile {
    let bytes = gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE)
        .expect("sound resource not found");
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    gtk::MediaFile::for_input_stream(&stream)
}

impl Default for SoundPlayer {
    fn default() -> Self {
        let prefix = "/io/github/nacho/mundi/sounds";
        Self {
            correct: media_file_for_resource(&format!("{prefix}/correct.oga")),
            wrong: media_file_for_resource(&format!("{prefix}/wrong.oga")),
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
