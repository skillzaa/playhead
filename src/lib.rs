use std::time::Instant;
#[derive(Debug)]
pub struct PlayHead {
    duration:u128,
    old_time:u128,
    paused:bool,
    instant:Instant, 
}

impl PlayHead {
    pub fn new(duration:u128,paused:bool)->Self{
        PlayHead  {
            duration,
            paused,
            old_time: 0,
            instant :  Instant::now(),
        }
    }
    pub fn start(&mut self)->bool{
        if self.paused == true{
            self.instant =  Instant::now();
            // self.old_time += self.instant.elapsed().as_millis();
            self.paused = false;
        }      
        true
    }
    pub fn play(&mut self)->bool{
        if self.paused == true{
            self.instant =  Instant::now();
            // self.old_time += self.instant.elapsed().as_millis();
            self.paused = false;
        }      
        true
    }
    pub fn time(&self)->u128{ 
        if self.paused == false{
            self.instant.elapsed().as_millis() + self.old_time                    
         }else {
            self.old_time
        } 
    }
    pub fn pause (&mut self)->bool{
        if self.paused == false{ 
            // update old_time on each pause
            self.old_time += self.instant.elapsed().as_millis();//store time already ran
            self.paused = true;
        }
           true   
    }
    pub fn stop(&mut self)->bool{
        self.paused = true;
        self.old_time = 0;
        true
    }

}//playHead impl ends


#[cfg(test)]
mod test {
    #[test]
    fn check_initial_values() {        
        let ph:super::PlayHead = super::PlayHead::new(100, true);
        assert_eq!(ph.paused, true);
        assert_eq!(ph.old_time, 0);
        assert!(ph.duration > 0);
    }
    // #[should_panic]
    #[test]
    fn play() {        
        let mut ph:super::PlayHead = super::PlayHead::new(100, true);
        ph.start();
        assert_eq!(ph.paused, false);
        println!("{}",(ph.instant.elapsed().as_millis()));
        // assert_ne!((ph.instant.elapsed().as_millis()), 0);
    }
    // #[should_panic]
    #[test]
    fn stop() {        
        let mut ph:super::PlayHead = super::PlayHead::new(100, true);
        ph.start();
        assert_eq!(ph.paused, false);
        let a = ph.instant.elapsed().as_millis();
        // How to assert greater than
        // debug_assert_ge!(a > 0 as u128);
    }
   
}//mod test
