fn main() {
    let a = [9, 8, 7, 6, 5];
    println!("a[0]={}",a[0]);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b={:?}",b);

    let c = [3; 5];
    println!("c={:?}",c);

    let d = [[3; 3]; 4]; // 二维数组
    println!("d={:?}",d);
    for row in &d {
        // println!("row={:?}",row);

        // for n in row.iter() {
        //     print!("\t{} ", n);
        // }
        
        for i in 0..row.len() {
            print!("\t{} ", row[i])
        }
        println!()
    }
}
