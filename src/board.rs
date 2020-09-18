use crossterm::{cursor, queue, style};
use rand::prelude::*;
use std::io::Write;

static ALIVE_COL: style::Color = style::Color::Rgb {
    r: 40,
    g: 90,
    b: 240,
};

pub struct Board {
    prev: Vec<Vec<bool>>,
    alive: Vec<Vec<bool>>,
    start: Vec<Vec<bool>>,
    pub height: usize,
    pub width: usize,
    pub renders: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let vec = random_vec(width, height);

        Self {
            prev: vec.clone(),
            alive: vec.clone(),
            start: vec,
            width,
            height,
            renders: 0,
        }
    }

    pub fn edit(&mut self, vec: Vec<Vec<bool>>) {
        self.width = vec[0].len();
        self.height = vec.len();
        self.alive = vec.clone();
        self.start = vec;
        self.renders = 0;
    }

    pub fn reset(&mut self) {
        self.alive = self.start.clone();

        self.renders = 0;
    }

    fn _status_at(&self, i: i32, j: i32) -> bool {
        self.prev[(i + self.height as i32) as usize % self.height]
            [(j + self.width as i32) as usize % self.width]
    }

    pub fn display(&self, stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
        display_alive(stdout, &self.alive)
    }

    // 1. Any live cell with two or three live neighbours survives.
    // 2. Any dead cell with three live neighbours becomes a live cell.
    // 3. All other live cells die in the next generation.
    //    Similarly, all other dead cells stay dead.
    pub fn next(&mut self) {
        self.prev = self.alive.clone();

        for i in 0..(self.height as i32) {
            for j in 0..(self.width as i32) {
                let mut alive_num = 0u8;
                for di in -1..2 {
                    for dj in -1..2 {
                        // Note: Current cell status is counted as there is no check for
                        // di, dj = 0, 0
                        alive_num += self._status_at(i + di, j + dj) as u8;
                    }
                }

                // There are 2 cases for current cell
                // 1. Alive
                //      2/3 neighbours => alive
                //      since self is also counted,
                //      3/4 => alive
                // 2. Dead
                //      3 neighbours => alive
                //      self is 0 so doesnt matter
                self.alive[i as usize][j as usize] =
                    alive_num == 3 || (alive_num == 4 && self._status_at(i, j));
            }
        }
        self.renders += 1
    }
}

pub fn display_alive(
    stdout: &mut std::io::Stdout,
    alive: &Vec<Vec<bool>>,
) -> crossterm::Result<()> {
    queue!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, 0),
        style::SetForegroundColor(ALIVE_COL)
    )?;

    let h = alive.len();
    let w = alive[0].len();

    // use ▀▄ as 4 pixels
    let mut y = 0;
    while y < h {
        for x in 0..w {
            let talive = alive[y][x];
            let balive = y + 1 < h && alive[y + 1][x];

            queue!(
                stdout,
                style::Print(if talive && balive {
                    "█"
                } else if talive {
                    "▀"
                } else if balive {
                    "▄"
                } else {
                    " "
                })
            )?;
        }
        queue!(stdout, cursor::MoveToNextLine(1))?;
        y += 2;
    }

    queue!(stdout, cursor::RestorePosition, style::ResetColor,)?;

    Ok(())
}

fn random_vec(w: usize, h: usize) -> Vec<Vec<bool>> {
    let mut random = rand::thread_rng();

    let mut vec = Vec::with_capacity(h);
    for _ in 0..h {
        let mut v = Vec::with_capacity(w);

        for _ in 0..w {
            v.push(random.gen::<f32>() < 0.15);
        }

        vec.push(v);
    }

    vec
}
