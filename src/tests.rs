#[cfg(test)]
use crate::functions::*;

#[tokio::test]
async fn test_queue() {
    use serenity::all::UserId;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut queue = queue.lock().await;
    let player1_tank = create_player(UserId::new(1), Roles::Tank);
    let player1_healer = create_player(UserId::new(1), Roles::Healer);
    let player2_healer = create_player(UserId::new(2), Roles::Healer);
    let player2_dps = create_player(UserId::new(2), Roles::Dps);
    let player2_tank = create_player(UserId::new(2), Roles::Tank);
    let player3_tank = create_player(UserId::new(3), Roles::Tank);
    let player3_healer = create_player(UserId::new(3), Roles::Healer);
    let player4_dps = create_player(UserId::new(4), Roles::Dps);
    let player5_tank = create_player(UserId::new(5), Roles::Tank);
    let player6_dps = create_player(UserId::new(6), Roles::Dps);
    let players = vec!["6", "4", "1", "2", "3"];

    queue.push_back(player1_tank.clone());
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player1_healer.clone());
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player2_healer);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player2_dps);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player2_tank);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player3_tank);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player3_healer);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player4_dps);
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player5_tank.clone());
    assert!(check_group_found(&mut queue).is_none());
    queue.push_back(player6_dps.clone());
    let text = check_group_found(&mut queue).unwrap();
    println!("{text}");
    for player in players {
        assert!(text.contains(player));
    }
    assert!(queue.contains(&player5_tank));
    assert!(!queue.contains(&player1_tank));
    assert!(!queue.contains(&player1_healer));
}
