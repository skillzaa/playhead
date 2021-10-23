
# PlayHead

> PlayHead is a very simple and small **counter** utility with *play* , *pause* and *stop* function. At any give time the lapsed time can be obtained from *time* function. 

- The main purpose of making this module is for educational purposes. I will maintain this completely but without adding any more functionalities.
- It has no dependencies.
- The lapsed time is returned in milli seconds.

# Usage
Please see the documentation for further details but in brief:

## To create a new Timer 
##### the name of the timer is called play head 
> let playhead  = PlayHead::new();
---
## To Start 
> playhead.start();
---
## To Stop 
> playhead.stop();
---
## To Pause 
> playhead.pause();
---
## To check the lapsed Time 
> playhead.time();


