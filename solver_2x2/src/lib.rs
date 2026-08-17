//         +----------+----------+
//        /    0     /    1     /|
//       /          /          / |
//      +----------+----------+  |
//     /    3     /    2     /|1 +
//    /          /          / | /|
//   +----------+----------+  |/ |
//   |          |          |2 +  |
//   |     3    |     2    | /|5 +
//   |          |          |/ | /
//   +----------+----------+  |/
//   |          |          |6 +
//   |     7    |     6    | /
//   |          |          |/
//   +----------+----------+

// "default" state of cube is solved with white up, blue front, orange right
// each corner has an id, which is its "default" index in the cube array
// each corner also tracks orientation
// 0 is default orientation, 1 is twisted cw, 2 is wtisted ccw

pub mod assembler;
pub mod cube;
pub mod solver;


#[cfg(test)]
mod assembler_tests;
#[cfg(test)]
mod cube_tests;
#[cfg(test)]
mod solver_tests;
