fn main(){
let a:f64 = 450000.00;
let b:f64 = 1500000.00;
let c:f64 = 750000.00;
let d:f64 = 2850000.00;
let e:f64 = 250000.00;
let f:f64 = 0.2;

//calculating sum
let sum =a+b+c+d+e;
println!("The total sum is {}", sum);
let avg = sum*f;
println!("The average of the dataset is {}", avg);
}