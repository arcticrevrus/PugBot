#[cfg(test)]
mod tests {
    use crate::functions::*;
    use serenity::all::UserId;
    use std::{collections::VecDeque, sync::Arc};
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_queue() {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut queue = queue.lock().await;
        let player1 = create_player(UserId::new(1), Roles::Tank);
        let player2 = create_player(UserId::new(2), Roles::Healer);
        let player3 = create_player(UserId::new(3), Roles::Tank);
        let player1_dps = create_player(UserId::new(1), Roles::Dps);
        let player4 = create_player(UserId::new(4), Roles::Dps);
        let player5 = create_player(UserId::new(5), Roles::Healer);
        let player6 = create_player(UserId::new(6), Roles::Dps);
        queue.push_back(player1);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player2);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player3);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player1_dps);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player4);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player5);
        assert!(check_group_found(&mut queue).is_none());
        queue.push_back(player6);
        let text = check_group_found(&mut queue).unwrap();
        let players = vec!["6", "4", "1", "2", "5"];
        println!("{text}");
        for player in players {
            assert!(text.contains(player))
        }
    }
}
