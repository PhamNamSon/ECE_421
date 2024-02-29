struct Player {
    id: i32,
    first_name: String,
    last_name: String,
}


fn main(){
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_player_name_type() {
        let player = Player {
            id: 1,
            first_name: "Nam Son".to_string(),
            last_name: "Pham".to_string(),
        };
        assert_that!(player.first_name, is(type_of::<String>()));
        assert_that!(player.last_name, is(type_of::<String>()));
    }

    #[test]
    fn test_two_plaers_with_same_id() {
        let player1 = Player {
            id: 1,
            first_name: "Nam Son".to_string(),
            last_name: "Pham".to_string(),
        };
        let player2 = Player {
            id: 1,
            first_name: "Nam Son".to_string(),
            last_name: "Pham".to_string(),
        };
        assert_that!(player1.id, is(equal_to(player2.id)));
        assert_that!(player2.first_name, is(equal_to(player1.first_name)));
        assert_that!(player2.last_name, is(equal_to(player1.last_name)));
    }
}