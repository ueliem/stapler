#[deriving(Show)]
pub struct Node<'a> {
  name: &'a str,
  children: Option<&'a [&'a DomNode]>
}

#[deriving(Show)]
pub struct Html<'a> {
  name: &'a str,
  children: Option<&'a [&'a DomNode]>
}

impl<'a> DomNode for Node<'a> {
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

impl<'a> DomNode for Html<'a> {
  fn render_open(&self) -> String {
    String::new().append("<html").append(">")
  }
  fn render_close(&self) -> String {
    String::new().append("</html").append(">")
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

trait DomNode {
  fn render_open(&self) -> String;
  fn render_close(&self) -> String;
  fn render_self(&self) -> String;
  fn render_children(&self) -> String;
  fn render_self_and_children(&self) -> String;
  fn render_all(&self) -> String;
}

impl<'a> std::fmt::Show for &'a DomNode {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f.buf, "Hi")
  }
}

pub fn render_nodes(nodes: &[&DomNode]) {
  for index in range(0, nodes.len()) {
    println!("{}", nodes[index]);
  }
}

fn main() {
  render_nodes([Html{children: Some(&[Node{name: "body", children: None}])}]);
}
