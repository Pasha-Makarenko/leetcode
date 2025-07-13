// 2410. Maximum Matching of Players With Trainers

struct Solution;

impl Solution {
  pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    players.sort();
    trainers.sort();

    let mut player_index = 0;
    let mut trainer_index = 0;
    let players_count = players.len();
    let trainers_count = trainers.len();

    while player_index < players_count && trainer_index < trainers_count {
      if players[player_index] <= trainers[trainer_index] {
        player_index += 1;
      }
      trainer_index += 1;
    }

    player_index as i32
  }
}

#[test]
fn test() {
  let test_cases = [(vec![4, 7, 9], vec![8, 2, 5, 8], 2), (vec![1, 1, 1], vec![10], 1)];

  for (players, trainers, expected) in test_cases {
    assert_eq!(Solution::match_players_and_trainers(players, trainers), expected);
  }
}
