use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a:usize = 42;
    let b:&[u8;10] = &B;
    let c:Box<[u8]> = Box::new(C);

    println!("a符号のない整数:");
    println!("場所 {:p}",&a);
    println!("サイズ {:?}byte",size_of::<usize>());
    println!("値 {:?}",a);

    println!("b(Bへの参照):");
    println!("場所 {:p}",&b);
    println!("サイズ {:?}byte",size_of::<&[u8;10]>());
    println!("値 {:?}",b);

    println!("c(Cを入れたBox):");
    println!("場所 {:p}",&c);
    println!("サイズ {:?}byte",size_of::<Box<[u8]>>());
    println!("値 {:?}",c);

    println!("10byteの配列:");
    println!("場所 {:p}",&B);
    println!("サイズ {:?}byte",size_of::<[u8;10]>());
    println!("値 {:?}",B);

    println!("C(11bbyteの配列):");
    println!("場所 {:p}",&C);
    println!("サイズ {:?}byte",size_of::<[u8;11]>());
    println!("値 {:?}",C);
}