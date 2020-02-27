#[derive(PartialEq, Debug, Clone)]
struct LedColor(i32, i32, i32); // (1) Create a tuple-struct to hold the three-color codes

const SENSOR_BATTERY: i32 = 20;

const OFF:LedColor =  LedColor(0, 0, 0);
const YELLOW: LedColor = LedColor(255, 255, 0);
const GREEN: LedColor = LedColor(0, 255, 0);
const RED: LedColor = LedColor(255, 0, 0);
const PCT_PER_PIXEL: f64 = 12.5_f64;


extern "C" {
    fn set_led(led_index: i32, r: i32, g: i32, b: i32); // (2) Import the set_led function from our host
}


#[no_mangle]
pub extern "C" fn sensor_update(sensor_id: i32, sensor_value: f64) -> f64 { // (3) Expose the sensor_update and apply functions to the host
    if sensor_id == SENSOR_BATTERY {
        set_leds(get_led_values(sensor_value));
    }
    sensor_value
}

#[no_mangle]
pub extern "C" fn apply(_frame: u32) {
    // NO OP, not an animated indicator
}


fn get_led_values(battery_remaining: f64) -> [LedColor; 8] { // (4) Core logic to convert a percentage into a set of eight color codes
    let mut arr: [LedColor; 8] = [OFF,OFF,OFF,OFF,OFF,OFF,OFF,OFF,];
    let lit = (battery_remaining / PCT_PER_PIXEL).ceil();

    // 0 - 20 : Red
    // 21 - <50 : Yellow
    // 51 - 100 : Green

    let color = if 0.0 <= battery_remaining &&
        battery_remaining <= 20.0 {
        RED
    } else if battery_remaining > 20.0 && battery_remaining < 50.0 {
        YELLOW
    } else {
        GREEN
    };

    for idx in 0..lit as usize {
        arr[idx] = color.clone();
    }

    arr
}

fn set_leds(values: [LedColor; 8]) { // (5) Invoke the unsafe import in a loop to set all the LED colors on the host
    for x in 0..8 {
        let LedColor(r, g, b) = values[x];
        unsafe {
            set_led(x as i32, r,g,b);
        }
    }
}
#[cfg(test)] // to do the unit test for this lib
mod tests {

    use {OFF, YELLOW, RED, GREEN, get_led_values};

    #[test]
    fn test_0_pct() {
        assert_eq!(get_led_values(0.0),
            [OFF,OFF,OFF,OFF,OFF,OFF,OFF,OFF,]);
    }

    #[test]
    fn test_15_pct() {
        assert_eq!(get_led_values(15.0),
            [RED, RED, OFF, OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn test_49_pct() {
        assert_eq!(get_led_values(49.0),
            [YELLOW, YELLOW, YELLOW, YELLOW, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn test_75_pct() {
        assert_eq!(get_led_values(75.0),
            [GREEN,GREEN,GREEN,GREEN,GREEN,GREEN,OFF,OFF,]);
    }

    #[test]
    fn test_100_pct() {
        assert_eq!(get_led_values(100.0),
            [GREEN,GREEN,GREEN,GREEN,GREEN,GREEN,GREEN,GREEN,]);
    }
}