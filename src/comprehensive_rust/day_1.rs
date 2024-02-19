pub fn fib(n: u32) -> u32 {
    return if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    };
}


pub fn collatz_length(mut n: i32) -> u32 {
    let mut steps: u32 = 1;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        steps += 1;
    }
    steps
}


pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for (line_index, line) in matrix.iter().enumerate() {
        for (column_index, value) in line.iter().enumerate() {
            transposed[column_index][line_index] = *value;
        }
    }
    transposed
}

pub fn magnitude(v: &[f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalize(v: &mut [f64; 3]) -> &mut [f64; 3] {
    let mgntd = magnitude(v);
    v[0] /= mgntd;
    v[1] /= mgntd;
    v[2] /= mgntd;
    v
}

#[derive(Debug, PartialEq)]
pub enum Event {
    FloorButtonPressed(i32),
    LobbyCallButtonPressed(i32, Direction),
    Arrived(i32),
    DoorOpened,
    DoorClosed,

}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

pub fn car_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

fn car_door_opened() -> Event {
    Event::DoorOpened
}

fn car_door_closed() -> Event {
    Event::DoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCallButtonPressed(floor, dir)
}

fn car_floor_button_pressed(floor: i32) -> Event {
    Event::FloorButtonPressed(floor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn test_collatz_length() {
        assert_eq!(collatz_length(3), 8);
        assert_eq!(collatz_length(11), 15);
    }

    #[test]
    fn test_transpose() {
        let matrix = [
            [101, 102, 103],
            [201, 202, 203],
            [301, 302, 303],
        ];
        let transposed = transpose(matrix);
        assert_eq!(
            transposed,
            [
                [101, 201, 301],
                [102, 202, 302],
                [103, 203, 303],
            ]
        );
    }

    #[test]
    fn test_magnitude() {
        let v = [3.0, 2.0, 5.0];
        let mgntd = 38.0_f64.sqrt();
        assert_eq!(magnitude(&v), mgntd);
    }

    #[test]
    fn test_normalize() {
        let mut v = [3.0, 2.0, 5.0];
        let mgntd = 38.0_f64.sqrt();
        let v_norm = [v[0] / mgntd, v[1] / mgntd, v[2] / mgntd];
        assert_eq!(*normalize(&mut v), v_norm);
    }

    #[test]
    fn test_car_arrived() {
        let floor = 3;
        assert_eq!(car_arrived(3), Event::Arrived(floor));
    }

    #[test]
    fn test_car_door_opened() {
        assert_eq!(car_door_opened(), Event::DoorOpened);
    }

    #[test]
    fn test_car_door_closed() {
        assert_eq!(car_door_closed(), Event::DoorClosed);
    }

    #[test]
    fn test_lobby_call_button_pressed() {
        let floor = 15;
        assert_eq!(lobby_call_button_pressed(floor, Direction::Up), Event::LobbyCallButtonPressed(floor, Direction::Up));
    }

    #[test]
    fn test_car_floor_button_pressed() {
        let floor = 2;
        assert_eq!(car_floor_button_pressed(floor), Event::FloorButtonPressed(floor));
    }
}