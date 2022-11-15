use sfml::{window::{ Event, Style}, graphics::{RenderWindow, RenderTarget, RectangleShape, Color, Transformable, Shape}, system::Vector2f};
use rand::Rng;
fn main() {
    let size = 2;
    const num_cell: i32 = 400;
    let win_size: u32 = 800;
    let mut window = RenderWindow::new((win_size, win_size), "Life", Style::CLOSE, &Default::default());  
    let mut state = [[0; num_cell as usize]; num_cell as usize];
    let mut next_state = [[0; num_cell as usize]; num_cell as usize];
    for i in 0..num_cell{                                    
        for j in 0..num_cell{
            state[i as usize][j as usize ] = rand::thread_rng().gen_range(0..2) as u8;
        }
    }
    let mut r = RectangleShape::new();
    r.set_size(Vector2f::new(size as f32, size as f32));
    let mut pos = Vector2f::new(0.0, 0.0);
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
                    let cl = state[i as usize][j as usize];
                    for k in -1..2{
                        for l in -1..2{
                            if !(k==0 && l==0){
                                let col = (i + k + num_cell) % num_cell;
                                let row = (j + l + num_cell) % num_cell;
                                
                                sum = sum + state[col as usize][row as usize];
                            }
                        }
                    }
                    pos.x = ((win_size as i32/ num_cell as i32) * i ) as f32;
                    pos.y =  ((win_size as i32/ num_cell as i32) * j ) as f32;
                    r.set_position(pos);
                    if cl == 0 && sum == 3{
                        next_state[i as usize][j as usize] = 1;
                        r.set_fill_color(Color::RED);
                        window.draw(&r);
                    }else if cl == 1 && (sum <2 || sum > 3){
                        next_state[i as usize][j as usize] = 0;
                        r.set_fill_color(Color::BLACK);
                        window.draw(&r);
                    }else{
                        next_state[i as usize][j as usize] = cl;
                    }
                }
            }
            state = next_state;          
            window.display();
    }
}