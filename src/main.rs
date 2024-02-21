/* Copyright (C) SniverDaBest - All Rights Reserved
 * Unauthorized copying or distribution of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Owen Marquardt <owen.gary629@gmail.com>, Month Year
 */

extern crate piston_window;
extern crate image;

use piston_window::*;
use std::collections::HashSet;
use image::*;

// Player Struct
pub struct Player {
    x: f64,
    y: f64,
    direction: i32,
    isMoving: bool,
    hp: i32,
    maxHP: i32,
    stamina: i32,
    maxStamina: i32,
    speed: f64,
    maxSpeed: f64,
    mana: i32,
    maxMana: i32,
}

// Player functions
impl Player {
    pub fn move_up(&mut self, speed: f64) {
        self.y += speed;
        self.direction =  0;
    }

    pub fn move_down(&mut self, speed: f64) {
        self.y -= speed;
        self.direction =  180;
    }

    pub fn move_left(&mut self, speed: f64) {
        self.x -= speed;
        self.direction = -90;
    }

    pub fn move_right(&mut self, speed: f64) {
        self.x += speed;
        self.direction =  90;
    }
}

// Main function
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Untitled Pixel Game: Developer Version", [1270, 720])
        .exit_on_esc(false).build().unwrap();

    let mut player = Player { x: 0.0, y: 0.0, direction: 180, isMoving: false, hp: 10, maxHP: 10, stamina: 30, maxStamina: 30, speed: 0.0, maxSpeed: 10.0, mana: 0, maxMana: 0}; // Player variable
    let mut keys: HashSet<Key> = HashSet::new(); // Make keypress variable
    let mut img = image::open("assets/images/stevess.png").unwrap(); // Load the player spritesheet image
    let playerDown = img.sub_image(0, 0, 48, 48).to_image(); // Set variable for player facing down
    let playerUp = img.sub_image(0, 96, 48, 48).to_image(); // Set variable for player facing up
    let playerRight = img.sub_image(0, 48, 48, 48).to_image(); // Set variable for player facing right
    let playerLeft = img.sub_image(0, 144, 48, 48).to_image(); // Set variable for player facing left
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    }; // Idk what this does
    let playerDown = Texture::from_image(&mut texture_context, &playerDown, &TextureSettings::new()).unwrap(); // More player sprite stuff
    let playerUp = Texture::from_image(&mut texture_context, &playerUp, &TextureSettings::new()).unwrap();
    let playerRight = Texture::from_image(&mut texture_context, &playerRight, &TextureSettings::new()).unwrap();
    let playerLeft = Texture::from_image(&mut texture_context, &playerLeft, &TextureSettings::new()).unwrap();

    while let Some(event) = window.next() { // While loop
        if let Some(Button::Keyboard(key)) = event.press_args() {
            keys.insert(key); // On keypress
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            keys.remove(&key); // On key release
        }

        // Movement stuff...
        if keys.contains(&Key::W) || keys.contains(&Key::Up) {
            player.move_up(10.0);
            player.isMoving = true;
        }

        if keys.contains(&Key::A) || keys.contains(&Key::Left) {
            player.direction = -90;
            player.x -= 0.5;
            player.isMoving = true
        }

        if keys.contains(&Key::S) || keys.contains(&Key::Down) {
            player.direction = 180;
            player.y += 0.5;
            player.isMoving = true;
        }

        if keys.contains(&Key::D) || keys.contains(&Key::Right) {
            player.direction = 90;
            player.x += 0.5;
            player.isMoving = true;
        }

        if !keys.contains(&Key::W) || !keys.contains(&Key::Up) || !keys.contains(&Key::S) || !keys.contains(&Key::Down) || !keys.contains(&Key::A) || !keys.contains(&Key::Left) || !keys.contains(&Key::D) || !keys.contains(&Key::Right) {
            player.isMoving = false;
        }

        // Draw the crap to the screen 
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics); // Player rotation stuff...
            if player.direction == 0 {
                image(&playerUp, context.transform.trans(player.x, player.y), graphics);
            } else if player.direction == 90 {
                image(&playerRight, context.transform.trans(player.x, player.y), graphics);
            } else if player.direction == 180 {
                image(&playerDown, context.transform.trans(player.x, player.y), graphics);
            } else if player.direction == -90 {
                image(&playerLeft, context.transform.trans(player.x, player.y), graphics);
            } else if player.direction == 360 {
                player.direction = 0;
            } else {
                println!("Weird direction: {}. Rendering playerDown and setting rotation to 180...", player.direction);
                player.direction = 180;
                image(&playerDown, context.transform.trans(player.x, player.y), graphics);
            }
        });
    }
}

