use zoon::*;
use rand::prelude::*;

zoons!{

    #[derive(Copy, Clone, Debug)]
    struct Car {
        x: f64,
        y: f64,
        speed: f64,
        color: Color,
        width: f64,
        height: f64,
    }

    impl Car {
        /// Pixels per second.
        /// _Note_:
        /// Optional feature "wasm-bindgen" has to be enabled for crate `rand` (otherwise it panics).
        fn generate_speed() -> f64 {
            thread_rng().gen_range(400., 800.)
        }
    
        fn generate_color() -> Color {
            let hue = thread_rng().gen_range(0, 360);
            hsl(hue, 80, 50)
        }
    }

    impl Default for Car {
        fn default() -> Self {
            let car_width = 120.;
            Self {
                x: -car_width,
                y: 100.,
                speed: Self::generate_speed(),
                color: Self::generate_color(),
                width: car_width,
                height: 60.,
            }
        }
    }

    #[model]
    fn car() -> Car {
        Car::default()
    }

    #[model]
    fn viewport_width() -> f64 {
        0
    }

    #[update]
    fn update_viewport_width(width: f64) {
        viewport_width().set(width);
    }

    #[update]
    fn move_car(animation_frame: AnimationFrame) {
        let delta = match animation_frame.timestamp_delta {
            Some(delta) if delta > 0. => delta,
            _ => return,            
        };
        let car = car();
        
        if car.map(|car| car.x) > viewport_width().inner() {
            // We don't see car anymore => back to start + generate new color and speed.
            car.set(Car::default)
        } else {
            // Move car at least 1px to the right
            car.update(|car| car.x += f64::max(1., delta / 1000. * car.speed));
        }
    }

    #[view]
    fn view() -> View {
        view![
            viewport::on_width_change(update_viewport_width),
            view::on_animation_frame(move_car),
            column![
                width!(fill()),
                sky(),
                road(),
                car(),
            ],
        ]
    }

    #[view]
    fn sky() -> El {
        el![
            width!(fill()),
            height!(170),
            background::color(color::blue().set_l(65)),
        ]
    } 

    #[view]
    fn road() -> El {
        el![
            width!(fill()),
            height!(20),
            background::color(color::gray()),
        ]
    } 

    #[view]
    fn car() -> Column {
        let car = car().inner();
        column![
            in_front(),
            width!(car.width),
            height!(car.height),
            move_right(car.x),
            move_down(car.y),
            windows(),
            body(),
        ]
    } 

    #[view]
    fn windows() -> El {
        let car = car().inner();
        el![
            background::color(color::white().set_a(50)),
            width!(car.width * 0.5),
            height!(car.height * 0.6),
            center_x(),
            border::rounded!(fully()),
        ]
    } 

    #[view]
    fn body() -> Row {
        let car = car().inner();
        let first_wheel_x = use_reset_state!(true, || car.width * 0.15);
        let second_wheel_x = use_reset_state!(true, || car.width * 0.6);
        row![
            background::color(car.color),
            width!(fill()),
            height!(car.height * 0.6),
            move_up(car.height * 0.1)
            border::rounded!(fully()),
            wheel(first_wheel_x),
            wheel(second_wheel_x),
        ]
    } 

    #[view]
    fn wheel(x: State<f64>) -> El {
        let car = car().inner();
        let x = x.inner();
        let wheel_radius = car.height * 0.4;
        el![
            background::color(color::black()),
            width!(wheel_radius),
            height!(wheel_radius),
            move_right(wheel_x),
            move_down(car.height * 0.05),
            border::rounded!(fully()),
        ]        
    } 
}

fn main() {
    start!(zoons)
}
