use std::thread;
use std::time::Duration;
pub fn concurrency() {
    //creating a thread
    //its not necessary use to use handle can write it from threads onwards to create a thread.
    let handle = thread::spawn(|| {
        //can use spawn(move||{}) to take ownership of variables used inside the
        //thread
        for i in 1..10 {
            println! {"Spawned Thread{}",i};
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println! {"Main Thread{}",i};

        thread::sleep(Duration::from_millis(1));
    }
    //without using handle , it will  to produce equal main threads and spwaned threads ,so use
    //handle to not create equal threads
    handle.join().unwrap();
}
