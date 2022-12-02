use core::time;

extern crate libusb;

fn main() {
    // device identifiers
    let vendor_id = 0x048d;
    let product_id = 0xc955;

    let context = libusb::Context::new().unwrap();
    let mut keyboard: libusb::DeviceHandle;
    match context.open_device_with_vid_pid(vendor_id, product_id) {
        Some(dh) => {
            keyboard = dh;
            println!("select device OK");
        }
        None => {
            std::panic!("NO DEVICE FOUND");
        }
    };

    match keyboard.claim_interface(0x00) {
        Ok(result) => {
            println!("attached success");
            result
        }
        Err(e) => {
            std::panic!("ERROR:: {}", e);
        }
    };

    let mut l5k = L5K::new();
    let effect = Effect {
        effect_type: EffectType::Static,
        speed: EffectSpeed::Fast,
        brightness: Brightness::High,
        colors: [
            Color::white(),
            Color::white(),
            Color::white(),
            Color::white(),
        ],
        direction: Direction::LTR,
    };

    let buf = l5k.build(&effect);
    println!("Payload is {:?}", buf);

    // Write info
    let request_type: u8 = 0x21;
    let request: u8 = 0x9;
    let value: u16 = 0x03cc;
    let index: u16 = 0x00;
    let timeout: time::Duration = time::Duration::new(30, 0);

    let result = match keyboard.write_control(request_type, request, value, index, &buf, timeout) {
        Ok(_) => "ok",
        Err(e) => {
            println!("error is {}", e);
            "error"
        }
    };
    println!("result is {}", result);
}

struct L5K {
    data: [u8; 32],
}
impl L5K {
    fn build(&mut self, effect: &Effect) -> [u8; 32] {
        self.data[Address::EFFECT.index()] = effect.effect_type.value();
        self.data[Address::SPEED.index()] = effect.speed.value();
        self.data[Address::BRIGHTNESS.index()] = effect.brightness.value();

        self.data[Address::RED1.index()] = effect.colors[0].red;
        self.data[Address::GREEN1.index()] = effect.colors[0].green;
        self.data[Address::BLUE1.index()] = effect.colors[0].blue;

        self.data[Address::RED2.index()] = effect.colors[1].red;
        self.data[Address::GREEN2.index()] = effect.colors[1].green;
        self.data[Address::BLUE2.index()] = effect.colors[1].blue;

        self.data[Address::RED3.index()] = effect.colors[2].red;
        self.data[Address::GREEN3.index()] = effect.colors[2].green;
        self.data[Address::BLUE3.index()] = effect.colors[2].blue;

        self.data[Address::RED4.index()] = effect.colors[3].red;
        self.data[Address::GREEN4.index()] = effect.colors[3].green;
        self.data[Address::BLUE4.index()] = effect.colors[3].blue;

        let direction = effect.direction.value();
        self.data[Address::RTL.index()] = direction.0;
        self.data[Address::LTR.index()] = direction.0;

        self.data
    }
}

impl L5K {
    fn new() -> L5K {
        let mut l5k = L5K {
            data: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };

        l5k.data[Address::HEAD1.index()] = 0xCC;
        l5k.data[Address::HEAD2.index()] = 0x16;
        l5k.data[Address::EMPTY.index()] = 0x00;

        l5k
    }
}

enum Address {
    HEAD1,
    HEAD2,
    EFFECT,
    SPEED,
    BRIGHTNESS,
    RED1,
    BLUE1,
    GREEN1,
    RED2,
    BLUE2,
    GREEN2,
    RED3,
    BLUE3,
    GREEN3,
    RED4,
    BLUE4,
    GREEN4,
    EMPTY,
    RTL,
    LTR,
}
impl Address {
    fn index(&self) -> usize {
        match self {
            Address::HEAD1 => 0,
            Address::HEAD2 => 1,
            Address::EFFECT => 2,
            Address::SPEED => 3,
            Address::BRIGHTNESS => 4,
            Address::RED1 => 5,
            Address::BLUE1 => 6,
            Address::GREEN1 => 7,
            Address::RED2 => 8,
            Address::BLUE2 => 9,
            Address::GREEN2 => 10,
            Address::RED3 => 11,
            Address::BLUE3 => 12,
            Address::GREEN3 => 13,
            Address::RED4 => 14,
            Address::BLUE4 => 15,
            Address::GREEN4 => 16,
            Address::EMPTY => 17,
            Address::RTL => 18,
            Address::LTR => 19,
        }
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn red() -> Color {
        Color {
            red: 0xff,
            green: 0x00,
            blue: 0x00,
        }
    }
    fn green() -> Color {
        Color {
            red: 0x00,
            green: 0xff,
            blue: 0x00,
        }
    }
    fn blue() -> Color {
        Color {
            red: 0x00,
            green: 0x00,
            blue: 0xff,
        }
    }
    fn black() -> Color {
        Color {
            red: 0x00,
            green: 0x00,
            blue: 0x00,
        }
    }
    fn white() -> Color {
        Color {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
        }
    }
}
struct Effect {
    effect_type: EffectType,
    speed: EffectSpeed,
    brightness: Brightness,
    colors: [Color; 4],
    direction: Direction,
}

enum Direction {
    None,
    RTL,
    LTR,
}
impl Direction {
    fn value(&self) -> (u8, u8) {
        match self {
            Direction::None => (0x00, 0x00),
            Direction::RTL => (0x01, 0x00),
            Direction::LTR => (0x00, 0x01),
        }
    }
}
enum Brightness {
    Default,
    High,
    Low,
}
impl Brightness {
    fn value(&self) -> u8 {
        match self {
            Brightness::Default => 0x00,
            Brightness::High => 0x01,
            Brightness::Low => 0x02,
        }
    }
}
enum EffectSpeed {
    Default,
    Slowest,
    Slow,
    Fast,
    Fastest,
}
impl EffectSpeed {
    fn value(&self) -> u8 {
        match self {
            EffectSpeed::Default => 0x00,
            EffectSpeed::Slowest => 0x01,
            EffectSpeed::Slow => 0x02,
            EffectSpeed::Fast => 0x03,
            EffectSpeed::Fastest => 0x04,
        }
    }
}

enum EffectType {
    Static,
    Breath,
    Wave,
    HUE,
}
impl EffectType {
    fn value(&self) -> u8 {
        match self {
            EffectType::Static => 0x01,
            EffectType::Breath => 0x03,
            EffectType::Wave => 0x04,
            EffectType::HUE => 0x06,
        }
    }
}
