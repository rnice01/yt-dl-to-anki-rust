pub struct SubTrack {
    pub time_start: String,
    pub time_end: String,
    pub subtitles: String
}

impl SubTrack {
    pub fn set_timestamps(mut self, timestamps: (String, String)) {
        self.time_start = timestamps.0;
        self.time_end = timestamps.1;
    }
}