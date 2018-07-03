use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct F {
    x: u32,
    y: u32,
    z: u32,
    w: Option<u32>,
}

#[derive(Debug)]
pub struct K(u32);


unsafe
fn convert_from_bytes<T>(slice: &[u8]) -> T {
    let mut ret: T;
    unsafe {
        ret = mem::uninitialized();
        ptr::copy_nonoverlapping(slice.as_ptr(),
                                 &mut ret as *mut T as *mut u8,
                                 mem::size_of::<T>());
    }
    ret
}

trait ConstantSized {
    fn constant_sized(&self) -> bool;
    fn read(x: &[u8], result: &mut Self);
    fn try_read(x: &[u8], result: &mut Self) -> bool {
        ConstantSized::read(x, result);
        true
    }
    fn size(&self) -> usize;
}

impl ConstantSized for u32 {
    fn constant_sized(&self) -> bool {
        return true;
    }
    fn size(&self) -> usize {
       return std::mem::size_of::<Self>()

    }
    fn read(x: &[u8], result: &mut Self) {
        *result = unsafe { convert_from_bytes(x) };
    }
    fn try_read(x: &[u8], result: &mut Self) -> bool {
        if x.len() >= std::mem::size_of::<Self>() {
            *result = unsafe { convert_from_bytes(x) };
            return true;
        }
        return false;
    }
}

impl ConstantSized for K {
    fn constant_sized(&self) -> bool {
        return false;
    }
    fn size(&self) -> usize {
        return std::mem::size_of::<Self>()
    }
    fn try_read(x: &[u8], result: &mut Self) -> bool {
        if x.len() >= 4 {
            *result = K( unsafe { convert_from_bytes(x) });
            return true;
        }
        return false;
    }
    fn read(x: &[u8], result: &mut Self) {
         *result = K( unsafe { convert_from_bytes(x) })
    }

}

impl<T: ConstantSized> ConstantSized for Option<T> {
    fn constant_sized(&self) -> bool {
        return false;
    }
    fn try_read(mut x: &[u8], result: &mut Self) -> bool {
        if x.len() < 1 {
            return false;
        }
        let some: bool = unsafe { convert_from_bytes(x) };
        if some {
            x = &x[1..];
            let mut t: T = unsafe { mem::uninitialized() };
            if !ConstantSized::try_read(x, &mut t) {
                return false;
            }
            *result = Some(t);
        } else {
            *result = None;
        }
        true
    }
    fn read(x: &[u8], result: &mut Self) {
        panic!();
    }
    fn size(&self) -> usize {
        return std::mem::size_of::<Self>()
    }
}


pub fn f(vec: &Vec<u8>, result: &mut F) -> bool {
    let x = true;
    let m = || {};
    let mut length = 0;
    let mut offset = 0;

    let mut fx = &mut result.x;
    let mut fy = &mut result.y;
    let mut fz = &mut result.z;
    let mut fw = &mut result.w;

    let mut s = &vec[..];
    {
    let previous_reads = |s: &[u8]| { return true };

    let cs = fx.constant_sized();
    if cs {
        length += fx.size();
    } else {
        if s.len() < length {
            return false;
        }
        previous_reads(s);
        s = &s[length..];
        length = 0;
        ConstantSized::try_read(&s[0..], fx);
    }
    let captured_length = length;
    let mut new_read = |s: &[u8]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(&s[captured_length - 1..], fx);
            return true;
        }
        return true;
    };
    let mut previous_reads = new_read;

    let cs = fy.constant_sized();
    if cs {
        length += fy.size();
    } else {
        if s.len() < length {
            return false;
        }
        previous_reads(s);
        s = &s[length..];
        length = 0;
        ConstantSized::try_read(&s[0..], fy);
    }
    let captured_length = length;
    let mut new_read = |s: &[u8]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(&s[captured_length - 1..], fy);
            return true;
        }
        return true;
    };
    let mut previous_reads = new_read;

    let cs = fz.constant_sized();
    if cs {
        length += fz.size();
    } else {
        if s.len() < length {
            return false;
        }
        previous_reads(s);
        s = &s[length..];
        length = 0;
        ConstantSized::try_read(&s[0..], fz);
    }
    let captured_length = length;
    let mut new_read = |s: &[u8]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(&s[captured_length - 1..], fz);
            return true;
        }
        return true;
    };
    let mut previous_reads = new_read;


    let cs = fw.constant_sized();
    if cs {
        length += fw.size();
    } else {
        if s.len() < length {
            return false;
        }
        previous_reads(s);
        s = &s[length..];
        length = 0;
        ConstantSized::try_read(&s[0..], fw);
    }
    let captured_length = length;
    let mut new_read = |s: &[u8]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(&s[captured_length - 1..], fw);
            return true;
        }
        return true;
    };
    let mut previous_reads = new_read;

    if length <= s.len() {
        if !previous_reads(s) {
            return false;
        }
    } else {
        return false;
    }
    }
    return true;
}