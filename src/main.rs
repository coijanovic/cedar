struct Snake {
    kind: char,
    body: Vec<(u8, u8)>, 
}

impl Snake {
    fn new(kind: char) -> Snake {
        Snake {
            kind,
            body: vec![(0,0), (1,0), (2,0), (3,0), (4,0)],
        }
    }
}

struct Field {
    width: u8,
    height: u8,
    kind: char,
    snake: Snake,
}

impl Field {
    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                // check if there's a snake
                if self.snake.body.contains(&(j,i)) {
                    print!("{}", self.snake.kind);
                } else {
                    print!("{}", self.kind);
                }
            }
            print!("\n");
        }
    }   
}

fn main() {
    let s = Snake::new('🟠');
    let f = Field {
       width: 30,
       height: 20,
       kind: '⚫',
       snake: s,
    };
    f.print();
}
