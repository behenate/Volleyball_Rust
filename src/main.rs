use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::graphics::{self,Color,Texture};
use tetra::window;
use tetra::math::Vec2;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PLAYER_SPEED: f32 = 1.5f32;
const JUMP_STRENGTH: f32 = 6f32;
const FLOOR_LEVEL: f32 = 350f32;
const GRAVITY: f32 = 0.198;
const BALL_GRAVITY:f32 = 0.098;
const PLAYER_MASS: f32 = 100f32;
const BALL_MASS:f32 = 50f32;
fn main()-> tetra::Result {
    ContextBuilder::new("Volleyball", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
struct GameState {
    player1: Entity,
    player2: Entity,
    court: Staticobject,
    net: Staticobject,
    score: Score,
    ball:Ball,
}
impl GameState{
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let court_texture = Texture::new(ctx, "./resources/court.jpg")?;
        let net_texture = Texture::new(ctx, "./resources/net.png")?;
        let ball_texture = Texture:: new(ctx, "./resources/ball.png")?;
        let score_texture0 = Texture::new(ctx, "./resources/0.png")?;
        let score_texture1 = Texture::new(ctx, "./resources/1.png")?;
        let score_texture2 = Texture::new(ctx, "./resources/2.png")?;
        let score_texture3 = Texture::new(ctx, "./resources/3.png")?;
        let score_texture4 = Texture::new(ctx, "./resources/4.png")?;
        let score_texture5 = Texture::new(ctx, "./resources/5.png")?;
        let scoreline_texture = Texture::new(ctx, "./resources/scoreline.png")?;
        let scorenumbers_textures: [Texture;7] = [score_texture0,score_texture1,score_texture2,score_texture3,score_texture4,score_texture5,scoreline_texture];
        let player1_position =
            Vec2::new(16.0, WINDOW_HEIGHT - player1_texture.height() as f32 -court_texture.height() as f32);
        let player2_position =
            Vec2::new(WINDOW_WIDTH - 16.0 - player1_texture.width() as f32, WINDOW_HEIGHT - player2_texture.height() as f32 -court_texture.height() as f32);
        let court_position =
            Vec2::new(0.0, WINDOW_HEIGHT - court_texture.height() as f32);
        let net_position =
            Vec2::new(WINDOW_WIDTH/2.0 + net_texture.width() as f32/2.0 , WINDOW_HEIGHT - court_texture.height() as f32 - net_texture.height() as f32);
        let ball_position =
            Vec2::new(WINDOW_WIDTH - 100.0, ball_texture.height() as f32);
        let score_position =
            Vec2::new(WINDOW_WIDTH /2.0, 15.0);
        let mut ball: Ball = Ball::new(ball_texture, ball_position, 0f32, 620f32);
        let mut ball_ref: &Ball = &ball;
        Ok(GameState { 
            player1: Entity::new(player1_texture, player1_position, 30f32, 300f32), 
            player2: Entity::new(player2_texture, player2_position, 350f32, 600f32),
            court: Staticobject::new(court_texture, court_position),
            net: Staticobject::new(net_texture, net_position),
            ball: ball,
            score: Score::new(scorenumbers_textures,score_position,0,0)
         })
    }
    
}
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.court.texture.draw(ctx, self.court.position);
        self.net.texture.draw(ctx, self.net.position);
        self.ball.texture.draw(ctx, self.ball.position);
        self.score.scornumbers[6].draw(ctx, self.score.position);
        self.score.scornumbers[self.score.player1_score as usize].draw(ctx, Vec2::new(WINDOW_WIDTH /2.0 -10.0, 15.0));
        self.score.scornumbers[self.score.player2_score as usize].draw(ctx, Vec2::new(WINDOW_WIDTH /2.0 +10.0, 15.0));
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
        self.player1.checkBallCol(&mut self.ball);
        self.player2.checkBallCol(&mut self.ball);
        self.ball.checkNetCol(&mut self.net);
        self.score.calculateScore(&mut self.ball,&mut self.court);
        println!("{}",self.ball.position.y,);
        println!("{}",WINDOW_HEIGHT-self.court.texture.height()as f32);
        if self.score.player1_score == 5 {
            window::quit(ctx);
            println!("Player 1 wins!");
        }
        if self.score.player2_score == 5 {
            window::quit(ctx);
            println!("Player 2 wins!");
        }
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
struct Score{
    scornumbers: [Texture;7],
    position: Vec2<f32>, 
    player1_score: i32,
    player2_score: i32,
}

impl Score{
    fn new(scornumbers: [Texture;7], position: Vec2<f32>, player1_score: i32, player2_score: i32) -> Score {
        Score { scornumbers, position, player1_score, player2_score }
    }
    //WINDOW_WIDTH - 100.0, ball_texture.height() as f32
    fn calculateScore(&mut self, ball: &mut Ball,court: &mut Staticobject){
        if (ball.position.y > WINDOW_HEIGHT-court.texture.height()as f32- ball.texture.height() as f32*2.0) &&(ball.position.x > WINDOW_WIDTH/2.0){
            ball.position.x= WINDOW_WIDTH - 100.0;
            ball.position.y= ball.texture.height() as f32;
            ball.velocity.x =0.0;
            ball.velocity.y =0.0;
            self.player1_score = self.player1_score+1;

        }
        if (ball.position.y > WINDOW_HEIGHT-court.texture.height()as f32- ball.texture.height() as f32*2.0) &&(ball.position.x < WINDOW_WIDTH/2.0){
            ball.position.x= 100.0;
            ball.position.y= ball.texture.height() as f32;
            ball.velocity.x =0.0;
            ball.velocity.y =0.0;
            self.player2_score = self.player2_score+1;
        }
    }
}
struct Staticobject{
    texture: Texture,
    position: Vec2<f32>,
}

impl Staticobject{
    fn new(texture: Texture, position: Vec2<f32>) -> Staticobject {
        Staticobject { texture, position }
    }
}

struct Entity{
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    acceleration: Vec2<f32>,
    col: Col,
    x_left_lim: f32,
    x_right_lim: f32,
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
        if input::is_key_down(ctx, jump) && self.col.b{
            self.velocity.y = -JUMP_STRENGTH;
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
            self.position.y = FLOOR_LEVEL+1f32;
        }else{
            self.velocity.y += self.acceleration.y;
            if self.velocity.y > 0f32 {
                self.velocity.y += self.acceleration.y;
            }
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
    }
    fn checkBallCol(&mut self, ball: &mut Ball){
        let self_cx:f32 = self.position.x + self.texture.width()as f32/2 as f32;
        let mut self_cy:f32 = self.position.y + self.texture.height()as f32/2 as f32;
        let mut ball_cx:f32 = ball.position.x + ball.texture.width()as f32/2 as f32;
        let mut ball_cy:f32 = ball.position.y + ball.texture.height()as f32/2 as f32;
        let mut dist:f32 =  ((self_cx-ball_cx).powf(2f32) + (self_cy - ball_cy).powf(2f32)).sqrt();
        if dist < (ball.texture.width()/2 + self.texture.width()/2) as f32{
            // let u1x: f32 = (PLAYER_MASS-BALL_MASS)/(PLAYER_MASS+BALL_MASS) * self.velocity.x + (2f32*BALL_MASS)/(PLAYER_MASS+BALL_MASS)*ball.velocity.x;
            // let u1y: f32 = (PLAYER_MASS-BALL_MASS)/(PLAYER_MASS+BALL_MASS) * self.velocity.y + (2f32*BALL_MASS)/(PLAYER_MASS+BALL_MASS)*ball.velocity.y;
            // let u2x: f32 = (2f32*PLAYER_MASS)/(PLAYER_MASS+BALL_MASS)*self.velocity.x + (BALL_MASS - PLAYER_MASS)/(PLAYER_MASS+BALL_MASS) * ball.velocity.x;
            // let u2y: f32 = (2f32*PLAYER_MASS)/(PLAYER_MASS+BALL_MASS)*self.velocity.y + (BALL_MASS - PLAYER_MASS)/(PLAYER_MASS+BALL_MASS) * ball.velocity.y;
            ball.velocity.x += self.velocity.x;
            ball.velocity.y = -ball.velocity.y/2f32 + self.velocity.y/2f32;
            while dist < (ball.texture.width()/2 + self.texture.width()/2) as f32 + 3f32{
                let self_cx = self.position.x + self.texture.width()as f32/2 as f32;
                let self_cy:f32 = self.position.y + self.texture.height()as f32/2 as f32;
                let ball_cx:f32 = ball.position.x + ball.texture.width()as f32/2 as f32;
                let ball_cy:f32 = ball.position.y + ball.texture.height()as f32/2 as f32;
                dist =  ((self_cx-ball_cx).powf(2f32) + (self_cy - ball_cy).powf(2f32)).sqrt();
                ball.position.x += ball.velocity.x;
                ball.position.y += ball.velocity.y;
            }
            // self.position.x = 0f32;
        }
    }
}
struct Ball{
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    acceleration: Vec2<f32>,
    x_left_lim: f32,
    x_right_lim: f32,
}

impl Ball {
    fn new(texture: Texture, position: Vec2<f32>, x_left_lim:f32, x_right_lim:f32) -> Ball {
        let velocity = Vec2::new(0f32,0f32);
        let acceleration = Vec2::new(0f32, BALL_GRAVITY);
        Ball { texture, position, velocity, acceleration, x_left_lim, x_right_lim}
    }
    fn updateVel(&mut self){
        self.velocity.x = self.velocity.x * 0.999;
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        
    }
    fn updatePos(&mut self){
        self.position.x += self.velocity.x;  
        self.position.y += self.velocity.y;
        if self.position.y > FLOOR_LEVEL{
            self.position.y = FLOOR_LEVEL;
            self.velocity.y = -self.velocity.y;
        };
        if self.position.x < self.x_left_lim{
            self.position.x = self.x_left_lim;
            self.velocity.x = -self.velocity.x;
        };
        if self.position.x > self.x_right_lim{
            self.position.x = self.x_right_lim;
            self.velocity.x = -self.velocity.x;
        };
    }
    fn checkNetCol(&mut self, net: &mut Staticobject){
        let tw:f32 = self.texture.width() as f32 / 2.0;
        let net_tw:f32 = net.texture.width() as f32;
        if (self.position.x+tw > net.position.x) &&(self.position.x-tw < net.position.x)&& self.position.y > FLOOR_LEVEL-net.texture.height() as f32{
            self.velocity.x = -self.velocity.x/2f32;
            while self.position.x+tw > net.position.x {
                self.updatePos();
            }
        }
        if (self.position.x-tw < net.position.x+net_tw) &&(self.position.x+tw > net.position.x + net_tw)&& self.position.y > FLOOR_LEVEL-net.texture.height() as f32{
            self.velocity.x = -self.velocity.x/2f32;
            while (self.position.x < net.position.x+net_tw) &&(self.position.x+tw > net.position.x + net_tw)&& self.position.y > FLOOR_LEVEL-net.texture.height() as f32 {
                self.updatePos();
            }
        } 
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