use sfml::window::{ Event, Style };
use sfml::graphics::{ RenderWindow, RenderTarget, Color, CircleShape, Transformable, Shape };
use rand::Rng;
use verlet::{ World, Ball };


mod verlet;


fn main() {
    let mut window = RenderWindow::new((800, 600),
        "Verlet Integration",
        Style::CLOSE,
        &Default::default());
    window.set_framerate_limit(60);


    let mut world = World::new();

    let mut rng = rand::thread_rng();

    let mut frame = 0;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } => {
                    match code {
                        sfml::window::Key::Space => {
                            let mut ball = Ball::new((200.0, 0.0), rng.gen_range(5.0..20.0));
                            ball.color = Color::rgb(frame, 0, 255 - frame);
                            world.add(ball)
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        window.clear(Color::rgb(127, 127, 127));
        let mut mask = CircleShape::new(1.0, 30);
        mask.set_origin((1.0, 1.0));
        mask.set_position((400.0, 300.0));
        mask.set_scale((250.0, 250.0));
        mask.set_fill_color(Color::BLACK);
        window.draw(&mask);

        world.render(&mut window);
        world.update(1.0 / 60.0);

        window.display();

        if frame == 255 {
            frame = 0
        } else {
            frame += 1
        }
    }
}
