use std::thread;
use std::time;

fn main() {
    let (mut a, mut b): (f64, f64) = (0.0, 0.0);

    loop {

        a += 0.07;
        b += 0.03;

        let ((sin_a, cos_a),
               (sin_b, cos_b),
                    mut b) = (a.sin_cos(), b.sin_cos(), [' '; 1760]);

        let (mut z, mut j): ([f64; 1760], f64) =([0.0; 1760], 0.0);

        while j <= 6.28 {

            let (u, v) = j.sin_cos();
            let mut i: f64 = 0.0;

            while i <= 6.28 {
                let (w, c) = i.sin_cos();
                let h = v + 2.0;
                let (d, t) = (1.0 / (w * h * sin_a + u * cos_a + 5.0),
                                        w * h * cos_a - u * sin_a);

                let (x, y) = ((40.0 + 30.0 * d * (c * h * cos_b - t * sin_b)) as usize,
                    (12.0 + 15.0 * d * (c * h * sin_b + t * cos_b)) as usize);

                let (o, n) = (x + 80 * y, 8.0 * ((u * sin_a - w * v * cos_a)
                                        * cos_b - w * v * sin_a - u * cos_a - c * v * sin_b));

                if y < 22 && x < 79 && d > z[o] {
                    z[o] = d;
                    b[o] = (".,-~:;=".to_owned() + "!*#$@")
                        .chars()
                        .nth(n as usize)
                        .or(Some('.'))
                        .unwrap();
                }
                i += 0.02
            }
            j += 0.07
        }
        print!(
            "\x1B[H{}",
            b.chunks(80)
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
        thread::sleep(time::Duration::from_millis(20));
    }
}