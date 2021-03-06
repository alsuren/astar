use super::{astar, SearchProblem};
use std::collections::VecDeque;
use std::vec::IntoIter;

struct GridState {
    start: (i32, i32),
    end: (i32, i32)
}

fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

impl SearchProblem for GridState {
    type Node = (i32, i32);
    type Cost = i32;
    type Iter = IntoIter<((i32, i32), i32)>;

    fn start(&self) -> (i32, i32) { self.start }
    fn is_end(&self, other: &(i32, i32)) -> bool { other == &self.end }
    fn heuristic(&self, &(p_x, p_y): &(i32, i32)) -> i32 {
        let (s_x, s_y) = self.end;
        abs(s_x - p_x) + abs(s_y - p_y)
    }
    fn neighbors(&mut self, &(x, y): &(i32, i32)) -> IntoIter<((i32, i32), i32)> {
        let mut vec = vec![];
        for i in (-1 .. 1 + 1) {
            for k in (-1 .. 1 + 1) {
                if !(i == 0 && k == 0) {
                    vec.push(((x + i, y + k), 1));
                }
            }
        }
        vec.into_iter()
    }
}

fn path(start: (i32, i32), end: (i32, i32)) -> Option<VecDeque<(i32, i32)>> {
    let mut gs = GridState{ start: start, end: end };
    astar(&mut gs)
}

#[test]
fn test_iter() {
    let mut gs = GridState{ start: (0,0), end: (0,0) };
    assert!(
        gs.neighbors(&(0,0)).collect::<Vec<_>>() ==
        vec![
            ((-1, -1), 1),
            ((-1, 0), 1),
            ((-1, 1), 1),
            ((0, -1), 1),
            ((0, 1), 1),
            ((1, -1), 1),
            ((1, 0), 1),
            ((1, 1), 1)
        ])
}

#[test]
fn test_start_end() {
    let p = path((0,0), (0,0)).unwrap();
    assert!(p == vec![(0, 0)].into_iter().collect());
}

#[test]
fn test_next() {
    let p = path((0,0), (0,1)).unwrap();
    assert!(p == vec![(0,0), (0,1)].into_iter().collect());
}

#[test]
fn test_few() {
    let p = path((0,0), (0,4)).unwrap();
    assert!(p == vec![(0,0), (0,1) ,(0,2), (0,3), (0,4)].into_iter().collect());
}
