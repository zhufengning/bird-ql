use sfml::{
    graphics::{
        RenderTarget, RenderWindow, Color, Texture, Sprite, Transformable, Text, Font, RectangleShape, Shape
    },
    window::{
        ContextSettings, Style, Event, Key
    },
    system::{
        Clock
    },
};
use rand::Rng;
use std::io;

#[derive(Debug)]
enum Game {
    Start,
    Running,
    Died
}

#[derive(Copy, Clone, Debug)]
struct Node {
    x:usize,
    y:usize,
    jp:bool
}

fn main() {
    let b:u32 = 1;

    let mut rng = rand::thread_rng();

    let mut q:[[[f32;2];1285];365] = [[[0.;2];1285];365];
    let keepr = 1;
    let deadr = -100;
    let passr = 15;
    let qa:f32 = 0.6;
    let qg:f32 = 0.8;
    let mut qx:i32;
    let mut qy:i32;
    let mut qlst:i32 = 2;
    let qac:usize = 1;
    let mut enq = false;

    let mut choise:Node;
    let mut i = 0;
    let mut j;
    while i <= 360 {
        j = 0;
        while j <= 1280 {
            q[i][j][0] = 0.;//rng.gen_range(-100.. 100) as f32;
            q[i][j][1] = 0.;//rng.gen_range(-100.. 100) as f32;
            j += 1;
        }
        i += 1;
    }

    let context_settings = ContextSettings::default();

    let font = Font::from_file("src/手书体.ttf").unwrap();

    let mut w = RenderWindow::new((360, 640), "bird", Style::CLOSE, &context_settings);
    let sfps:u32 = 40;
    let mut sfr:u32 = 1;
    w.set_framerate_limit(sfps * sfr);

    let bird1 = Texture::from_file("src/bird1.png").unwrap();
    let mut bird_speed_x:f32 = 0.;
    let mut bird_speed_y:f32 = 0.;
    let mut bird_acc:f32 = 0.;
    let mut bird_x:f32 = 180.;
    let mut bird_y:f32 = 320.;
    let mut bird = Sprite::new();
    let jumps:f32 = 0.5;
    let jumpa:f32 = 0.002;
    bird.set_origin((23.5, 19.5));
    bird.set_position((bird_x, bird_y));
    bird.set_texture(&bird1, false);

    let mut wall_first_up = RectangleShape::new();
    let mut wall_first_h:f32 = rng.gen_range(160.. 240) as f32;
    let wall_w:f32 = 80.;
    let  wall_speed_x:f32 = -0.1;
    let mut wall_first_x = 360.;
    wall_first_up.set_size((wall_w, wall_first_h));
    wall_first_up.set_fill_color(Color::MAGENTA);
    wall_first_up.set_position((wall_first_x, 0.));

    let mut wall_first_down = RectangleShape::new();
    let wall_dis:f32 = 200.;
    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
    wall_first_down.set_fill_color(Color::MAGENTA);
    wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));


    let mut wall_second_up = RectangleShape::new();
    let mut wall_second_h:f32 = rng.gen_range(0.. 640 - wall_dis as i32) as f32;
    let mut wall_second_x = 580.;
    wall_second_up.set_size((wall_w, wall_second_h));
    wall_second_up.set_fill_color(Color::MAGENTA);
    wall_second_up.set_position((wall_second_x, 0.));

    let mut wall_second_down = RectangleShape::new();
    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
    wall_second_down.set_fill_color(Color::MAGENTA);
    wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));

    qx = (wall_first_x - bird_x + wall_w) as i32;
    qy = (bird_y - wall_first_h) as i32;
    choise = Node{x: qx as usize, y: (qy + 640) as usize, jp: false};

    let mut statu = Game::Start;

    let mut fps = 0;
    let mut fpsc = 0;
    let mut fps_txt = Text::new("", &font, 24);

    let mut maxc = 0;
    let mut cnt = 0;
    let mut lcnt = 0;
    let mut lst = 2;
    let mut cnt_txt = Text::new("", &font, 36);
    cnt_txt.set_fill_color(Color::BLACK);

    let mut clock2 = Clock::start();//好像是算帧率用的


    while w.is_open() {
        while let Some(event) = w.poll_event() {
            match event {
                Event::Closed => w.close(),
                Event::KeyPressed{
                    code : Key::Space, ..
                } => {
                    match statu {
                        Game::Running => {
                            if !enq {
                                bird_speed_y = -jumps;
                            }
                        },
                        Game::Start => {
                            bird_acc = jumpa;
                            wall_first_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                            wall_first_x = 360.;
                            wall_first_up.set_size((wall_w, wall_first_h));
                            wall_first_up.set_fill_color(Color::MAGENTA);
                            wall_first_up.set_position((wall_first_x, 0.));
                            wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                            wall_first_down.set_fill_color(Color::MAGENTA);
                            wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));
                            wall_second_x = 580.;
                            wall_second_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                            wall_second_up.set_size((wall_w, wall_second_h));
                            wall_second_up.set_fill_color(Color::MAGENTA);
                            wall_second_up.set_position((wall_second_x, 0.));
                            wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                            wall_second_down.set_fill_color(Color::MAGENTA);
                            wall_second_down.set_position((wall_second_x, 160. + wall_second_h));
                            cnt = 0;
                            lst = 2;
                            statu = Game::Running;
                        },
                        _ => ()
                    }
                },
                Event::KeyPressed{
                    code : Key::R, ..
                } => {
                    if let Game::Died = statu {
                        bird_acc = jumpa;
                        wall_first_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                        wall_first_x = 360.;
                        wall_first_up.set_size((wall_w, wall_first_h));
                        wall_first_up.set_fill_color(Color::MAGENTA);
                        wall_first_up.set_position((wall_first_x, 0.));
                        wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                        wall_first_down.set_fill_color(Color::MAGENTA);
                        wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));
                        wall_second_x = 580.;
                        wall_second_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                        wall_second_up.set_size((wall_w, wall_second_h));
                        wall_second_up.set_fill_color(Color::MAGENTA);
                        wall_second_up.set_position((wall_second_x, 0.));
                        wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                        wall_second_down.set_fill_color(Color::MAGENTA);
                        wall_second_down.set_position((wall_second_x, 160. + wall_second_h));
                        cnt = 0;
                        lst = 2;
                        statu = Game::Running;
                    }
                },
                Event::KeyPressed {
                    code: Key::D, ..
                } => {
                    let mut i:usize = 0;
                    let mut j:usize;
                    while i <= 360 / qac {
                        j = 0;
                        let mut num = String::new();
                        io::stdin().read_line(&mut num).unwrap();
                        let mut num = num.trim().split(' ');
                        while j <= 1280 / qac {
                            q[i][j][0] = num.next().unwrap().parse().unwrap();
                            q[i][j][1] = num.next().unwrap().parse().unwrap();
                            
                            j += 1;
                        }
                        i += 1;
                    }
                },
                Event::KeyPressed {
                    code: Key::X, ..
                } => {
                    print!("\n\n\n\n\n");
                    let mut i:usize = 0;
                    let mut j:usize;
                    while i <= 360 / qac {
                        j = 0;
                        while j <= 1280 / qac {
                            print!("{} ", q[i][j][0]);
                            print!("{} ", q[i][j][1]);                            
                            j += 1;
                        }
                        print!("\n");
                        i += 1;
                    }
                },
                Event::KeyPressed{
                    code : Key::Up, ..
                } => {
                    sfr += 1;
                    if sfr > 4 {
                        sfr -= 1;
                    }
                    w.set_framerate_limit(sfr * sfps);
                },
                Event::KeyPressed{
                    code : Key::Down, ..
                } => {
                    if sfr > 0 {
                        sfr -= 1;
                        w.set_framerate_limit(sfr * sfps);
                    }
                },
                Event::KeyPressed {
                    code: Key::Q, ..
                } => {
                    enq = !enq;
                },
                _ => {}
            }
        }
        w.clear(Color::WHITE);

        let etime = (1000 / sfps * b) as f32;
        match statu {
            Game::Start => {
                w.draw(&bird);
            },
            Game::Running => {
                let rew:i32;

                bird_x += bird_speed_x * etime;
                bird_y += bird_speed_y * etime + 0.5 * bird_acc * etime * etime;
                bird_speed_y += bird_acc * etime;
                bird.set_position((bird_x, bird_y));
                if bird_speed_y > 0. {
                    bird.set_rotation(20.);
                } else if bird_speed_y == 0. {
                    bird.set_rotation(0.);
                } else {
                    bird.set_rotation(-20.);
                }

                wall_first_x += wall_speed_x * etime;
                wall_first_up.set_position((wall_first_x, 0.));
                wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));

                wall_second_x += wall_speed_x * etime;
                wall_second_up.set_position((wall_second_x, 0.));
                wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));

                if wall_first_x <= -80. {
                    wall_first_x = 360.;
                    wall_first_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                    wall_first_up.set_size((wall_w, wall_first_h));
                    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                }

                if wall_second_x <= -80. {
                    wall_second_x = 360.;
                    wall_second_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                    wall_second_up.set_size((wall_w, wall_second_h));
                    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                }

                if lst == 1 {
                    if bird_x > wall_second_x + wall_w {
                        cnt += 1;
                        lst = 2;
                    }
                } else {
                    if bird_x > wall_first_x + wall_w {
                        cnt += 1;
                        lst = 1;
                    }
                }
                if cnt > maxc {
                    maxc = cnt;
                }
                if qlst == 1 {
                    if bird_x > wall_second_x + wall_w {
                        qlst = 2;
                        qx = (wall_first_x - bird_x + wall_w) as i32;
                        qy = (bird_y - wall_first_h) as i32;
                    } else {
                        qx = (wall_second_x - bird_x + wall_w) as i32;
                        qy = (bird_y - wall_second_h) as i32;
                    }
                } else {
                    if bird_x > wall_first_x + wall_w {
                        qlst = 1;
                        qx = (wall_second_x - bird_x + wall_w) as i32;
                        qy = (bird_y - wall_second_h) as i32;
                    } else {
                        qx = (wall_first_x - bird_x + wall_w) as i32;
                        qy = (bird_y - wall_first_h) as i32;
                    }
                }

                if bird_x >= wall_first_x && bird_x <= wall_first_x + wall_w {
                    if bird_y <= wall_first_h || bird_y >= wall_first_h + wall_dis {
                        statu = Game::Died;
                    }
                }
                if bird_x >= wall_second_x && bird_x <= wall_second_x + wall_w {
                    if bird_y <= wall_second_h || bird_y >= wall_second_h + wall_dis {
                        statu = Game::Died;
                    }
                }

                if bird_y > 640. || bird_y < 0. {
                    statu = Game::Died;
                }

                cnt_txt.set_string(&format!("{}", cnt));
                cnt_txt.set_position((180. - cnt_txt.character_size() as f32 / 2., 0.));

                w.draw(&bird);
                w.draw(&wall_first_up);
                w.draw(&wall_first_down);
                w.draw(&wall_second_up);
                w.draw(&wall_second_down);
                w.draw(&cnt_txt);

                if enq {
                    let i = qx as usize / qac;
                    let j = (qy + 640) as usize / qac;
                    if let Game::Died = statu {
                        rew = deadr;
                    }
                    else if lcnt < cnt {
                        lcnt = cnt;
                        rew = passr;
                    }
                    else {
                        rew = keepr;
                    }
                    let typ:usize = choise.jp as usize;
                    q[choise.x][choise.y][typ] 
                    = (1. - qa) * q[choise.x][choise.y][typ] 
                    + qg * (rew as f32 + 0.5 * (
                        if j > 1280 {
                            0.
                        } else if q[i][j][0] > q[i][j][1] {
                            q[i][j][0]
                        } else {
                            q[i][j][1]
                        }));

                    if let Game::Died = statu {
                        continue;
                    }
                    if rng.gen_range(1.. 50) == 51 {
                        //jump
                        bird_speed_y = -jumps;
                        choise = Node{x:i, y:j, jp:true};
                    }
                    else {
                        if q[i][j][0] < q[i][j][1] {
                            bird_speed_y = -jumps;
                            choise = Node{x:i, y:j, jp:true};
                        }
                        else {
                            choise = Node{x:i, y:j, jp:false};
                        }
                    }
                }
            },
            Game::Died => {
                bird_x = 180.;
                bird_y = 320.;
                bird.set_rotation(0.);
                bird_speed_y = 0.;
                bird_speed_x = 0.;
                bird.set_position((bird_x, bird_y));
                w.draw(&bird);
                w.draw(&cnt_txt);
                if enq {
                    bird_acc = jumpa;
                    wall_first_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                    wall_first_x = 360.;
                    wall_first_up.set_size((wall_w, wall_first_h));
                    wall_first_up.set_fill_color(Color::MAGENTA);
                    wall_first_up.set_position((wall_first_x, 0.));
                    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                    wall_first_down.set_fill_color(Color::MAGENTA);
                    wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));
                    wall_second_x = 580.;
                    wall_second_h = rng.gen_range(0.. 640 - wall_dis as i32 + 1) as f32;
                    wall_second_up.set_size((wall_w, wall_second_h));
                    wall_second_up.set_fill_color(Color::MAGENTA);
                    wall_second_up.set_position((wall_second_x, 0.));
                    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                    wall_second_down.set_fill_color(Color::MAGENTA);
                    wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));
                    cnt = 0;
                    lcnt = 0;
                    lst = 2;
                    qlst = 2;
                    qx = (wall_first_x - bird_x + wall_w) as i32;
                    qy = (bird_y - wall_first_h) as i32;
                    choise = Node{x: qx as usize, y: (qy + 640) as usize, jp: false};
                    statu = Game::Running;
                }
            } 
        }
        fpsc += 1;
        let etime2 = clock2.elapsed_time().as_microseconds();
        if etime2 > 1000000 {
            clock2.restart();
            fps = fpsc;
            fpsc = 0;
        }

        fps_txt.set_string(&format!("fps:{}\nMax score:{}\nQ-Learning:{}\n{:?}", fps, maxc, enq, statu));
        fps_txt.set_fill_color(Color::BLACK);
        fps_txt.set_position((0., 0.));
        w.draw(&fps_txt);
        w.display();

    }
}
