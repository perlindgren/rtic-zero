use rtic_zero::*;

// auto generated
static R1: Resource<u32, 1> = Resource::new(5);
static R2: Resource<u32, 2> = Resource::new(500);

fn inc(r: &mut Mutex<u32, 1>) {
    println!(
        "inc {}",
        r.lock(|v| {
            *v += 1;
            *v
        })
    );
}

fn dec(r: &mut Mutex<u32, 1>) {
    println!(
        "dec {}",
        r.lock(|v| {
            *v -= 1;
            *v
        })
    );
}

fn gen<const CEIL: u8>(r: &mut Mutex<u32, CEIL>) {
    println!(
        "gen {}",
        r.lock(|v| {
            *v -= CEIL as u32;
            *v
        })
    );
}

fn two(r1: &mut Mutex<u32, 1>, r2: &mut Mutex<u32, 1>) {
    println!("two");
    r1.lock(|v| {
        println!("v {}", *v);
        r2.lock(|v| {
            println!("v {}", *v);
        })
    })
}

mod some_external_code {
    use rtic_zero::*;
    pub fn gen<const CEIL: u8>(r: &mut Mutex<u32, CEIL>) {
        println!(
            "external gen {}",
            r.lock(|v| {
                *v -= CEIL as u32;
                *v
            })
        );
    }
}

fn main() {
    let mut m = unsafe { Mutex::new(&R1) };
    let mut m2 = unsafe { Mutex::new(&R2) };
    let f_array = [inc, dec, gen, some_external_code::gen];

    for f in f_array {
        f(&mut m);
    }

    gen(&mut m2);
    // two(&mut m, &mut m); // cannot point to the same Mutex proxy
}
