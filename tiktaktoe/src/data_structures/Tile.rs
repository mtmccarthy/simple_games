use data_structures::PlayerToken::PlayerToken;

#[derive(Debug, Clone)]
pub struct Tile {
    pub token: PlayerToken
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool{
        return self.token == other.token;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}