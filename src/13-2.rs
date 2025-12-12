fn main(){
    let v = vec![1,2,3];
    
    for item in v.iter() {
        println!("iter item: {}", item);
    }
    let total: u32 = v.iter().sum();
    println!("total: {}", total);

    let v2: Vec<i32> = vec![1,2,3];
    let v3: Vec<i32> = v2.iter().map(|m| m + 1).collect();
    println!("v2: {:?}, v3: {:?}", v2, v3); 

    let f = vec![1,2,3,4,5,6,7,8,9,10];
    let ff: Vec<u32> = f.into_iter().filter(|i| *i > 5).collect();
    println!("ff: {:?}", ff);
}