

// #[cfg(test)]
// mod test {
//     #[test]
//     fn check_initial_values() {        
//         let ph:super::PlayHead = super::PlayHead::new();
//         assert_eq!(ph.paused, true);
//         assert_eq!(ph.old_time, 0);
//     }
//     // #[should_panic]
//     #[test]
//     fn play() {        
//         let mut ph:super::PlayHead = super::PlayHead::new();
//         ph.play();
//         assert_eq!(ph.paused, false);
//         println!("{}",(ph.time_lapsed.elapsed().as_millis()));
//         // assert_ne!((ph.time_lapsed.elapsed().as_millis()), 0);
//     }
//     // #[should_panic]
//     #[test]
//     fn stop() {        
//         let mut ph:super::PlayHead = super::PlayHead::new();
//         ph.play();
//         assert_eq!(ph.paused, false);
//         let a = ph.time_lapsed.elapsed().as_millis();
//         // How to assert greater than
//         // debug_assert_ge!(a > 0 as u128);
//     }
   
// }//mod 
