use pyxel::{Pyxel, PyxelCallback};

pub struct App {
    x: i32,
    y: i32,
}

impl App {
    fn new() -> App {
        App { x: 0, y: 0 }
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count() < 60 {
            self.x += (pyxel.frame_count() % 2) as i32;
            self.y -= 1;
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        pyxel.pset(self.x, 20, 7);
        pyxel.rect(self.x + 10, 25, 15, 10, 8);
        pyxel.rectb(self.x + 15, 45, 15, 10, pyxel::COLOR_WHITE);

        pyxel.blt(0, 0, 0, 0, 0, 8, 8, None);
    }
}

pub fn main() {
    let mut pyxel = Pyxel::new(
        200,
        150,
        Some("Hello, Pyxel in Rust!"),
        None,
        None,
        None,
        None,
    );
    let mut app = App::new();

    pyxel.image(0).set(
        &mut pyxel,
        0,
        0,
        &[
            "00011000", "00010100", "00010010", "00010010", "00010100", "00010000", "01110000",
            "01100000",
        ],
    );

    pyxel.sound(0).set(
        &pyxel,
        "e2e2c2g1 g1g1c2e2 d2d2d2g2 g2g2rr c2c2a1e1 e1e1a1c2 b1b1b1e2 e2e2rr",
        "p",
        "6",
        "vffn fnff vffs vfnn",
        25,
    );

    pyxel.sound(1).set(
        &pyxel,
        "r a1b1c2 b1b1c2d2 g2g2g2g2 c2c2d2e2 f2f2f2e2 f2e2d2c2 d2d2d2d2 g2g2r r ",
        "s",
        "6",
        "nnff vfff vvvv vfff svff vfff vvvv svnn",
        25,
    );

    pyxel.sound(2).set(
        &pyxel,
        "c1g1c1g1 c1g1c1g1 b0g1b0g1 b0g1b0g1 a0e1a0e1 a0e1a0e1 g0d1g0d1 g0d1g0d1",
        "t",
        "7",
        "n",
        25,
    );

    pyxel.sound(3).set(
        &pyxel,
        "f0c1f0c1 g0d1g0d1 c1g1c1g1 a0e1a0e1 f0c1f0c1 f0c1f0c1 g0d1g0d1 g0d1g0d1",
        "t",
        "7",
        "n",
        25,
    );

    pyxel.sound(4).set(
        &pyxel,
        "f0ra4r f0ra4r f0ra4r f0f0a4r",
        "n",
        "6622 6622 6622 6422",
        "f",
        25,
    );

    pyxel.play(0, &[0, 1], true);
    pyxel.play(1, &[2, 3], true);
    pyxel.play(2, &[4], true);

    pyxel.run(&mut app);
}
