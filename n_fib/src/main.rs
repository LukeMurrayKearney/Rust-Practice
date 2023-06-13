fn main() {
    const N:usize = 20; let mut fibs = [0;N];
    fibs[0] = 1; fibs[1] = 2;
    for i in 2..N {
        let _ind:usize = i;
        fibs[i] += fibs[i-1] + fibs[i-2];
    }
    println!("{}", fibs.last().unwrap());
    println!("{:?}", fibs);
}
