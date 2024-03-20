#[derive(Debug)]
// An event in the elevator system that the controller must react to.
enum Event {
    // TODO: add required variants
}

// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

// The car has arrived at the given floor.
fn car_arrived(floor: i32) -> Event {
    todo!()
}

// The car doors have opened.
fn car_door_opened() -> Event {
    todo!()
}

// The car doors have closed.
fn car_door_closed() -> Event {
    todo!()
}

// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    todo!()
}

// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    todo!()
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
