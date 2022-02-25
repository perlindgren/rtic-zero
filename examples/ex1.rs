use rtic_zero::export::*;

// auto generated
static R1: Resource<u32, 1> = Resource::new(5);
static R2A: Resource<u32, 2> = Resource::new(500);
static R2B: Resource<u32, 2> = Resource::new(1000);

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

fn two<const CEIL1: u8, const CEIL2: u8>(r1: &mut Mutex<u32, CEIL1>, r2: &mut Mutex<u32, CEIL2>) {
    println!("two");
    r1.lock(|v| {
        println!("v {}", *v);
        r2.lock(|v| {
            println!("v {}", *v);
        })
    })
}

mod some_external_code {
    use rtic_zero::export::*;
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
    let priority = unsafe { Priority::new(0) };
    let mut m1 = unsafe { Mutex::new(&R1, &priority) };
    let mut m2a = unsafe { Mutex::new(&R2A, &priority) };
    let mut m2b = unsafe { Mutex::new(&R2B, &priority) };
    let f_array = [inc, dec, gen, some_external_code::gen];

    for f in f_array {
        f(&mut m1);
    }

    two(&mut m1, &mut m2a); // cannot point to the same Mutex proxy
    println!("priority {}", priority.get());

    two(&mut m2a, &mut m2b); // cannot point to the same Mutex proxy
    println!("priority {}", priority.get());
}
