use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

use gui::view::board_view::Renderer;
use gui::model::game::{self, make_blank_board, GameState, BoardPiece};


fn main() -> Result<(), String> {

    let screen_width = 800;
    let screen_height = 600;
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust!", screen_width, screen_height)
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let board_view = Renderer::new(screen_width,screen_height);

    let mut game_state = GameState::new();

    
    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    
    while running {
        
        for event in event_queue.poll_iter() {
            
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                
                Event::MouseButtonDown { x, y ,..} => {
                    let col : usize = (5 * x / board_view.screen_area.w).try_into().unwrap();
                    let row : usize = (5 * y / board_view.screen_area.h).try_into().unwrap();
                    game_state.handle_click(row, col)
                }
                    
                _ => {}
            }
            }
            board_view.render(&mut canvas, &game_state.board);
            canvas.present();
        }
        
    Ok(())
}