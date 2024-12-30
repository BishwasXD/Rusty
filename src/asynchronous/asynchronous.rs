
//async programming lets u to do work(which takes time) for eg making db request, server req,
//without blocking the whole program.
//why we need async function for, lets say we are making a db request to get some data, while we
//dont have data we cannot proceed further so we have to wait for that function before moving any
//further for that we use await keyword, and the function that pools db result is async
//


use std::{thread::sleep, time::Duration};

//tells tokio to run this function as async 
#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
pub async fn asynchronous(){
    println!("this is async fn ");
    //do_something().await // blocks the coming execution until do_something is resolved
    //tokio::spawn(do_something());//donot block the execution and did something is printed earlier,
    //this will execute on new thread for and operation will run parallely so did something will gets printed
    //first cause do_something takes 2 sec to execute

    //testing what will happen if we spawn thread 7 times:
    //the 7th task will be queued and it will only execute after 6th task is completed
    //we will see some noticeable delay when when a thread has to assigned for the 7th task
    //it gets assgined when one of the thread is freed so that will add delay of ~2sec
    //one interesting thing here is the task are not executed in the order they are called
    //it has to do something with how task are scheduled 
    //maybe tasks are collected first which means when i = 1 i == 0 task is not running but
    //collected by scheduler program of tokio
    for i in 0..7{
        tokio::spawn(do_something(i));
        println!("did something inside loop");
    }

    // println!("did something");
}
async fn do_something(idx:usize){
    sleep(Duration::from_millis(2000));
    println!("do_something with thread {}", idx + 1);
}

//Thread management: Tokio takes the control of managing the thread for async operation, initially
//we have decalred 6 threads are available for this execution, when we call tokio::spawn one of the
//six thread will execute the fn
