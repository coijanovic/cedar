use std::{thread, time};

const SLEEP_INTERVAL : time::Duration = time::Duration::from_millis(100);

enum Direction {
 Up,
 Down,
 Left,
 Right,
}

struct Snake {
    kind: char,
    body: Vec<(u8, u8)>, 
}

impl Snake {
    fn new(kind: char) -> Snake {
        Snake {
            kind,
            body: vec![(4,0), (3,0), (2,0), (1,0), (0,0)],
        }
    }

    fn step(&mut self, field: &Field, dir: Direction) {
        let _ = self.body.pop();
        let new_head : (u8, u8) = match dir {
            Direction::Up => (self.body[0].0, (self.body[0].1 + field.height - 1) % field.height),
            Direction::Down => (self.body[0].0, (self.body[0].1 + 1) % field.height), 
            Direction::Left => ((self.body[0].0 + field.width - 1) % field.width, self.body[0].1),
            Direction::Right => ((self.body[0].0 + 1) % field.width, self.body[0].1),
        };
        self.body.insert(0, new_head);
    }
}

struct Field {
    width: u8,
    height: u8,
    kind: char,
}

impl Field {
    fn print(&self, snake: &Snake) {
        for i in 0..self.height {
            for j in 0..self.width {
                // check if there's a snake
                if snake.body.contains(&(j,i)) {
                    print!("{}", snake.kind);
                } else {
                    print!("{}", self.kind);
                }
            }
            print!("\n");
        }
        print!("\n");
    }   
}

fn main() {
    let mut s = Snake::new('🟠');
    let f = Field {
       width: 30,
       height: 20,
       kind: '⚫',
    };
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        s.step(&f, Direction::Right);
        f.print(&s);
        thread::sleep(SLEEP_INTERVAL);

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        s.step(&f, Direction::Up);
        f.print(&s);
        thread::sleep(SLEEP_INTERVAL);
    }
}
