use sfml::system::Vector2f;
use sfml::window::{ Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Color, Transformable, Shape};
use rand::Rng;

fn main() {
    let size = 10;
    let num_cell: i32 = 100;
    let win_size: u32 = 1000;
    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell{rect: RectangleShape::new(), state: 0}; num_cell as usize]; num_cell as usize];
    let mut next_state: Vec<Vec<Cell>> = vec![vec![Cell{rect: RectangleShape::new(), state: 0}; num_cell as usize]; num_cell as usize];
    let mut window = RenderWindow::new((win_size, win_size),
                                "Life",
                                Style::CLOSE,
                                &Default::default());
    
    window.set_framerate_limit(60);                            
    for i in 0..num_cell{                                    
        for j in 0..num_cell{
            let mut r = RectangleShape::new();
            r.set_size(Vector2f::new(size as f32, size as f32));
            let y_pos: f32 = ((win_size as i32/ num_cell as i32) * i ) as f32;
            let x_pos:f32 =  ((win_size as i32/ num_cell as i32) * j ) as f32;
            r.set_position(Vector2f::new(x_pos, y_pos));            
            grid[i as usize][j as usize] = Cell{rect: r, state: rand::thread_rng().gen_range(0..2)};
        }
    }

    while window.is_open(){
        while let Some(event) = window.poll_event() {
            
            match event{
                Event::Closed => window.close(),
                _ => { /* Do nothing */ }

            }

        }
        window.set_active(true);

            window.clear(Color::BLACK);
            
            for i in 0..num_cell{
                for j in 0..num_cell{
                    let mut sum = 0;
                    grid[i as usize].push(Cell{rect: RectangleShape::new(), state: 0});
                    let mut cl = grid[i as usize].swap_remove(j as usize);
                    
                    for k in -1..2{
                        for l in -1..2{
                            if !(k==0 && l==0){
                                let col = (i + k + num_cell) % num_cell;
                                let row = (j + l + num_cell) % num_cell;
                                
                                sum = sum + grid[col as usize][row as usize].state;
                            }
                        }
                    }

                    if cl.state == 0 && sum == 3{
                        next_state[i as usize][j as usize] = Cell::new(1, cl.rect.position(), cl.rect.size());
                    }else if cl.state == 1 && (sum <2 || sum > 3){
                        next_state[i as usize][j as usize] = Cell::new(0, cl.rect.position(), cl.rect.size());
                    }else{
                        next_state[i as usize][j as usize] = Cell::new(cl.state, cl.rect.position(), cl.rect.size());
                    }

                    if cl.state == 0{
                        cl.rect.set_fill_color(Color::BLACK);
                    }else{
                        cl.rect.set_fill_color(Color::RED);
                    }

                    window.draw(&cl.rect);
                    grid[i as usize].push(cl);
                    grid[i as usize].swap_remove(j as usize);
                }
            }
            grid = (*next_state).to_vec();

            
            window.display();
    }
}

#[derive(Clone)]
struct Cell<'a>{
    rect: RectangleShape<'a>,
    state: i32,
}

impl Cell<'_>{
    fn new(s: i32, p: Vector2f, size: Vector2f)->Cell<'static>{
        let mut r = RectangleShape::new();
        r.set_position(p);
        r.set_size(size);
        Cell{
            rect: r,
            state: s,
        }
    }
}