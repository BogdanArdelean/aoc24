use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
type Grid = Vec<Vec<char>>;

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect())
}

fn find(g: &Grid, target: char) -> Position {
    for x in 0..g.len() {
        for y in 0..g[0].len() {
            if g[x][y] == target {
                return Position { x, y };
            }
        }
    }

    panic!("Not found!");
}

fn can_go(p: Position, g: &Grid) -> bool {
    p.x < g.len() && p.y < g[0].len() && g[p.x][p.y] != '#'
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn neighbours(&self) -> [Direction; 3] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right, self.clone()],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down, self.clone()],
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

type PosDir = (Position, Direction);
#[derive(Debug, Clone)]
struct QueueEntry {
    pd: PosDir,
    cost: i64,
}

impl QueueEntry {
    fn new(pd: PosDir, cost: i64) -> Self {
        QueueEntry { pd, cost }
    }
}

impl Eq for QueueEntry {}

impl PartialEq<Self> for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
fn all_min_paths(from: Position, to: Position, g: &Grid) -> (i64, HashSet<Position>) {
    let mut viz = HashSet::new();
    let mut q = BinaryHeap::new();
    let mut paths_at = HashMap::<(i64, PosDir), HashSet<Position>>::new();
    let mut unique_positions = HashSet::new();
    let mut min_cost = i64::MAX;

    q.push(QueueEntry::new((from, Direction::Right), 0));
    paths_at.insert((0, (from, Direction::Right)), HashSet::from([from]));

    while let Some(e) = q.pop() {
        let (cost, (p, d)) = (e.cost, e.pd);
        if viz.contains(&(p, d)) {
            continue;
        }

        let paths = paths_at.get(&(cost, (p, d))).unwrap().clone();
        if to == p && min_cost >= cost {
            assert!(min_cost == cost || min_cost == i64::MAX); // if this breaks, I messed up min path
            unique_positions.extend(paths.clone());
            min_cost = cost;
            continue;
        }

        viz.insert((p, d));
        for nd in d.neighbours() {
            if !can_go(p + nd, g) || viz.contains(&(p + nd, nd)) {
                continue;
            }

            let nc = cost + 1 + ((nd != d) as i64 * 999);
            let np = if nd != d { p } else { p + nd };
            q.push(QueueEntry::new(
                (np, nd),
                nc,
            ));

            paths_at.entry((nc, (np, nd))).or_default().extend(paths.clone());
            paths_at.entry((nc, (np, nd))).or_default().extend(HashSet::from([np]));
        }
    }

    (min_cost, unique_positions)
}

fn solve(g: &Grid) -> (i64, usize) {
    let start = find(g, 'S');
    let end = find(g, 'E');

    let (min_cost, all_points) = all_min_paths(start, end, g);
    (min_cost, all_points.len())
}

fn main() -> Result<()> {
    let g = parse(Path::new("input.txt"))?;
    let (p1, p2) = solve(&g);
    println!("Part 1 {}", p1);

    println!("Part 2 {}", p2);
    Ok(())
}
