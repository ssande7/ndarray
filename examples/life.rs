#[macro_use]
extern crate ndarray;

use ndarray::{
    RcArray,
    Ix,
};

type Ix2 = (Ix, Ix);

const INPUT: &'static [u8] = include_bytes!("life.txt");
//const INPUT: &'static [u8] = include_bytes!("lifelite.txt");

const N: usize = 100;
//const N: usize = 8;

type Board = RcArray<u8, Ix2>;

fn parse(x: &[u8]) -> Board {
    // make a border of 0 cells
    let mut map = RcArray::from_elem(((N + 2) as Ix, (N + 2) as Ix), 0);
    let a: RcArray<u8, Ix> = x.iter().filter_map(|&b| match b {
        b'#' => Some(1),
        b'.' => Some(0),
        _ => None,
    }).collect();

    let a = a.reshape((N as Ix, N as Ix));
    map.slice_mut(s![1..-1, 1..-1]).assign(&a);
    map
}

// Rules
//
// 2 or 3 neighbors: stay alive
// 3 neighbors: birth
// otherwise: death

fn iterate(z: &mut Board, scratch: &mut Board) {
    // compute number of neighbors
    let mut neigh = scratch.view_mut();
    neigh.assign_scalar(&0);
    neigh += &z.slice(s![0..-2, 0..-2]);
    neigh += &z.slice(s![0..-2, 1..-1]);
    neigh += &z.slice(s![0..-2, 2..  ]);

    neigh += &z.slice(s![1..-1, 0..-2]);
    neigh += &z.slice(s![1..-1, 2..  ]);

    neigh += &z.slice(s![2..  , 0..-2]);
    neigh += &z.slice(s![2..  , 1..-1]);
    neigh += &z.slice(s![2..  , 2..  ]);

    // birth where n = 3 and z[i] = 0,
    // survive where n = 2 || n = 3 and z[i] = 1
    let mut zv = z.slice_mut(s![1..-1, 1..-1]);

    // this is autovectorized amazingly well!
    zv.zip_mut_with(&neigh, |y, &n| {
        *y = ((n == 3) || (n == 2 && *y > 0)) as u8
    });
}

fn turn_on_corners(z: &mut Board) {
    let (n, m) = z.dim();
    z[[1    , 1    ]] = 1;
    z[[1    , m - 2]] = 1;
    z[[n - 2, 1    ]] = 1;
    z[[n - 2, m - 2]] = 1;
}

fn render(a: &Board) {
    for row in a.inner_iter() {
        for &x in row {
            if x > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let mut a = parse(INPUT);
    let mut scratch = Board::zeros((N as Ix, N as Ix));
    let steps = 100;
    turn_on_corners(&mut a);
    for _ in 0..steps {
        iterate(&mut a, &mut scratch);
        turn_on_corners(&mut a);
        //render(&a);
    }
    render(&a);
    let alive = a.iter().filter(|&&x| x > 0).count();
    println!("After {} steps there are {} cells alive", steps, alive);
}
