fn main() {
}

#[derive(Default)]
struct Player {
    id: i32,
    first_name: String,
    last_name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn player_has_first_name(){
        let p = Player::default();
        assert_that!(p.first_name, anything());
    }

    #[test]
    fn player_has_last_name_type_string(){
        let p = Player::default();
        assert_that!(p.last_name, type_of::<String>());
    }

    #[test]
    fn player_same_id_should_have_same_first_last_name(){
        let p1 = Player{ id: 0, first_name: String::from("1"), last_name: String::from("2") };
        let p2 = Player{ id: 0, first_name: String::from("3"), last_name: String::from("4") };
        
        assert_that!(p1.id, eq(p2.id));
        assert_that!(p1.first_name, eq(p2.first_name));
        assert_that!(p1.last_name, eq(p2.last_name));
    }
}
