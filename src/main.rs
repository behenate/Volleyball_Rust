use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::graphics::{self,Color,Texture};
use tetra::math::Vec2;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PLAYER_SPEED: f32 = 1.5f32;
const FLOOR_LEVEL: f32 = 350f32;
const GRAVITY: f32 = 0.98;
fn main()-> tetra::Result {
    ContextBuilder::new("Volleyball", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
struct GameState {
    player1: Entity,
    player2: Entity,
    court: Entity,
    voley: Entity,
    ball: Entity,
}
impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let court_texture = Texture::new(ctx, "./resources/court.jpg")?;
        let voley_texture = Texture::new(ctx, "./resources/voley.png")?;
        let ball_texture = Texture:: new(ctx, "./resources/ball.png")?;
        let player1_position =
            Vec2::new(16.0, WINDOW_HEIGHT - player1_texture.height() as f32 -court_texture.height() as f32);
        let player2_position =
            Vec2::new(WINDOW_WIDTH - 16.0 - player1_texture.width() as f32, WINDOW_HEIGHT - player2_texture.height() as f32 -court_texture.height() as f32);
        let court_position =
            Vec2::new(0.0, WINDOW_HEIGHT - court_texture.height() as f32);
        let voley_position =
            Vec2::new(WINDOW_WIDTH/2.0 - voley_texture.width() as f32/2.0, WINDOW_HEIGHT - court_texture.height() as f32 - voley_texture.height() as f32);
        let ball_position =
            Vec2::new(WINDOW_WIDTH - 16.0 - player1_texture.width() as f32, ball_texture.height() as f32);
        Ok(GameState { 
            player1: Entity::new(player1_texture, player1_position, 30f32, 300f32), 
            player2: Entity::new(player2_texture, player2_position, 350f32, 600f32),
            court: Entity::new(court_texture, court_position, 0f32, 640f32),
            voley: Entity::new(voley_texture, voley_position, 0f32, 640f32),
            ball: Entity::new(ball_texture, ball_position, 0f32, 640f32)
         })
    }
    
}
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

    
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.court.texture.draw(ctx, self.court.position);
        self.voley.texture.draw(ctx, self.voley.position);
        self.ball.texture.draw(ctx, self.ball.position);
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.player1.checkInput(ctx, Key::A, Key::D, Key::W);
        self.player2.checkInput(ctx, Key::Left, Key::Right, Key::Up);
        self.player1.updateVel();
        self.player2.updateVel();
        self.player1.updatePos();
        self.player2.updatePos();
        self.ball.updateVel();
        self.ball.updatePos();
        Ok(())
    }
    
}
struct Col{
    t: bool,
    b: bool,
    l: bool,
    r: bool
}
impl Col{
    fn new(t:bool, b:bool, l:bool, r:bool) -> Col{
        Col {t,b,r,l}
    } 
}

struct Entity{
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    acceleration: Vec2<f32>,
    col: Col,
    x_left_lim: f32,
    x_right_lim: f32
}
impl Entity {
    fn new(texture: Texture, position: Vec2<f32>, x_left_lim:f32, x_right_lim:f32) -> Entity {
        let velocity = Vec2::new(0f32,0f32);
        let acceleration = Vec2::new(0f32, GRAVITY);
        let col = Col::new(false, false, false, false);
        Entity { texture, position, velocity, acceleration, col, x_left_lim, x_right_lim}
    }
    fn checkInput(&mut self, ctx: &mut Context, left:Key, right:Key, jump:Key){
        if input::is_key_down(ctx, right) {
            self.acceleration.x = PLAYER_SPEED;
        }else if input::is_key_down(ctx, left) {
            self.acceleration.x = -PLAYER_SPEED;
        }else{
            self.acceleration.x = 0f32;
        }
        if input::is_key_down(ctx, jump){
            self.velocity.y = -5f32;
        }
    }
    fn updateVel(&mut self){
        
        self.velocity.x = self.velocity.x - (self.velocity.x)*0.2;
        if (self.col.l && self.velocity.x < 0f32 || self.col.r && self.velocity.x > 0f32){
            self.velocity.x = 0f32;
        }else{
            self.velocity.x += self.acceleration.x;
        }
        if (self.col.b) && self.velocity.y > 0f32{
            self.velocity.y = 0f32;
        }else{
            self.velocity.y += self.acceleration.y;
        }
        
    }
    fn updatePos(&mut self){
        if (!self.col.l || self.velocity.x > 0f32) && (!self.col.r || self.velocity.x < 0f32){
            self.position.x += self.velocity.x;  
        }
        if (!self.col.b || self.velocity.y < 0f32) && (!self.col.t || self.velocity.y > 0f32){
            self.position.y += self.velocity.y;
        } 
        self.col.r =self.position.x > self.x_right_lim;
        self.col.l = self.position.x < self.x_left_lim;
        self.col.b = self.position.y > FLOOR_LEVEL;
        println!("{}", self.position.y);
    }
}












// struct Bb{
//     x1: f32,
//     y1: f32,
//     x2: f32,
//     y2: f32
// }
// impl Bb{
//     fn new(x1:f32, y1:f32, x2: f32, y2: f32) -> Bb{
//         Bb {x1, y1, x2, y2}
//     }
//     fn checkCol(&mut self, other: Bb) -> Col{
//         let intersect_r: bool = self.x2 > other.x1 && self.x2 < other.x2;
//         let intersect_l: bool = self.x1 < other.x2 && self.x2 > other.x2;
//         let intersect_b: bool = self.y1 < other.y2 && self.y1 > other.y1;
//         let intersect_t: bool = self.y2 > other.y1 && self.y2 < other.y2;
//         let intersect_x: bool = intersect_l || intersect_r;
//         let intersect_y: bool = intersect_t || intersect_b;
//         let newCol = Col::new(intersect_t && intersect_x, intersect_b && intersect_y, intersect_l && intersect_y, intersect_r&&intersect_y);
//         return newCol;
//     }
// }