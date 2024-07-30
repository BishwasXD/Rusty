//C = 5/9(F – 32).

pub fn convert_temp(){
    //32 is p
    //set one of them to 0.0
    let celsius:f32 = 0.0;
    let fahrenheit:f32 = 1.0;
    if(celsius>0.0&& fahrenheit==0.0){
        let fahrenheit = ((9.0*celsius)/5.0)+32.0;
    println!("In Fahrenheit :{fahrenheit},In Celsius:{celsius}");
    }
    else if(celsius==0.0&& fahrenheit>0.0){
        let celsius = (5.0/9.0)*(fahrenheit - 32.0);
    println!("In Fahrenheit :{fahrenheit},In Celsius:{celsius}");
    }
    else{
        println!("Set one of the values to 0 and other greater than 0");
        println!("In Fahrenheit :{fahrenheit},In Celsius:{celsius}");
    }
   
}