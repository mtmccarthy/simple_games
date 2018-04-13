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

impl Tile {
    pub fn update_tile(&mut self, token: PlayerToken) -> Result<(), &'static str> {
        if self.token == token {
            return Err("Cannot update tile with the same token");
        }
        else {
            self.token = token;
            return Ok(())
        }
    }
}



#[cfg(test)]
mod tests {
    use data_structures::Tile::*;
    use data_structures::PlayerToken::PlayerToken;
    #[test]
    fn test_update_tile(){
        let mut tile = Tile{token: PlayerToken::OToken};
        assert_eq! (Err("Cannot update tile with the same token"),
                    tile.update_tile(PlayerToken::OToken));
        let updated_tile = Tile {token: PlayerToken::XToken};
        assert_eq!(Ok(()), tile.update_tile(PlayerToken::XToken));
        assert_eq!(PlayerToken::XToken, tile.token);
    }
}