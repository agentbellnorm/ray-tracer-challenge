/*
Perlins original algorithm:
    https://mrl.cs.nyu.edu/~perlin/noise/

I looked a lot at this one as well:
    https://github.com/keijiro/PerlinNoise/blob/master/Assets/Perlin.cs
*/

pub fn noise2(mut x: f64, mut y: f64) -> f64 {
    let x_ = x.floor() as i64 & 0xff;
    let y_ = y.floor() as i64 & 0xff;

    x -= x.floor();
    y -= y.floor();

    let u = fade(x);
    let v = fade(y);

    let a = (get_perm(x_) + y_) & 0xff;
    let b = (get_perm(x_ + 1) + y_) & 0xff;

    lerp(
        v,
        lerp(u, grad2(get_perm(a), x, y), grad2(get_perm(b), x - 1.0, y)),
        lerp(
            u,
            grad2(get_perm(a + 1), x, y - 1.0),
            grad2(get_perm(b + 1), x - 1.0, y - 1.0),
        ),
    )
}

pub fn noise3(mut x: f64, mut y: f64, mut z: f64) -> f64 {
    let x_ = x.floor() as i64 & 0xff;
    let y_ = y.floor() as i64 & 0xff;
    let z_ = z.floor() as i64 & 0xff;
    x -= x.floor();
    y -= y.floor();
    z -= z.floor();

    let u = fade(x);
    let v = fade(y);
    let w = fade(z);

    let a = (get_perm(x_) + y_) & 0xff;
    let b = (get_perm(x_ + 1) + y_) & 0xff;
    let aa = (get_perm(a) + z_) & 0xff;
    let ba = (get_perm(b) + z_) & 0xff;
    let ab = (get_perm(a + 1) + z_) & 0xff;
    let bb = (get_perm(b + 1) + z_) & 0xff;

    lerp(
        w,
        lerp(
            v,
            lerp(
                u,
                grad3(get_perm(aa), x, y, z),
                grad3(get_perm(ba), x - 1.0, y, z),
            ),
            lerp(
                u,
                grad3(get_perm(ab), x, y - 1.0, z),
                grad3(get_perm(bb), x - 1.0, y - 1.0, z),
            ),
        ),
        lerp(
            v,
            lerp(
                u,
                grad3(get_perm(aa + 1), x, y, z - 1.0),
                grad3(get_perm(ba + 1), x - 1.0, y, z - 1.0),
            ),
            lerp(
                u,
                grad3(get_perm(ab + 1), x, y - 1.0, z - 1.0),
                grad3(get_perm(bb + 1), x - 1.0, y - 1.0, z - 1.0),
            ),
        ),
    )
}

fn get_perm(i: i64) -> i64 {
    PERM[i as usize]
}

static PERM: [i64; 257] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151,
];

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

fn grad2(hash: i64, x: f64, y: f64) -> f64 {
    let a = if (hash & 1) == 0 { x } else { -x };
    let b = if (hash & 2) == 0 { y } else { -y };
    a + b
}

fn grad3(hash: i64, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };

    let g = if h == 12 || h == 14 { x } else { z };
    let v = if h < 4 { y } else { g };

    (if (h & 1) == 0 { u } else { -u }) + (if (h & 2) == 0 { v } else { -v })
}
