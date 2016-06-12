struct Game {
    elapsedTime:f64,
}

impl Game {
    fn tick(&self, time:f64){
        let mut elapsedTime = self.elapsedTime;
        elapsedTime += time;

    }
}