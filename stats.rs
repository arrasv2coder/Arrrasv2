struct Playerstats{
  health : f64,
  body_dmg : f64,
  bulletspeed : f64,
  
}
impl Playerstats{
  fn logmax(&self) {
    let var = if self.health >= {1self.health} else {"1_f64"};
    var.log();
  }
}
//this part is just a test
fn main() {
  let mut stats=Playerstats {
    health:"2_f64",
    body_dmg:"1_f64",
    bulletspeed:"3_f64",
  };
  stats.logmax();
}
