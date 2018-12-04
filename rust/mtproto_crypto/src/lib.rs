extern crate libc;
extern crate rand;

mod string;

use string::StringPtr;
use rand::Rng;

// string ffi

#[no_mangle]
pub unsafe extern fn rust_string_ptr(s: *mut String) -> *mut StringPtr {
  Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}


#[no_mangle]
pub unsafe extern fn mygcd(mut a: u64, mut b:u64) -> u64 {
    
    while a != 0 && b != 0 {
        while (b & 1) == 0 {
            b = b.clone() >> 1;
        }
        
        while (a & 1) == 0 {
            a = a.clone() >> 1;
        }
        
        if a > b {
            a = a.clone() - b;
        } else {
            b = b.clone() - a;
        }
    }

    if b == 0 {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern fn mt_factorize(what: u64) -> Vec<u64> {
    let mut it = 0;
    let mut i = 0;
    let mut g: u64 = 0;

    while i < 3 || it < 1000 {
        let mut x = rand::random::<u64>() % (what - 1) + 1;
        let mut y = x;

        let q = ((rand::random::<u64>() & 0xF) + 17) % what;
        let lim = 1 << (i + 18);

        for j in 1..lim {
            it += 1;
            let mut a = x;
            let mut b = x;
            let mut c = q;

            while b != 0 {
                if b & 1 != 0{
                    c += a;
                    if c >= what {
                        c -= what;
                    }
                }

                a += a;

                if a >= what {
                    a -= what;
                }
                b >>= 1;
            }

            x = c;

            let mut z = if x < y {
                what + x - y
            } else {
                x - y
            };

            g = mygcd(z, what);

            if g != 1 {
                break;
            }

            if (j & (j - 1)) == 0 {
                y = x;
            }
        }
        
        if g > 1 && g < what {
            break;
        }
            
        i += 1;
    }
    
    if g > 1 && g < what {
        let mut p1 = g;
        let mut p2 = what / g;
        if p1 > p2 {
            let tmp = p1;
            p1 = p2;
            p2 = tmp;
        }
        
        vec![p1, p2]
    } else {
        vec![]
    }
}

#[no_mangle]
pub unsafe extern fn test_factorization() -> String {
//  let pq = 0x17ED48941A08F981;
//  let answer = mt_factorize(pq);//primes::factors(pq);//factors(pq);
//  let result = format!("p = {:#X}, q = {:#X}", answer[0], answer[1]);
//  result

  let mut pq: u64 = 0;
/*
  loop {
    let mut temprnd = rand::thread_rng().gen_range(0xFFFFFFFusize, 0xFFFFFFFFusize);
    let p = primapalooza::get_next_prime_number(temprnd);
    temprnd = rand::thread_rng().gen_range(0xFFFFFFFusize, 0xFFFFFFFFusize);
    let q = primapalooza::get_next_prime_number(temprnd);
    pq = (p as u64) * (q as u64);

    if (pq >> 63) == 0 {
        break;
    }
  }
*/
    
  let answer = mt_factorize(pq);//primapalooza::prime_factorization(pq);//factor::factorize(pq);//primes::factors(pq);//factors(pq);

  format!("Challenge {:#X}\nFactors: {:#X}, {:#X}\n", pq, answer[0], answer[1])
}

#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::JNIEnv;
  use self::jni::objects::{JClass, JString};
  use self::jni::sys::jstring;
  

  #[no_mangle]
  pub unsafe extern fn Java_com_test_1project_MobileAppBridge_factorize(env: JNIEnv, _: JClass, name: JString) -> jstring {
    let name: String = env.get_string(name).unwrap().into();
    let response = ::test_factorization();//format!("Hello {}!", name);
    env.new_string(response).unwrap().into_inner()
  }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}