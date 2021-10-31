enum TrafficLight {
    Red {duration: i8},
    Yellow {duration: i8},
    Green {duration: i8}
}

trait Attribution {
  fn getDuration(&self) ->&i8;
}


impl Attribution for TrafficLight{
  fn getDuration(&self) ->&i8 {
    match self {
      TrafficLight::Red { duration } => {
        return duration;
      },
      TrafficLight::Yellow { duration } => {
        return duration;
      },
      TrafficLight::Green { duration } => {
        return duration;
      }
    }
  }
}


fn main() {

  //红灯3s
  let redLight = TrafficLight::Red{duration: 3};

  //黄灯2s
  let yellowLight = TrafficLight::Yellow{duration: 2};

  //绿灯4s
  let greenLight = TrafficLight::Green{duration: 4};

  //打印
  println!("Red light duration is : {}", redLight.getDuration());
  println!("Yellow light duration is : {}",   yellowLight.getDuration());
  println!("Green light duration is : {}", greenLight.getDuration());
}
