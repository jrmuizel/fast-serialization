trait ConstSized {
    fn constant_size(&self) -> bool;
    fn length(&self) -> usize;
}

impl ConstSized for u32 {
    fn constant_size(&self) -> bool { true }
    fn length(&self) -> usize { 4 }
}

struct S {
    x: u32,
    y: u32,
    z: u32
}

struct R {
    len: usize
}


fn read_field() -> u32 {
    6
}


fn read_x<O: Fn()>(s: &mut S, slice: R, mut length: usize, old: O) -> bool {
    if s.x.constant_size() {
        return read_y(s, slice, length + s.x.length(), || { old(); s.x = read_field() })
    } else {
        if slice.len < length {
            return false;
        }
        old();
        s.x = read_field();
        return read_y(s, slice, 0, || {});
    }
}

fn read_y<O: Fn()>(s: &mut S, slice: R, mut length: usize, old: O) -> bool {
    if s.x.constant_size() {
        return read_z(s, slice, length + s.x.length(), || { old(); s.x = read_field() })
    } else {
        if slice.len < length {
            return false;
        }
        old();
        s.x = read_field();
        return read_z(s, slice,0, || {});
    }
}

fn read_z<O: Fn()>(s: &mut S, slice: R, mut length: usize, old: O) -> bool {
    if s.x.constant_size() {
        length += s.x.length();
        if slice.len < length {
            return false;
        }
        old();
        s.z = read_field();
        return true;
    } else {
        if slice.len < length {
            return false
        }
        old();
        s.x = read_field();
        return true;
    }
}


fn main() {
    //read(||{}, |x: Fn()|{ x() });
    println!("Hello, world!");
}
