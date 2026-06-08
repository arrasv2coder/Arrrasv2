struct Playerstats{
  health : f64,
  body_dmg : f64,
  bulletspeed : f64,
  
}
impl Playerstats{
  fn logmax(&self) {
    let var = self.health;
    var.log();
  }
}
