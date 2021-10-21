use playhead::PlayHead;
#[test]
fn seq(){
    let mut ph = PlayHead::new();
    ph.play();
    
    let mut time_before_first_puase:u128=0;
    let mut time_after_first_puase:u128=0;
    let mut started_again:u128=0;

    for n in 1..100000000 {
        if n== 99999999{
            time_before_first_puase = ph.time();
            ph.pause();

            assert!(time_before_first_puase > 0);
            
            time_after_first_puase = ph.time();
            
            assert!(time_before_first_puase == time_after_first_puase);
            break;
        }
    }

    for n in 1..100000000 {
        if n== 99999999{
            assert!(time_after_first_puase == ph.time());
            ph.play();
            started_again = ph.time();
            break;
        }
    }
    for n in 1..100000000 {
        if n== 99999999{
            assert!(started_again < ph.time());

            break;
        }
    }
 
}
