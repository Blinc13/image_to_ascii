use image::io::Reader;


fn main() {
    let x = Reader::open("/home/blinc/Загрузки/rei_p.jpg").unwrap();

    println!("{:?}", x.format());
    let x = x.decode().unwrap();
    println!("{}", x.height());
}
