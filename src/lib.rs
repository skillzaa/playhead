use std::time::Instant;
#[derive(Debug)]
pub struct PlayHead {
    old_time:u128,
    paused:bool,
    time_lapsed:Instant, 
}

impl PlayHead {
    pub fn new()->Self{
        PlayHead  {
            //it must start paused since "first play" or any "play after pause" is the same. 
            paused:true, 
            old_time: 0,
            time_lapsed :  Instant::now(),//start time
        }
    }
    /* 
    - There is no diff bw first play and play after pause 
    - Here we can just set paused to false and thats it
    - We can be in either paused mode or play mode, there are just these 2 states, there is no third state for play head.
    - The time is the time spent in play mode excluding the time spent in pause mode.
     */   
    pub fn play(&mut self)->bool{
        if self.paused == true{
            self.time_lapsed =  Instant::now();
            self.paused = false;
        }      
        true
    }
    pub fn time(&self)->u128{ 
        if self.paused == false{
            self.time_lapsed.elapsed().as_millis() + self.old_time                    
         }else {
            self.old_time
        } 
    }
    /**
     - pause fn is used to pause the counter and before that add the current time into old_time.
     - the lapsed_time will not be added again if pause is called twice since the old_time is updated only when pause is false. once it is true the code inside will not run. to make it false again we have to go through play.
     - here we can just set pause to true and nothing more. 
     */
    pub fn pause (&mut self)->bool{
        if self.paused == false{ 
            self.old_time += self.time_lapsed.elapsed().as_millis();
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
        let ph:super::PlayHead = super::PlayHead::new();
        assert_eq!(ph.paused, true);
        assert_eq!(ph.old_time, 0);
    }
    // #[should_panic]
    #[test]
    fn play() {        
        let mut ph:super::PlayHead = super::PlayHead::new();
        ph.play();
        assert_eq!(ph.paused, false);
        println!("{}",(ph.time_lapsed.elapsed().as_millis()));
        // assert_ne!((ph.time_lapsed.elapsed().as_millis()), 0);
    }
    // #[should_panic]
    #[test]
    fn stop() {        
        let mut ph:super::PlayHead = super::PlayHead::new();
        ph.play();
        assert_eq!(ph.paused, false);
        let a = ph.time_lapsed.elapsed().as_millis();
        // How to assert greater than
        // debug_assert_ge!(a > 0 as u128);
    }
   
}//mod 
