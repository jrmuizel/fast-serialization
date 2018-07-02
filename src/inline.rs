#[derive(Debug)]
pub struct F {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

#[derive(Debug)]
pub struct K(u32);

trait ConstantSized {
    fn constant_sized(&self) -> bool;
    fn read(x: u32, result: &mut Self);
    fn size(&self) -> usize;
}

impl ConstantSized for u32 {
    fn constant_sized(&self) -> bool {
        return true;
    }
    fn size(&self) -> usize {
       return std::mem::size_of::<Self>() / std::mem::size_of::<u32>()

    }
    fn read(x: u32, result: &mut Self) {
        *result = x
    }
}

impl ConstantSized for K {
    fn constant_sized(&self) -> bool {
        return false;
    }
    fn size(&self) -> usize {
        return std::mem::size_of::<Self>() / std::mem::size_of::<u32>()
    }
    fn read(x: u32, result: &mut Self) {
         *result = K(x)
    }

}

impl ConstantSized for Option<u32> {
    fn constant_sized(&self) -> bool {
        return false;
    }
    fn read(x: u32, result: &mut Self) {
        *result = Some(x)
    }
    fn size(&self) -> usize {
        return std::mem::size_of::<Self>()
    }
}


pub fn f(vec: &Vec<u32>, result: &mut F) -> bool {
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
    let previous_reads = |s: &[u32]| { return true };

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
        ConstantSized::read(unsafe { *s.get_unchecked(0) }, fx);
    }
    let captured_length = length;
    let mut new_read = |s: &[u32]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(unsafe { *s.get_unchecked(captured_length - 1) }, fx);
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
        ConstantSized::read(unsafe { *s.get_unchecked(0) }, fy);
    }
    let captured_length = length;
    let mut new_read = |s: &[u32]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(unsafe { *s.get_unchecked(captured_length - 1) }, fy);
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
        ConstantSized::read(unsafe { *s.get_unchecked(0) }, fz);
    }
    let captured_length = length;
    let mut new_read = |s: &[u32]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(unsafe { *s.get_unchecked(captured_length - 1) }, fz);
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
        ConstantSized::read(unsafe { *s.get_unchecked(0) }, fw);
    }
    let captured_length = length;
    let mut new_read = |s: &[u32]| {
        if cs {
            if !previous_reads(s) { return false }
            ConstantSized::read(unsafe { *s.get_unchecked(captured_length - 1) }, fw);
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