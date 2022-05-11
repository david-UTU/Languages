use std::cmp;

const MOVE_NORMAL_COST: i32 = 10;
const MOVE_DIAGONAL_COST: i32 = 14;

#[derive(Copy, Clone)]
struct Node {
  index: i32,
  parent_index: i32,
  p_diagonal: bool,
  traversable: bool,
  x: i32,
  y: i32,
  g: i32,
  h: i32,
  f: i32,
}

impl Node {
  fn new(index: i32, x: i32, y: i32) -> Node {
    Node {
      index: index,
      parent_index: -1,
      p_diagonal: false,
      traversable: true,
      x: x,
      y: y,
      g: 0,
      h: 0,
      f: 0,
    }
  }

  fn set_h(&mut self, goal: Node) {
    if !self.traversable {
      return;
    }

    let dx: i32 = (self.x - goal.x).abs();
    let dy: i32 = (self.y - goal.y).abs();

    self.h = MOVE_DIAGONAL_COST * cmp::max(dx, dy);
  }

  fn set_g_f(&mut self, parent: Node, diagonal: bool) {
    self.g = parent.g
      + if diagonal {
        MOVE_DIAGONAL_COST
      } else {
        MOVE_NORMAL_COST
      };
    self.f = self.g + self.h;
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}

struct Grid {
  width: i32,
  height: i32,
  nodes: Vec<Node>,
}

impl Grid {
  fn new(width: i32, height: i32, walls: Vec<i32>) -> Grid {
    let mut nodes = Vec::new();
    let mut index = 0;

    for y in 0..height {
      for x in 0..width {
        let mut node: Node = Node::new(index, x as i32, y as i32);

        if walls.contains(&index) {
          node.traversable = false;
        }

        nodes.push(node);
        index += 1;
      }
    }

    Grid {
      width: width,
      height: height,
      nodes: nodes,
    }
  }

  fn get_nodes(&self) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    for node in self.nodes.iter() {
      nodes.push(*node);
    }

    nodes
  }

  fn get_neighbors(&self, node: Node) -> Vec<Node> {
    let mut neighbors: Vec<Node> = Vec::new();
    let mut pos: u8 = 0;

    for y in (node.y - 1)..=(node.y + 1) {
      for x in (node.x - 1)..=(node.x + 1) {
        if x == node.x && y == node.y {
          continue;
        }

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
          let mut neighbor: Node = self.nodes[(y * self.width + x) as usize];

          if neighbor.traversable {
            if pos % 2 == 0 {
              neighbor.p_diagonal = true;
            }

            neighbors.push(neighbor);
          }
        }

        pos += 1;
      }
    }

    neighbors
  }

  fn get_path(&self, closed_nodes: Vec<Node>, mut node: Node) -> Vec<Node> {
    let mut path: Vec<Node> = vec![node];

    while node.parent_index != -1 {
      match closed_nodes.iter().find(|&n| n.index == node.parent_index) {
        Some(parent) => node = *parent,
        None => break,
      }

      path.push(node);
    }

    path.reverse();
    path
  }

  fn a_star(&self, mut start: Node, goal: Node) -> Vec<Node> {
    let mut open_nodes: Vec<Node> = vec![start];
    let mut closed_nodes: Vec<Node> = Vec::new();
    let mut current_node: Node;

    start.set_h(goal);
    start.f = start.g + start.h;

    while open_nodes.len() > 0 {
      open_nodes.sort_by(|x, y| x.f.cmp(&y.f));

      current_node = open_nodes.swap_remove(0);
      closed_nodes.push(current_node);

      if current_node.index == goal.index {
        return self.get_path(closed_nodes, current_node);
      }

      for neighbor in self.get_neighbors(current_node).iter_mut() {
        neighbor.parent_index = current_node.index;

        if !closed_nodes.contains(neighbor) {
          neighbor.set_g_f(current_node, neighbor.p_diagonal);

          match open_nodes.iter().find(|&n| n == neighbor) {
            Some(node) => {
              let mut open_neighbor: Node = *node;

              if neighbor.g < open_neighbor.g {
                open_neighbor.g = neighbor.g;
                open_neighbor.parent_index = neighbor.parent_index;
              }
            }
            None => open_nodes.push(*neighbor),
          }
        }
      }
    }

    Vec::new()
  }
}

fn main() {
  let grid: &mut Grid = &mut Grid::new(7, 7, vec![9, 16, 23, 30, 37, 44]);
  let nodes = grid.get_nodes();
  let path: Vec<Node> = grid.a_star(nodes[36], nodes[48]);

  let new_line = |x: i32| -> String {
    if x == grid.width - 1 {
      String::from("\n")
    } else {
      String::new()
    }
  };

  for node in nodes.iter() {
    if path.first() == Some(node) {
      print!("{:<3}{}", '\u{1F535}', new_line(node.x))
    } else if path.last() == Some(node) {
      print!("{:<3}{}", '\u{1F534}', new_line(node.x))
    } else if node.traversable {
      print!(
        "{:<3}{}",
        if path.contains(node) {
          String::from("\u{26AA}")
        } else {
          node.index.to_string()
        },
        new_line(node.x)
      );
    } else {
      print!("{:<3}{}", '\u{2B1B}', new_line(node.x));
    }
  }

  print!("\nPath: ");
  for (index, node) in path.iter().enumerate() {
    if index == path.len() - 1 {
      print!("{}\n", node.index);
    } else {
      print!("{} -> ", node.index);
    }
  }

  println!("Cost: {}", path.last().unwrap().f);
}