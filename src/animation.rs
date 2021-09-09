
use crate::media;


let QUERY_ANIME: String = "query ($id: Int, $page: Int, $perPage: Int, $search: String) {
  Page(page: $page, perPage: $perPage) {
    media(id: $id, search: $search, type: ANIME) {
      id
      title {
        romaji
        english
        native
      }
      episodes
      description
      format
      status
      duration
      genres
      tags {
        name
      }
      studios {
        nodes {
          name
        }
      }
      startDate {
        year
        month
        day
      }
      endDate {
        year
        month
        day
      }
      season
      seasonYear
      countryOfOrigin
      coverImage {
        medium
        large
        extraLarge
      }
      bannerImage
      source
      hashtag
      synonyms
      meanScore
      averageScore
      isAdult
      nextAiringEpisode {
        timeUntilAiring
        airingAt
      }
      trailer {
        id
        thumbnail
        site
      }
      staff(sort: FAVOURITES_DESC) {
        edges {
          node {
            name {
              full
            }
            id
          }
        }
      }
      characters(role: MAIN) {
        edges {
          node {
            name {
              full
            }
          }
        }
      }
    }
  }
}";


pub struct Anime {
    infos : media::MediaInfos,
    episodes_ : u8,
    duration : u32,
}

impl Anime {
    fn new(infos : media::MediaInfos, episodes_ : u8, duration : u32) -> Anime {
        Anime {infos, episodes_, duration}
    }

    pub fn by_id(id : u64) -> Anime {}

    pub fn episodes(&self) -> u8 {
        self.episodes_
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }
}


pub struct Movie {
    infos : media::MediaInfos,
    duration : u32
}

impl Movie {
    fn new(infos : media::MediaInfos, duration : u32) -> Movie {
        Movie {infos, duration}
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }
}
