extern crate rand;

use rand::Rng;
/*
ull g(ull x, ull c, uint8_t pow) {
        ull p = 1;
        for(int i = 0; i < pow; i++) {
            p *= x;
        }
        return p + c;
    }
  */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn g(x: u128, c: u128, pow: u128) -> u128
{
    let mut p: u128 = 1;
    for i in 0..pow
    {
        p = p*x;
    }
    return p+c;
}


fn gcd(mut a:u128, mut b:u128) -> u128
{
    loop
    {
        let buf = a % b;
        a = b;
        b = buf;
        if b == 0
        {
            return a;
        }
    }
    return 0;
}

pub fn pollard_rho_factor(n:u128) -> u128
{
    let mut rng = rand::thread_rng();
    if n== 4
    {
        return 2;
    }
    let mut success:bool = false;
    let mut d: u128 = 0;
    let mut i: u8 = 0;
    loop {
        let mut x = rng.gen::<u128>() % n;
        let mut y = x;
        let c = i+1;
        let j:u8 = 0;
        loop {
            x = g(x, c as u128, 2) % n;
            y = g(g(y, c as u128, 2), c as u128, 2) % n;
            d = gcd(x-y, n);
            if d != 1 || j >= 1000
            {
                break;
            }
            if j>= 1000
            {
                i += 1;
            } else if d!=1 && d!= n
            {
                success = true;
            } else
            {
                i += 1;
            }
            if success || i >= 100
            {
                break;
            }
        }
    }
    if !success
    {
        return n;
    }
    return d;
}

pub fn pollard_rho_factorise( n: u128) -> Vec<u128>
{
    let mut factors:Vec<u128> = vec![];
    let a = pollard_rho_factor(n);
    if a == n
    {
        return factors;
    }
    let mut rec_result = pollard_rho_factorise(a);
    if rec_result.len() == 0
    {
        factors.push(a);
    }
    else
    {
        for i in rec_result.iter()
        {
            factors.push(*i);
        }
    }
    let b = n/a;
    rec_result = pollard_rho_factorise(b);
    if rec_result.len() == 0
    {
        factors.push(b);
    }
    else
    {
        for i in rec_result.iter()
        {
            factors.push(*i);
        }
    }
    return factors;
}