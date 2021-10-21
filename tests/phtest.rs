use playhead::PlayHead;
#[test]
fn start(){
    let ph:PlayHead = PlayHead::new();;
    let rt = ph.time();
    assert_eq!(rt,0);
}
//#[test]
// fn play_pause(){
//     let mut ph:PlayHead = PlayHead::new();;
//     ph.play();
//     let start_time = ph.time();
//     // let mut time_after_pause:u128 ;

//     let number = 10000; 
//     for num in 0..number { // change it to get range
//         if num == 1 {
//             ph.pause();
//         }
//     }
//     let time_after_pause = ph.time();
    
//     let time_after_pause_again = ph.time();
//     assert_eq!(time_after_pause,time_after_pause_again);
// }
