use std::ops::{Add, Sub, Mul};
use std::marker::Copy;

// パーセプトロン
#[derive(Debug)]
struct Vector<T>
where
    T: Copy + Add + Sub
{
    arr: Vec<T>,
}

// 各種演算の実装
impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
{
    fn new(vec: Vec<T>) -> Self {
        let mut arr = vec![];
        for x in vec {
            arr.push(x);
        }
        Vector { arr }
    }

    fn add(&self, other: &Self) -> Result<Self, &'static str> {
        if self.arr.len() != other.arr.len() {
            return Err("ベクトルの次元が一致しません");
        }

        let n = self.arr.len();
        let mut arr = vec![];
        for i in 0..n {
            arr.push( self.arr[i] + other.arr[i] );
        }
        Ok( Vector { arr } )
    }

    fn sub(&self, other: &Self) -> Result<Self, &'static str> {
        if self.arr.len() != other.arr.len() {
            return Err("ベクトルの次元が一致しません");
        }

        let n = self.arr.len();
        let mut arr = vec![];
        for i in 0..n {
            arr.push( self.arr[i] - other.arr[i] );
        }
        Ok( Vector { arr } )
    }

    fn mul_scalar(&self, x: T) -> Self {
        let n = self.arr.len();
        let mut arr = vec![];
        for i in 0..n {
            arr.push( x * self.arr[i] );
        }
        Vector { arr }
    }

    fn dot(&self, other: &Self) -> Result<T, &'static str> {
        if self.arr.len() != other.arr.len() {
            return Err("ベクトルの次元が一致しません");
        }
        
        let res = self.arr.iter()
                          .zip(other.arr.iter())
                          .map(|(&x, &y)| x * y)
                          .reduce(|acc, v| acc + v)
                          .unwrap();
        
        Ok( res )
    }
}


fn main() {

    let v1: Vector<isize> = Vector::new(vec![1, 2, -3]);
    let v2: Vector<isize> = Vector::new(vec![3, -2, 4]);

    println!("{:?} + {:?} = {:?}", v1, v2, v1.add(&v2).unwrap());
    println!("{:?} - {:?} = {:?}", v1, v2, v1.sub(&v2).unwrap());
    println!("5 * {:?} = {:?}", v1, v1.mul_scalar(5));
    println!("{:?} . {:?} = {:?}", v1, v2, v1.dot(&v2).unwrap());
}
