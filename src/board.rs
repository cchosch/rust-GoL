use rand::Rng;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy)]
pub enum BoardState {
    Alive,
    Dead,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    grid: Vec<Vec<BoardState>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: (0..height)
                .map(|_| (0..width).map(|_| BoardState::random()).collect())
                .collect(),
            width,
            height,
        }
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<BoardState> {
        if !(0..self.width).contains(&x) {
            return None;
        }
        if !(0..self.height).contains(&y) {
            return None;
        }
        return Some(self.grid[y][x]);
    }

    pub fn get_row(&self, y: usize) -> Vec<BoardState> {
        if !(0..self.height).contains(&y) {
            return vec![];
        }
        return self.grid[y].clone();
    }

    pub fn get_col(&self, x: usize) -> Vec<BoardState> {
        if !(0..self.width).contains(&x) {
            return vec![];
        }
        let mut new_vec = Vec::with_capacity(self.height);
        for y in 0..self.height {
            new_vec.push(self.get(x, y).unwrap());
        }
        new_vec
    }

    pub fn update(&mut self) {
        let ref_board = Arc::new(self.clone());
        let mut recvers: Vec<Receiver<(usize, Vec<BoardState>)>> = vec![];
        for row in 0..self.height {
            recvers.push(update_row(row, ref_board.clone()));
        }
        for recver in recvers {
            let (row, new_vec) = recver.recv().unwrap();
            self.grid[row] = new_vec;
        }
    }
}

impl BoardState {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..2) {
            0 => BoardState::Alive,
            1 => BoardState::Dead,
            _ => unreachable!(),
        }
    }
}

fn update_row(row: usize, board: Arc<Board>) -> Receiver<(usize, Vec<BoardState>)> {
    let (ch_send, ch_recv) = channel();
    thread::spawn(move || {
        let mut new_vals = Vec::new();
        for x in 0..board.width {
            new_vals.push(new_value(x, row, board.clone()));
        }
        ch_send.send((row, new_vals)).unwrap();
    });
    ch_recv
}

fn new_value(x: usize, y: usize, board: Arc<Board>) -> BoardState {
    let current = board.get(x, y);
    if let None = current {
        return BoardState::Dead;
    }
    let current = current.unwrap();
    let x = x as i32;
    let y = y as i32;

    let mut alive = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            alive += if board
                .get((x + i) as usize, (y + j) as usize)
                .unwrap_or(BoardState::Dead)
                == BoardState::Alive
            {
                1
            } else {
                0
            };
        }
    }
    if alive == 3 || (current == BoardState::Alive && alive == 2) {
        return BoardState::Alive;
    }
    return BoardState::Dead;
}
