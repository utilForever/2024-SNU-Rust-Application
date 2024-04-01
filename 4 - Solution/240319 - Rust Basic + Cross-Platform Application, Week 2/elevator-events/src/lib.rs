#[derive(Debug)]
// An event in the elevator system that the controller must react to.
enum Event {
    // A button was pressed.
    ButtonPressed(Button),

    // The car has arrived at the given floor.
    CarArrived(Floor),

    // The car's doors have opened.
    CarDoorOpened,

    // The car's doors have closed.
    CarDoorClosed,
}

// A floor is represented as an integer.
type Floor = i32;

// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

// A user-accessible button.
#[derive(Debug)]
enum Button {
    // A button in the elevator lobby on the given floor.
    LobbyCall(Direction, Floor),

    // A floor button within the car.
    CarFloor(Floor),
}

// The car has arrived at the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elevator_events_normal() {
        assert_eq!(
            format!("{:?}", lobby_call_button_pressed(0, Direction::Up)),
            "ButtonPressed(LobbyCall(Up, 0))"
        );
        assert_eq!(
            format!("{:?}", lobby_call_button_pressed(3, Direction::Down)),
            "ButtonPressed(LobbyCall(Down, 3))"
        );
        assert_eq!(format!("{:?}", car_arrived(0)), "CarArrived(0)");
        assert_eq!(format!("{:?}", car_door_opened()), "CarDoorOpened");
        assert_eq!(
            format!("{:?}", car_floor_button_pressed(3)),
            "ButtonPressed(CarFloor(3))"
        );
        assert_eq!(format!("{:?}", car_door_closed()), "CarDoorClosed");
        assert_eq!(format!("{:?}", car_arrived(3)), "CarArrived(3)");
    }
}
