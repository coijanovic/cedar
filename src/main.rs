use rand::random;
use rand::seq::SliceRandom;
use std::{thread, time};

const SLEEP_INTERVAL : time::Duration = time::Duration::from_millis(150);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
const DIRS : [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

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

    fn step(&mut self, field: &mut Field, dir: &Direction) {
        let new_head : (u8, u8) = match dir {
            Direction::Up => (self.body[0].0, (self.body[0].1 + field.height - 1) % field.height),
            Direction::Down => (self.body[0].0, (self.body[0].1 + 1) % field.height), 
            Direction::Left => ((self.body[0].0 + field.width - 1) % field.width, self.body[0].1),
            Direction::Right => ((self.body[0].0 + 1) % field.width, self.body[0].1),
        };
        self.body.insert(0, new_head);
        if self.can_eat(&field) {
            field.food = Food {
                kind: '🥐',
                position: field.get_random_pos(),
            };
        } else {
            let _ = self.body.pop();
        }
    }

    fn is_dead(&self) -> bool {
        for (i, one) in self.body.iter().enumerate() {
            for (j, two) in self.body.iter().enumerate() {
                if i != j && one == two {
                    return true;
                }
            }
        }
        return false;
    }

    fn can_eat(&self, field: &Field) -> bool {
        if self.body[0] == field.food.position {
            return true;
        }
        return false;
    }
}

struct Food {
    kind: char,
    position: (u8, u8),
}

struct Field {
    width: u8,
    height: u8,
    kind: char,
    food: Food,
}

impl Field {
    fn print(&self, snake: &Snake) {
        for i in 0..self.height {
            for j in 0..self.width {
                // check if there's a snake
                if snake.body.contains(&(j,i)) {
                    print!("{}", snake.kind);
                } else if (j,i) == self.food.position {
                    print!("{}", self.food.kind);
                } else {
                    print!("{}", self.kind);
                }
            }
            print!("\n");
        }
        print!("\n");
    }   

    fn get_random_pos(&self) -> (u8, u8) {
        let x : u8 = rand::random::<u8>() % self.width;
        let y : u8 = rand::random::<u8>() % self.height;
        (x, y)
    }
}

fn main() {
    let mut s = Snake::new('🤖');
    let mut f = Field {
        width: 30,
        height: 20,
        kind: '⚫',
        food: Food {
            kind: '🥐',
            position: (10, 15),
        },
    };
    loop {

        let d = DIRS.choose(&mut rand::thread_rng()).unwrap();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Next step: {:?}", d);
        s.step(&mut f, d);
        if s.is_dead() {
            println!("Snek is ded. So sad! 🪦");
            println!("Snek was {} years old.", s.body.len()-5);
            break;
        }
        f.print(&s);
        thread::sleep(SLEEP_INTERVAL);
    }

}
