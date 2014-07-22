#[deriving(Show)]
pub struct Node<'a> {
  name: &'a str,
  children: Option<&'a [Node<'a>]>
}

impl<'a> Node<'a> {
  fn render_open(&self) -> String {
    String::new().append("<").append(self.name).append(">")
  }
  fn render_close(&self) -> String {
    String::new().append("</").append(self.name).append(">")
  }
  fn render_self(&self) -> String {
    self.render_open() + self.render_close()
  }
  fn render_children(&self) -> String {
    String::new()
  }
  fn render_self_and_children(&self) -> String {
    self.render_open() + self.render_children() + self.render_close()
  }
  fn render_all(&self) -> String {
    self.render_self_and_children()
  }
}

pub fn render_nodes(nodes: &[Node]) {
  for index in range(0, nodes.len()) {
    println!("{}", nodes[index]);
  }
}

fn main() {
  render_nodes([Node{name: "html", children: Some(&[Node{name: "body", children: None}])}]);
}
