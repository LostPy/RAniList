
// Enumerations of media formats
pub enum MediaFormat {
    TV = "TV",
    TVShort = "TV_SHORT",
    Movie = "MOVIE",
    Special = "SPECIAL",
    OVA = "OVA",
    ONA = "ONA",
    Music = "Music",
    Manga = "MANGA",
    Novel = "Novel",
    OneShot = "ONE_SHOT",
}


// Enumerations of media status
pub enum MediaStatus {
    NotYetReleasing = "NOT_YET_RELEASING",  // To be released at a later date
    Releasing = "RELEASING",  // Currently releasing
    Finished = "FINISHED" ,  // Has completed and is no longer being released
    Cancelled = "CANCELLED",  // Ended before the work could be finished
    Hiatus = "HIATUS",  // Version 2 only. Is currently paused from releasing and will resume at a later date
}


// Enumerations of media season (period of release)
pub enum MediaSeason {
    Winter = "WINTER", // December to February
    Spring = "Spring",  // March to May
    Summer = "SUMMER", // June to August
    Fall = "FALL",  // September to November
}


// Enumerations of media source
pub enum MediaSource {
    Original = "ORIGINAL",
    Manga = "MANGA",
    LightNovel = "LIGHT_NOVEL",
    VisualNovel = "VISUAL_NOVEL",
    VideoGame = "VIDEO_GAME",
    Other = "OTHER",
    Novel = "NOVEL",
    Doujinshi = "DOUJINSHI",
    Anime = "ANIME",
    WebNovel = "WEB_NOVEL",
    LiveAction = "LIVE_ACTION",
    Game = "GAME",
    Comic = "COMIC",
    MultimediaProject = "MULTIMEDIA_PROJECT",
    PictureBook = "PICTURE_BOOK",
}


// Structure to represent titles of a media
pub struct MediaTitle {
    romaji_ : String,
    english_ : String,
    native_ : String,
}

impl MediaTitle {
    pub fn new(romaji_ : String, english_ : String, native_ : String) -> MediaTitle {
        MediaTitle {romaji_, english_, native_}
    }

    pub fn romaji(&self) -> &String {
        self.romaji_
    }

    pub fn english(&self) -> &String {
        self.english_
    }

    pub fn native(&self) -> &String {
        self.native_
    }
}


// Structure for a MediaCoverImage
pub struct MediaCoverImage {
    extra_large_ : String,  // image url
    large_ : String,  // image url
    medium_ : String,  // image url
}

impl MediaCoverImage {
    pub fn new(extra_large_ : String, large_ : String, medium_ : String) -> MediaCoverImage {
        MediaCoverImage {extra_large_, large_, medium_}
    }

    pub fn extraLarge(&self) -> &String {
        self.extra_large_
    }

    pub fn large(&self) -> &String {
        self.large_
    }

    pub fn medium(&self) -> &String {
        self.medium_
    }
}


// Structure to represent a date. Use for "start date" and "end date" of a media
pub struct MediaDate {
    year_ : u16,
    month_ : u8,
    day_ : u8,
}

impl MediaDate {
    pub fn new(year_ : u16, month_ : u8, day_ : u8) -> MediaDate {
        MediaDate {year_, month_, day_}
    }

    pub fn year(&self) -> u16 {
        self.year_
    }

    pub fn month(&self) -> u8 {
        self.month_
    }

    pub fn day(&self) -> u8 {
        self.day_
    }
}


// Structure to group informations of a media
pub struct MediaInfos {
    id_ : u64,
    title_ : MediaTitle,
    description_ : String,
    genres : Vec<String>,
    format_ : MediaFormat,
    source_ : MediaSource,
    status_ : MediaStatus,
    synonyms : Vec<String>,
    is_adult_ : bool,
    average_score : u8,
    mean_score : u8,
    tags : Vec<String>,
    bannerImage : Option<String>,
    coverImage : MediaCoverImage,
    hashtag : Option<String>,
    origin_country : String,  // ISO 3166-1 alpha-2 country code
    season : MediaSeason,
    season_year : u16,
    start_date : MediaDate,
    end_date : MediaDate,
}

impl MediaInfos {

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &MediaTitle {
        self.title_
    }

    pub fn description(&self) -> &String {
        self.description
    }

    pub fn format(&self) -> &MediaFormat {
        self.format_
    }

    pub fn source(&self) -> &MediaSource {
        self.source_
    }

    pub fn status(&self) -> &MediaStatus {
        self.status_
    }

    pub fn is_adult(&self) -> bool {
        self.is_adult_
    }

    pub fn  get_genres(&self) -> &Vec<String> {
        self.genres
    }

    pub fn get_synonyms(&self) -> &Vec<String> {
        self.synonyms
    }

    pub fn get_average_score(&self) -> u8 {
        self.average_score
    }

    pub fn get_mean_score(&self) -> u8 {
        self.mean_score
    }

    pub fn get_tags(&self) -> &Vec<String> {
        self.tags
    }

    pub fn get_banner_image(&self) -> &Option<String> {
        self.bannerImage
    }

    pub fn get_cover_image(&self) -> &MediaCoverImage {
        self.coverImage
    }

    pub fn get_hashtag(&self) -> &Option<String> {
        self.hashtag
    }

    pub fn get_origin_country(&self) -> &String {
        self.origin_country
    }

    pub fn get_season(&self) -> &MediaSeason {
        self.season
    }

    pub fn get_season_year(&self) -> u16 {
        self.season_year
    }

    pub fn get_start_date(&self) -> &MediaDate {
        self.start_date
    }

    pub fn get_end_date(&self) -> &MediaDate {
        self.end_date
    }
}
