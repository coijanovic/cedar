use std::{thread, time};

const SLEEP_INTERVAL : time::Duration = time::Duration::from_millis(100);

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

    fn distance_to_food(&self, field: &Field) -> f64 {
        let x : i32 = i32::pow(self.body[0].0 as i32 - field.food.position.0 as i32, 2);
        let y : i32 = i32::pow(self.body[0].1 as i32 - field.food.position.1 as i32, 2);
        f64::sqrt((x+y) as f64)
    }

    fn decide(&self, field: &Field) -> Direction {
        let mut ps : Vec<(Direction, f64)> = Vec::new();
        for dir in DIRS {
            let mut future_snake = Snake {
                kind : self.kind,
                body : self.body.clone(),
            };
            let mut future_field = Field {
                width : field.width,
                height : field.height,
                kind : field.kind,
                food : Food {
                    kind : field.food.kind,
                    position : field.food.position,
                },
            };
            future_snake.step(&mut future_field, &dir);
            if !future_snake.is_dead() {
                let dist: f64 = future_snake.distance_to_food(field);
                ps.push((dir, dist));
            }
        }
        let mut best_dir : Direction = Direction::Up;
        let mut min_dist : f64 = f64::MAX;
        for (d, f) in ps {
            if f < min_dist {
                min_dist = f;
                best_dir = d;
            }
        }
        return best_dir;
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

        //let d = DIRS.choose(&mut rand::thread_rng()).unwrap();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let next_dir : Direction = s.decide(&f);
        s.step(&mut f, &next_dir);
        if s.is_dead() {
            println!("Snek is ded. So sad! 🪦");
            println!("Snek was {} years old.", s.body.len()-5);
            break;
        }
        f.print(&s);
        thread::sleep(SLEEP_INTERVAL);
    }

}
