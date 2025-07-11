// 2402. Meeting Rooms III

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Room {
  room_id: u8,
}

impl Ord for Room {
  fn cmp(&self, other: &Self) -> Ordering {
    other.room_id.cmp(&self.room_id)
  }
}

impl PartialOrd for Room {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Meeting {
  end_time: u32,
  room_id: u8,
}

impl Ord for Meeting {
  fn cmp(&self, other: &Self) -> Ordering {
    other.end_time.cmp(&self.end_time).then_with(|| other.room_id.cmp(&self.room_id))
  }
}

impl PartialOrd for Meeting {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

struct Solution;

impl Solution {
  pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings;
    meetings.sort_unstable_by_key(|m| m[0]);

    let mut available_rooms: BinaryHeap<Room> = BinaryHeap::new();
    for i in 0..n {
      available_rooms.push(Room { room_id: i as u8 });
    }

    let mut ongoing_meetings: BinaryHeap<Meeting> = BinaryHeap::new();
    let mut room_usage_counts = vec![0; n as usize];

    for meeting in meetings.iter() {
      let (start, end) = (meeting[0] as u32, meeting[1] as u32);
      let duration = end - start;

      while let Some(ongoing) = ongoing_meetings.peek() {
        if ongoing.end_time <= start {
          let ongoing = ongoing_meetings.pop().unwrap();
          available_rooms.push(Room { room_id: ongoing.room_id });
        } else {
          break;
        }
      }

      if let Some(available_room) = available_rooms.pop() {
        let room_id = available_room.room_id;
        room_usage_counts[room_id as usize] += 1;

        ongoing_meetings.push(Meeting { end_time: start + duration, room_id });
      } else {
        let earliest = ongoing_meetings.pop().unwrap();
        let room_id = earliest.room_id;
        room_usage_counts[room_id as usize] += 1;
        ongoing_meetings.push(Meeting { end_time: earliest.end_time + duration, room_id });
      }
    }

    let max_usage = *room_usage_counts.iter().max().unwrap();
    room_usage_counts.iter().position(|&x| x == max_usage).unwrap() as i32
  }
}

#[test]
fn test() {
  let test_cases = [
    (2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]], 0),
    (3, vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]], 1),
  ];

  for (n, meetings, expected) in test_cases {
    assert_eq!(Solution::most_booked(n, meetings), expected);
  }
}
