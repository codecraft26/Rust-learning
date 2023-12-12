fn main(){ 

let mut mutable_string = String::from("Mutable ");
    mutable_string.push_str("String");
    mutable_string.push_str("nisha");
    
    
    
    println!("Mutable String: {}", mutable_string);
    
    
      let  mut numbers = [8, 2, 3, 4, 5];
      numbers.sort();
      
      
      numbers[0]=9;
        let size = numbers.len();
        
        println!("{}" ,size);
        
        
        
        
      
      for &number in & numbers{
          println!("Number{}",number)
      }
      
      
      
          let mut counter = 0;

    // Define the condition for the while loop
    while counter < 5 {
        // Code to be executed in each iteration
        println!("Counter: {}", counter);

        // Increment the counter
        counter += 1;
    }



        if counter == 5{
            println!("true");
        }
        
         
      
      
      
      
      
         let mut num: Vec<i32> = Vec::new();

    // Pushing elements into the vector
    num.push(1);
    num.push(2);
    num.push(3);
    
    
    
    for &n in &num{
        
        
        println!("{}",n);
    }
    
    
    let  mut ans=num.get(0);
    
    
    println!("{}",ans);
      
      
      
      
    
    
    
    }
