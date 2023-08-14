

#[derive(Debug)]
pub enum Commands {

    GETVER = 1,

    GETCPM = 2,

    GETVOLT = 5
}

/// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl std::fmt::Display for Commands {
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}