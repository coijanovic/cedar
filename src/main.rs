use std::{thread, time};
use rand::seq::SliceRandom;
use clap::{Parser};

const SLEEP_INTERVAL : time::Duration = time::Duration::from_millis(100);

/// Watch the snake as it hunts 🐍
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use random walk
    #[arg(short, long)]
    random: bool,

    /// Use greedy shortest path
    #[arg(short, long)]
    greedy: bool,

    /// Use 'L' algorithm
    #[arg(short, long)]
    angle: bool,
}

#[derive(Copy, Clone, Debug)]
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
    /// Returns a new snake which starts in the upper left corner
    fn new(kind: char) -> Snake {
        Snake {
            kind,
            body: vec![(4,0), (3,0), (2,0), (1,0), (0,0)],
        }
    }

    /// Lets the snake execute one step in the supplied direction on the supplied field.
    /// If the snake steps onto food, she eats it.
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

    /// Checks if the snake is dead.
    /// A dead snake is a snake who has crossed over herself.
    fn is_dead(&self) -> bool {
        for i in 0..self.body.len() {
            for j in i+1..self.body.len() {
                if self.body[i] == self.body[j] {
                    return true;
                }
            }
        }
        return false;
    }

    /// Checks if the snake can eat.
    /// This is the case if her head is in the same cell as food.
    fn can_eat(&self, field: &Field) -> bool {
        if self.body[0] == field.food.position {
            return true;
        }
        return false;
    }

    /// Computes the euclidian distance between the snake's head and the food.
    fn euclidian_distance_to_food(&self, field: &Field) -> f64 {
        let x : i32 = i32::pow(self.body[0].0 as i32 - field.food.position.0 as i32, 2);
        let y : i32 = i32::pow(self.body[0].1 as i32 - field.food.position.1 as i32, 2);
        f64::sqrt((x+y) as f64)
    }

    /// Computes the toroidal distance between the snake's head and food.
    /// This should cause the snake to go through walls more often.
    /// See https://blog.demofox.org/2017/10/01/calculating-the-distance-between-points-in-wrap-around-toroidal-space/
    fn toroidal_distance_to_food(&self, field: &Field) -> f64 {
        let mut x : f64 = (self.body[0].0 as f64 - field.food.position.0 as f64).abs();
        let mut y : f64 = (self.body[0].1 as f64 - field.food.position.1 as f64).abs();

        if x > 0.5 {
            x = 1.0 - x;
        }
        if y > 0.5 {
            y = 1.0 - y;
        }
        f64::sqrt(x*x + y*y)
    }

    /// Returns the direction the snake should step in next via a greedy algorithm.
    /// The snake selects the direction which does not kill her in the next step and minimizes her
    /// (toroidal) distance to food.
    fn decide_greedy_distance(&self, field: &Field) -> Direction {
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
                let dist: f64 = future_snake.toroidal_distance_to_food(field);
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

    /// Returns the direction the snake wants to step in next via a random choice (of directions which
    /// do not kill her in the next step).
    fn decide_random(&self, field: &Field) -> Direction {
        let mut ps : Vec<Direction> = Vec::new();
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
                ps.push(dir);
            }
        }
        return *ps.choose(&mut rand::thread_rng()).unwrap();
    }

    /// Returns the direction the snake wants to step in next based on the x-position of the food:
    /// If the snake is in the same column as the food, go down.
    /// Otherwise go right.
    fn decide_l(&self, field: &Field) -> Direction {
        if self.body[0].0 == field.food.position.0 {
            return Direction::Down;
        } else {
            return Direction::Right;
        }
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
    /// Prints the field including snake and food.
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

    /// Returns a random coordinate that lies within the field
    fn get_random_pos(&self) -> (u8, u8) {
        let x : u8 = rand::random::<u8>() % self.width;
        let y : u8 = rand::random::<u8>() % self.height;
        (x, y)
    }
}

fn main() {
    let args = Args::parse();

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
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut next_dir : Direction = Direction::Up;
        if args.random {
            next_dir = s.decide_random(&f);
        } else if args.greedy {
            next_dir = s.decide_greedy_distance(&f);
        } else if args.angle {
            next_dir = s.decide_l(&f);
        } else {
            next_dir = s.decide_random(&f);
        }

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
