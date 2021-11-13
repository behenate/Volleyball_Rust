use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::graphics::{self,Color,Texture};
use tetra::math::Vec2;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PLAYER_SPEED: f32 = 0.5f32;

fn main()-> tetra::Result {
    ContextBuilder::new("Volleyball", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
struct GameState {
    player1: Entity,
    player2: Entity,
}
impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player1_position =
            Vec2::new(16.0, (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);
        let player2_position =
            Vec2::new(300.0, (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0);
        Ok(GameState { 
            player1: Entity::new(player1_texture, player1_position), 
            player2: Entity::new(player2_texture, player2_position)
         })
    }
    
}
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
    
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        self.player1.checkInput(ctx, Key::A, Key::D);
        self.player2.checkInput(ctx, Key::Left, Key::Right);
        self.player1.updateVel();
        self.player2.updateVel();
        self.player1.updatePos();
        self.player2.updatePos();
        Ok(())
    }
}

struct Entity{
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    acceleration: Vec2<f32>
}
impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        let velocity = Vec2::new(0f32,0f32);
        let acceleration = Vec2::new(0f32, 9.81f32);
        Entity { texture, position, velocity, acceleration}
    }
    fn checkInput(&mut self, ctx: &mut Context, left:Key, right:Key){
        if input::is_key_down(ctx, right) {
            self.acceleration.x = PLAYER_SPEED;
        }else if input::is_key_down(ctx, left) {
            self.acceleration.x = -PLAYER_SPEED;
        }else{
            self.acceleration.x = 0f32;
        }
    }
    fn updateVel(&mut self){
        self.velocity.x += self.acceleration.x;
        self.velocity.x = self.velocity.x - (self.velocity.x)*0.1;
    }
    fn updatePos(&mut self){
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}