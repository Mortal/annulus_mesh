use std::ops::Add;
use std::io;
use std::io::{Write, Cursor, BufRead};
use std::process::{Command, Stdio};

/// A point in 2D.
#[derive(Copy, Clone, Debug)]
pub struct Point(pub f64, pub f64);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    /// Distance between two points.
    pub fn dist(self, y: Point) -> f64 {
        self.dist_sq(y).sqrt()
    }

    /// The square of the distance between two points.
    pub fn dist_sq(self, y: Point) -> f64 {
        (self.0 - y.0).powi(2) + (self.1 - y.1).powi(2)
    }
}

/// Compute the Delaunay triangulation of a set of points by running them through `qhull`.
///
/// # Panics
///
/// Panics if `qhull` command is not available or fails.
pub fn qhull_triangulation(ps: &[Point]) -> io::Result<Vec<Vec<usize>>> {
    let mut process = Command::new("qhull")
                          .stdin(Stdio::piped())
                          .stdout(Stdio::piped())
                          .arg("d")
                          .arg("i")
                          .spawn()?;

    if let Some(ref mut out) = process.stdin {
        writeln!(out, "2\n{}", ps.len()).unwrap();

        for &Point(x, y) in ps {
            writeln!(out, "{} {}", x, y).unwrap();
        }
    }

    // process.stdout.unwrap().read_to_string(&mut s);
    let output = process.wait_with_output().unwrap();

    let mut lines = Cursor::new(output.stdout).lines();

    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut res = vec![];

    for line in lines {
        let s = line.unwrap();
        let poly: Vec<usize> = s.split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect();
        res.push(poly);
    }

    assert_eq!(res.len(), n);
    Ok(res)
}
