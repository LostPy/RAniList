
pub let ANILIST_URL : String = "https://graphql.anilist.co";


pub mod media;
pub mod animation;
pub mod manga;
pub mod studio;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
