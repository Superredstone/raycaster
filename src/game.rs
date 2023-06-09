use std::collections::HashMap;
const a: u8 = 10;
use macroquad::{color, prelude::*, text::draw_text, time::get_fps, window::clear_background};

const MAP_SIZE: usize = 24;
const MINIMAP_SIZE: Vec2 = Vec2 { x: 200.0, y: 200.0 }; // { x: 200.0, y: 200.0 };
const MINIMAP_CELL_SIZE_X: f32 = MINIMAP_SIZE.x / MAP_SIZE as f32;
const MINIMAP_CELL_SIZE_Y: f32 = MINIMAP_SIZE.y / MAP_SIZE as f32;
const MINIMAP_PADDING: f32 = 5.0;

const TEST_MAP: [[u32; MAP_SIZE]; MAP_SIZE] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

pub struct Game {
    pub window_size: Vec2,
    pub player: Player,
    pub current_map: [[u32; MAP_SIZE]; MAP_SIZE],
    pub textures: HashMap<String, Texture2D>,
    pub textures_datas: HashMap<String, Image>,
    pub textures_image_data: HashMap<String, Vec<[u8; 4]>>,

    // u8 [r, g, b, alpha]
    pub buffer: Vec<Vec<[u8; 4]>>,
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: Vec2,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub camera_plane: Vec2,
}

impl Game {
    pub fn new(
        textures: HashMap<String, Texture2D>,
        textures_datas: HashMap<String, Image>,
        textures_image_data: HashMap<String, Vec<[u8; 4]>>,
    ) -> Game {
        const WINDOW_WIDTH: i32 = 800;
        const WINDOW_HEIGHT: i32 = 600;

        let buffer = vec![vec![[0, 0, 0, 0]; WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize];

        Game {
            window_size: Vec2 {
                x: WINDOW_WIDTH as f32,
                y: WINDOW_HEIGHT as f32,
            },
            player: Player {
                position: Vec2 { x: 1.25, y: 1.25 },
                velocity: Vec2 { x: 0.0, y: 0.0 },
                direction: Vec2 { x: 1.0, y: 0.0 },
                movement_speed: 5.0,
                rotation_speed: 3.0,
                // FOV 66°
                camera_plane: Vec2 { x: 0.0, y: 0.66 },
            },
            current_map: TEST_MAP,
            textures,
            textures_datas,
            textures_image_data,
            buffer,
        }
    }

    pub fn update(&mut self) {
        // Handle input
        self.handle_input();
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);

        for x in 0..(self.window_size.x as u32) {
            let camera_x = 2.0 * (x as f32) / self.window_size.x - 1.0;

            let raycast_direction = Vec2 {
                x: self.player.direction.x + self.player.camera_plane.x * camera_x,
                y: self.player.direction.y + self.player.camera_plane.y * camera_x,
            };

            let delta_distance_x = if raycast_direction.x == 0.0 {
                1e30
            } else {
                (1.0 / raycast_direction.x).abs()
            };

            let delta_distance_y = if raycast_direction.y == 0.0 {
                1e30
            } else {
                (1.0 / raycast_direction.y).abs()
            };

            #[allow(unused_assignments)]
            let mut perpendicular_wall_distance: f32 = 0.0;

            let mut side_distance = Vec2 { x: 0.0, y: 0.0 };

            let mut map_x = self.player.position.x as i16;
            let mut map_y = self.player.position.y as i16;

            #[allow(unused_assignments)]
            let mut step_x: i16 = 0;
            #[allow(unused_assignments)]
            let mut step_y: i16 = 0;

            let mut hit: u16 = 0;
            let mut side: u16 = 0;

            if raycast_direction.x < 0.0 {
                step_x = -1;
                side_distance.x = (self.player.position.x - map_x as f32) * delta_distance_x;
            } else {
                step_x = 1;
                side_distance.x = ((map_x as f32) + 1.0 - self.player.position.x) * delta_distance_x
            }

            if raycast_direction.y < 0.0 {
                step_y = -1;
                side_distance.y = (self.player.position.y - map_y as f32) * delta_distance_y
            } else {
                step_y = 1;
                side_distance.y = ((map_y as f32) + 1.0 - self.player.position.y) * delta_distance_y
            }

            while hit == 0 {
                if side_distance.x < side_distance.y {
                    side_distance.x += delta_distance_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_distance.y += delta_distance_y;
                    map_y += step_y;
                    side = 1;
                }

                if self.current_map[map_x as usize][map_y as usize] > 0 {
                    hit = 1;
                }
            }

            // let texture = self.textures.get("sample_wall").unwrap();
            let texture_data = self.textures_datas.get("sample_wall").unwrap();
            let texture_width = texture_data.width; // texture.width();
            let texture_height = texture_data.height; // texture.height();

            if side == 0 {
                perpendicular_wall_distance = side_distance.x - delta_distance_x;
            } else {
                perpendicular_wall_distance = side_distance.y - delta_distance_y;
            }

            let line_height = self.window_size.y / perpendicular_wall_distance;
            let pitch: f32 = 100.0;

            let mut draw_start: i32 =
                (-line_height / 2.0 + self.window_size.y / 2.0 + pitch) as i32;
            if draw_start < 0 {
                draw_start = 0;
            }

            let mut draw_end = line_height / 2.0 + self.window_size.y / 2.0 + pitch;
            if draw_end >= self.window_size.y {
                draw_end = self.window_size.y - 1.0;
            }

            let _texture_number = self.current_map[map_x as usize][map_y as usize] - 1;

            #[allow(unused_assignments)]
            let mut wall_x: f32 = 0.0;

            if side == 0 {
                wall_x = self.player.position.y + perpendicular_wall_distance * raycast_direction.y;
            } else {
                wall_x = self.player.position.x + perpendicular_wall_distance * raycast_direction.x;
            }
            wall_x -= f32::floor(wall_x);

            let mut texture_x = ((wall_x as u16) * texture_width) as u32;
            if side == 0 && raycast_direction.x > 0.0 {
                texture_x = (texture_width as u32) - texture_x - 1; // 1;
            }
            if side == 1 && raycast_direction.y < 0.0 {
                texture_x = (texture_width as u32) - texture_x - 1;
            }

            let color: Color = match self.current_map[map_x as usize][map_y as usize] {
                1 => BLACK,
                2 => GREEN,
                3 => PURPLE,
                4 => BLUE,
                5 => RED,
                _ => WHITE,
            };

            let step: f32 = 1.0 * (texture_height as f32) / line_height;
            let mut texture_position: f32 =
                ((draw_start as f32) - pitch - self.window_size.y / 2.0 + line_height / 2.0) * step;

            let mut y = draw_start;
            // while y < (draw_end as i32) {
            //     let texture_y = (texture_position as u32) & ((texture_height as u32) - 1);
            //     texture_position += step;

            //     let color = self
            //         .textures_datas
            //         .get("sample_wall")
            //         .unwrap()
            //         .get_image_data()
            //         [((texture_height as u32) * texture_y + texture_x) as usize];

            //     self.buffer[y as usize][x as usize] = color;

            //     y += 1;
            // }

            // unsafe {
            //     let buf_n = 5000;
            //     let mut buffer_u32: Vec<u32> = vec![];
            //     for _ in 0..buf_n {
            //         buffer_u32.push(0);
            //     }

            //     let a: *const c_void = buffer_u32.as_ptr() as *const GLvoid;

            //     let mut vbo: GLuint = 0;
            //     glGenBuffers(1, &mut vbo);
            //     glBindBuffer(GL_ARRAY_BUFFER, vbo);

            //     glBufferData(gl::GL_ARRAY_BUFFER, buf_n, a, GL_STATIC_DRAW);

            //     glDrawArrays(GL_POINTS, 0, buf_n);
            //     glDrawBuffers(buf_n, a as *const u32);
            // };

            draw_line(x as f32, draw_start as f32, x as f32, draw_end, 1.0, color);
        }

        self.draw_ui();
    }

    fn draw_ui(&mut self) {
        // FPS counter
        draw_text(
            &format!("FPS: {}", get_fps()),
            self.window_size.x - 100.0,
            20.0,
            24.0,
            BLACK,
        );

        self.draw_minimap();
    }

    fn draw_minimap(&mut self) {
        draw_rectangle_lines(
            MINIMAP_PADDING,
            MINIMAP_PADDING,
            MINIMAP_SIZE.x,
            MINIMAP_SIZE.y,
            3.0,
            BLACK,
        );

        for row in 0..MAP_SIZE {
            for col in 0..MAP_SIZE {
                let current_cell = self.current_map[row][col];
                let color: Color = match current_cell {
                    0 => WHITE,
                    1 => BLACK,
                    2 => GREEN,
                    3 => PURPLE,
                    4 => BLUE,
                    5 => RED,
                    _ => WHITE,
                };

                draw_rectangle(
                    MINIMAP_PADDING + row as f32 * MINIMAP_CELL_SIZE_X,
                    MINIMAP_PADDING + col as f32 * MINIMAP_CELL_SIZE_Y,
                    MINIMAP_CELL_SIZE_X,
                    MINIMAP_CELL_SIZE_Y,
                    color,
                );
            }

            // Draw player
            draw_circle(
                MINIMAP_PADDING + (MINIMAP_SIZE.x / 24.0 * self.player.position.x),
                MINIMAP_PADDING + (MINIMAP_SIZE.y / 24.0 * self.player.position.y),
                3.0,
                RED,
            );
        }
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::A) {
            let rotation_speed = self.player.rotation_speed * get_frame_time();
            let old_direction_x = self.player.direction.x;

            self.player.direction.x = old_direction_x * f32::cos(-rotation_speed)
                - self.player.direction.y * f32::sin(-rotation_speed);

            self.player.direction.y = old_direction_x * f32::sin(-rotation_speed)
                + self.player.direction.y * f32::cos(-rotation_speed);

            let old_plane_x = self.player.camera_plane.x;

            self.player.camera_plane.x = old_plane_x * f32::cos(-rotation_speed)
                - self.player.camera_plane.y * f32::sin(-rotation_speed);

            self.player.camera_plane.y = old_plane_x * f32::sin(-rotation_speed)
                + self.player.camera_plane.y * f32::cos(-rotation_speed);
        }

        if is_key_down(KeyCode::D) {
            let rotation_speed = -self.player.rotation_speed * get_frame_time();
            let old_direction_x = self.player.direction.x;

            self.player.direction.x = old_direction_x * f32::cos(-rotation_speed)
                - self.player.direction.y * f32::sin(-rotation_speed);

            self.player.direction.y = old_direction_x * f32::sin(-rotation_speed)
                + self.player.direction.y * f32::cos(-rotation_speed);

            let old_plane_x = self.player.camera_plane.x;

            self.player.camera_plane.x = old_plane_x * f32::cos(-rotation_speed)
                - self.player.camera_plane.y * f32::sin(-rotation_speed);

            self.player.camera_plane.y = old_plane_x * f32::sin(-rotation_speed)
                + self.player.camera_plane.y * f32::cos(-rotation_speed);
        }

        if is_key_down(KeyCode::S) {
            let movement_speed = self.player.movement_speed * get_frame_time();

            if self.current_map
                [(self.player.position.x - self.player.direction.x * movement_speed) as usize]
                [self.player.position.y as usize]
                == 0
            {
                self.player.position.x -= self.player.direction.x * movement_speed;
            }

            if self.current_map[self.player.position.x as usize]
                [(self.player.position.y - self.player.direction.y * movement_speed) as usize]
                == 0
            {
                self.player.position.y -= self.player.direction.y * movement_speed;
            }
        }

        if is_key_down(KeyCode::W) {
            let movement_speed = -self.player.movement_speed * get_frame_time();

            if self.current_map
                [(self.player.position.x - self.player.direction.x * movement_speed) as usize]
                [self.player.position.y as usize]
                == 0
            {
                self.player.position.x -= self.player.direction.x * movement_speed;
            }

            if self.current_map[self.player.position.x as usize]
                [(self.player.position.y - self.player.direction.y * movement_speed) as usize]
                == 0
            {
                self.player.position.y -= self.player.direction.y * movement_speed;
            }
        }
    }

    pub fn change_screen_size(&mut self, screen_width: f32, screen_height: f32) {
        self.window_size.x = screen_width;
        self.window_size.y = screen_height;

        request_new_screen_size(screen_width, screen_height);
    }
}
