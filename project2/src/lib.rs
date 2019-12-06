use std::os::raw::{c_schar,c_int};
use core::ptr;
extern crate libc;

#[no_mangle]
pub extern "C" fn fn_strspn(string: *const c_schar, chars: *const c_schar) -> libc::size_t {
unsafe{    
if *chars.offset(0) == 0 {
        return 0;
    }
    if *chars.offset(1) == 0 {
        let mut i = 0;
        loop {
            if *string.offset(i) != *chars {
                return i as usize;
            }
            i += 1;
        }
    }

    let mut bit_array = AsciiBitArray::new();
    let mut i = 0;
    loop {
        let c = *chars.offset(i);
        if c == 0 {
            break;
        }
        bit_array.set_bit(c as usize);
        i += 1;
    }

    let mut j = 0;
    loop {
        let s = *string.offset(j);
        if s == 0 || !bit_array.get_bit(s as usize) {
            break;
        }
        j += 1;
    }

    j as usize
}
}
const ASCII_MAX_VALUE: usize = 127;
const U32_N_BITS: usize = 8 * 4;
const BIT_ARRAY_LEN: usize = 1 + ASCII_MAX_VALUE / U32_N_BITS;

struct AsciiBitArray {
    bit_array: [u32; BIT_ARRAY_LEN],
}

impl AsciiBitArray {
    fn new() -> AsciiBitArray {
        AsciiBitArray {
            bit_array: [0; BIT_ARRAY_LEN],
        }
    }

    fn set_bit(&mut self, index: usize) {
        let value = (1u32 << (index % U32_N_BITS)) as u32;
        let byte_index = index % BIT_ARRAY_LEN / U32_N_BITS;

        self.bit_array[byte_index] |= value;
    }

    fn get_bit(&self, index: usize) -> bool {
        let value = (1u32 << (index % U32_N_BITS)) as u32;
        let byte_index = index % BIT_ARRAY_LEN / U32_N_BITS;

        self.bit_array[byte_index] & value > 0
    }
}


#[no_mangle]
pub unsafe extern "C" fn fn_memset(m: *mut libc::c_void, c: c_int, n: libc::size_t)->*mut libc::c_void
{
unsafe{
let mut i = n as isize;
    let s = m as *mut c_schar;
    let c = c as i8;
    while i > 0 {
        i -= 1;
        *s.offset(i) = c;
           
        }
  
    m

}
}


#[no_mangle]
pub unsafe extern "C" fn fn_bzero(s: *mut libc::c_void, n: libc::size_t) {
    fn_memset(s, 0, n);
}


#[no_mangle]
pub extern "C" fn fn_memrchr(m: *const libc::c_void, c: c_int, n: libc::size_t) -> *const libc:: c_void {
unsafe{

    let mut i = n as isize;
    let s = m as *const u8;
    let c = c as u8;
    while i > 0 {
        i -= 1;
        if *s.offset(i) == c {
            return s.offset(i) as *const libc::c_void;
        }
    }
    ptr::null()
}
}


#[no_mangle]
pub extern "C" fn fn_memcpy(dest: *mut libc::c_void, src: *const libc::c_void, n: libc::size_t)
{
unsafe{
	let mut p1 = dest as *mut c_schar; 
	let p2 = src as *const c_schar;
	for i in 0..n 
	{
		*p1.offset(i as isize)=*p2.offset(i as isize);
	}

}
}


#[no_mangle]
pub unsafe extern "C" fn fn_strchr(s: *const c_schar, n: c_int) -> *const c_schar {
for idx in 0.. {
let ptr = s.offset(idx);
if n == (*ptr) as i32 {
return ptr;
}
if (*ptr) == 0 {
break;
}
}
core::ptr::null()
}


#[no_mangle]
pub unsafe extern "C" fn fn_strcmp(l: *const c_schar, r: *const c_schar) -> c_int {
   
    let mut x=0;
    for i in 0.. {
        let lc = *l.offset(i);
        let rc = *r.offset(i);
        if lc == 0 || lc != rc {
        x=lc - rc;
        break;   
        }
    }
    return x as c_int;
    
}


#[no_mangle]
pub unsafe extern "C" fn fn_strncmp(l: *const c_schar, r: *const c_schar, n: c_int) -> c_int {
   
    let mut x=0;
    for i in 0..n {
        let lc = *l.offset(i as isize);
        let rc = *r.offset(i as isize);
        if lc == 0 || lc != rc {
        x=lc - rc;
        break;   
        }
    }
    return x as c_int;
    
}


#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const c_schar) -> usize {
	let mut result = 0;
	while *s != 0 {
		s = s.offset(1);
		result += 1;
	}
	result
}


/*pub unsafe fn fn_compare(x: *const libc::c_schar, y: *const libc::c_schar) -> c_int
{
	let mut i=0;
	while *x != 0 && *y != 0
	{
		if *x.offset(i as isize) != *y.offset(i as isize)
		{
			0 as c_int
		}
		i=i+1;
		
	}
}
#[no_mangle]
pub unsafe extern "C" fn fn_strstr(x: *const libc::c_schar, y: *const libc::c_schar) -> *const libc::c_schar
{
	let mut i=0;
	while *x != 0
	{
		if *x.offset(i as isize)==*y.offset(i as isize ) && fn_compare(x,y)
		{
			x;
		}
		i=i+1; 	
	}
	ptr::null()
	
}*/

#[no_mangle]

pub unsafe extern "C" fn fn_memcmp(obj1: *const libc::c_void,obj2: *const libc::c_void, n: libc::size_t) -> c_int
{
	let p1= obj1 as *const c_schar;
	let p2= obj2 as *const c_schar;
	let mut x=0;
	for i in 0..n 
	{
        	let lc = *p1.offset(i as isize);
        	let rc = *p2.offset(i as isize);
        	if lc == 0 || lc != rc 
        	{
        		x=lc - rc;
        		break;   
        	}
        }
        return x as c_int;
}


/*#[no_mangle]
pub unsafe extern "C" fn fn_bcmp(s1: *const libc::c_void, s2: *const libc::c_void, n: libc::size_t) -> c_int {
     fn_memcmp((s1 as *const u8), (s2 as *const u8), n) as c_int
}*/



















































