fn main(){
    
    
    let numbers:Vec<i32>=vec![1,4,5,3,2];
    let mut largest:i32=numbers[0];
    for n in numbers{
        if n > largest{
            largest=n;
            
        }
        
        
    }
    
    
    
    println!("{}",largest);
}
