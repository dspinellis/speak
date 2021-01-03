#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, main, ptr_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn lseek(__fd: std::os::raw::c_int, __offset: __off_t, __whence: std::os::raw::c_int)
     -> __off_t;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(__fd: std::os::raw::c_int, __buf: *mut std::os::raw::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn creat(__file: *const std::os::raw::c_char, __mode: mode_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type __mode_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = std::os::raw::c_ulong;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rec {
    pub word: std::os::raw::c_int,
    pub phon: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decenc {
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
}
#[no_mangle]
pub static mut diags: [*mut std::os::raw::c_char; 6] =
    [0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"bad option\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"no vocabulary\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"can\'t create\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"too many words\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"too many chars\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char];
#[no_mangle]
pub static mut aeiou: *mut std::os::raw::c_char =
    b"aeiou\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut aeiouy: *mut std::os::raw::c_char =
    b"aeiouy\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut aeiouwxy: *mut std::os::raw::c_char =
    b"aeiouwxy\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut aeo: *mut std::os::raw::c_char =
    b"aeo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut aou: *mut std::os::raw::c_char =
    b"aou\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut bcdfgkpt: *mut std::os::raw::c_char =
    b"bcdfgkpt\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub static mut recsize: std::os::raw::c_int =
    ::std::mem::size_of::<rec>() as std::os::raw::c_ulong as std::os::raw::c_int;
#[no_mangle]
pub static mut table: [rec; 800] = [rec{word: 0, phon: 0,}; 800];
#[no_mangle]
pub static mut strings: [std::os::raw::c_char; 9500] = [0; 9500];
#[no_mangle]
pub static mut ttop: std::os::raw::c_int = 1 as std::os::raw::c_int;
#[no_mangle]
pub static mut stop: std::os::raw::c_int = 1 as std::os::raw::c_int;
#[no_mangle]
pub static mut buf: [std::os::raw::c_char; 100] = [0; 100];
#[no_mangle]
pub static mut eflag: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut tflag: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut code: [std::os::raw::c_int; 128] =
    [('0' as i32) << 8 as std::os::raw::c_int | 'a' as i32, 0o33 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'a' as i32, 0o52 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'a' as i32, 0o67 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'W' as i32, 0o2 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'W' as i32, 0o54 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'W' as i32, 0o17 as std::os::raw::c_int,
     ('e' as i32) << 8 as std::os::raw::c_int | 'a' as i32, 0o21 as std::os::raw::c_int,
     ('a' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o20 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'A' as i32, 0o37 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'A' as i32, 0o71 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'A' as i32, 0o72 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o4 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o75 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o76 as std::os::raw::c_int,
     ('3' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o77 as std::os::raw::c_int,
     ('r' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o5 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'E' as i32, 0o23 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'E' as i32, 0o26 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'E' as i32, 0o35 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'y' as i32, 0o3 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'y' as i32, 0o36 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'i' as i32, 0o30 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'i' as i32, 0o64 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'i' as i32, 0o65 as std::os::raw::c_int,
     ('3' as i32) << 8 as std::os::raw::c_int | 'i' as i32, 0o66 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'o' as i32, 0o31 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'o' as i32, 0o12 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'o' as i32, 0o13 as std::os::raw::c_int,
     ('u' as i32) << 8 as std::os::raw::c_int | 'o' as i32, 0o51 as std::os::raw::c_int,
     ('u' as i32) << 8 as std::os::raw::c_int | 'e' as i32, 0o11 as std::os::raw::c_int,
     ('o' as i32) << 8 as std::os::raw::c_int | 'o' as i32, 0o50 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'u' as i32, 0o14 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'u' as i32, 0o15 as std::os::raw::c_int,
     ('2' as i32) << 8 as std::os::raw::c_int | 'u' as i32, 0o16 as std::os::raw::c_int,
     ('3' as i32) << 8 as std::os::raw::c_int | 'u' as i32, 0o34 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | 'U' as i32, 0o27 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | 'U' as i32, 0o10 as std::os::raw::c_int,
     'b' as i32, 0o61 as std::os::raw::c_int, 'd' as i32, 0o41 as std::os::raw::c_int,
     ('t' as i32) << 8 as std::os::raw::c_int | 'd' as i32, 0o73 as std::os::raw::c_int,
     'f' as i32, 0o42 as std::os::raw::c_int, 'g' as i32, 0o43 as std::os::raw::c_int,
     'h' as i32, 0o44 as std::os::raw::c_int, 'k' as i32, 0o46 as std::os::raw::c_int,
     'l' as i32, 0o47 as std::os::raw::c_int, 'm' as i32, 0o63 as std::os::raw::c_int,
     'n' as i32, 0o62 as std::os::raw::c_int, 'p' as i32, 0o32 as std::os::raw::c_int,
     'r' as i32, 0o24 as std::os::raw::c_int, 's' as i32, 0o40 as std::os::raw::c_int,
     't' as i32, 0o25 as std::os::raw::c_int, 'v' as i32, 0o60 as std::os::raw::c_int,
     'w' as i32, 0o22 as std::os::raw::c_int, 'z' as i32, 0o55 as std::os::raw::c_int,
     ('h' as i32) << 8 as std::os::raw::c_int | 's' as i32, 0o56 as std::os::raw::c_int,
     ('h' as i32) << 8 as std::os::raw::c_int | 'z' as i32, 0o70 as std::os::raw::c_int,
     'j' as i32, 0o45 as std::os::raw::c_int,
     ('h' as i32) << 8 as std::os::raw::c_int | 'c' as i32, 0o57 as std::os::raw::c_int,
     ('h' as i32) << 8 as std::os::raw::c_int | 't' as i32, 0o6 as std::os::raw::c_int,
     ('h' as i32) << 8 as std::os::raw::c_int | 'd' as i32, 0o7 as std::os::raw::c_int,
     ('g' as i32) << 8 as std::os::raw::c_int | 'n' as i32, 0o53 as std::os::raw::c_int,
     ('0' as i32) << 8 as std::os::raw::c_int | '-' as i32, 0o1 as std::os::raw::c_int,
     ('1' as i32) << 8 as std::os::raw::c_int | '-' as i32, 0o74 as std::os::raw::c_int,
     0 as std::os::raw::c_int, 0 as std::os::raw::c_int];
#[no_mangle]
pub static mut work: [std::os::raw::c_char; 100] = [0; 100];
#[no_mangle]
pub static mut line: [std::os::raw::c_char; 100] = [0; 100];
/* Voice synthesizer file descriptor */
#[no_mangle]
pub static mut vs: std::os::raw::c_int = 1 as std::os::raw::c_int;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut wtop: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut f: std::os::raw::c_int = 0;
    let mut pflag: std::os::raw::c_int = 0;
    let mut sflag: std::os::raw::c_int = 0;
    let mut vflag: std::os::raw::c_int = 0;
    let mut lflag: std::os::raw::c_int = 0;
    let mut xflag: std::os::raw::c_int = 0;
    let mut yflag: std::os::raw::c_int = 0;
    let mut wflag: std::os::raw::c_int = 0;
    let mut w: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut v: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    vflag = 1 as std::os::raw::c_int;
    sflag = vflag;
    eflag = sflag;
    pflag = eflag;
    if argc > 1 as std::os::raw::c_int &&
           **argv.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 {
        loop  {
            let ref mut fresh0 = *argv.offset(1 as std::os::raw::c_int as isize);
            let fresh1 = *fresh0;
            *fresh0 = (*fresh0).offset(1);
            match *fresh1 as std::os::raw::c_int {
                101 => { eflag = 0 as std::os::raw::c_int }
                112 => { pflag = 0 as std::os::raw::c_int }
                115 => { sflag = 0 as std::os::raw::c_int }
                118 => { vflag = 0 as std::os::raw::c_int }
                0 => { break ; }
                _ => { }
            }
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if vflag != 0 {
        vs =
            if argc > 2 as std::os::raw::c_int {
                creat(*argv.offset(2 as std::os::raw::c_int as isize),
                      0o666 as std::os::raw::c_int as mode_t)
            } else { 1 as std::os::raw::c_int }
    }
    readin(if argc > 1 as std::os::raw::c_int {
               *argv.offset(1 as std::os::raw::c_int as isize)
           } else {
               b"speak.m\x00" as *const u8 as *const std::os::raw::c_char
           } as *mut std::os::raw::c_char);
    loop  {
        t = line.as_mut_ptr();
        loop  {
            if read(0 as std::os::raw::c_int, t as *mut std::os::raw::c_void,
                    1 as std::os::raw::c_int as size_t) == 0 {
                exit(0 as std::os::raw::c_int);
            }
            let fresh2 = t;
            t = t.offset(1);
            if !(*fresh2 as std::os::raw::c_int != '\n' as i32) { break ; }
        }
        *t.offset(-(1 as std::os::raw::c_int as isize)) = ' ' as i32 as std::os::raw::c_char;
        *t = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if line[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '!' as i32 {
            let mut current_block_36: u64;
            match line[1 as std::os::raw::c_int as usize] as std::os::raw::c_int {
                99 => { copy(); current_block_36 = 7420279277351916581; }
                119 => {
                    writeout(name());
                    current_block_36 = 8402191382813745447;
                }
                114 => { current_block_36 = 8402191382813745447; }
                100 => {
                    if phread(work.as_mut_ptr(),
                              buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                          isize)) !=
                           buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize)
                       {
                        decode(1 as std::os::raw::c_int,
                               buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                           isize));
                    } else {
                        tflag = 1 as std::os::raw::c_int;
                        phpron(work.as_mut_ptr(), buf.as_mut_ptr());
                        tflag = 0 as std::os::raw::c_int
                    }
                    write(1 as std::os::raw::c_int,
                          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
                              *const std::os::raw::c_void,
                          1 as std::os::raw::c_int as size_t);
                    current_block_36 = 7420279277351916581;
                }
                112 => {
                    decode(1 as std::os::raw::c_int,
                           buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                       isize));
                    write(1 as std::os::raw::c_int,
                          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
                              *const std::os::raw::c_void,
                          1 as std::os::raw::c_int as size_t);
                    current_block_36 = 7420279277351916581;
                }
                108 => {
                    i = 0 as std::os::raw::c_int;
                    loop  {
                        i += 1;
                        if !(i < ttop) { break ; }
                        list(1 as std::os::raw::c_int,
                             &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).word
                                                                   as isize));
                    }
                    current_block_36 = 7420279277351916581;
                }
                _ => {
                    diag(1 as std::os::raw::c_int);
                    current_block_36 = 7420279277351916581;
                }
            }
            match current_block_36 {
                8402191382813745447 => {
                    t = name();
                    if *t != 0 { readin(t); }
                }
                _ => { }
            }
        } else {
            if replace() == 0 &&
                   line[1 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                       '\u{0}' as i32 {
                t = line.as_mut_ptr();
                u = work.as_mut_ptr();
                lflag = 0 as std::os::raw::c_int;
                's_288:
                    loop  {
                        while *t as std::os::raw::c_int == ' ' as i32 ||
                                  *t as std::os::raw::c_int == '\t' as i32 {
                            t = t.offset(1)
                        }
                        loop  {
                            let fresh3 = t;
                            t = t.offset(1);
                            *u = *fresh3;
                            if !(*u as std::os::raw::c_int != ' ' as i32 &&
                                     *u as std::os::raw::c_int != '\t' as i32) {
                                break ;
                            }
                            if !(*u != 0) { break 's_288 ; }
                            if 'a' as i32 <= *u as std::os::raw::c_int &&
                                   'z' as i32 >= *u as std::os::raw::c_int ||
                                   *u as std::os::raw::c_int == '%' as i32 {
                                lflag += 1
                            }
                            u = u.offset(1)
                        }
                        let fresh4 = u;
                        u = u.offset(1);
                        *fresh4 = 0 as std::os::raw::c_int as std::os::raw::c_char
                    }
                wtop = u
            }
            t = work.as_mut_ptr();
            while t < wtop {
                u =
                    phread(t,
                           buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                       isize));
                yflag = 0 as std::os::raw::c_int;
                wflag = yflag;
                if u == buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) &&
                       pflag != 0 {
                    v = t;
                    while oneof(*v,
                                b"([`\"\x00" as *const u8 as
                                    *const std::os::raw::c_char as *mut std::os::raw::c_char)
                              != 0 {
                        v = v.offset(1)
                    }
                    w = v;
                    while *w != 0 {
                        if *w as std::os::raw::c_int == '-' as i32 &&
                               w.offset_from(v) as std::os::raw::c_long >=
                                   2 as std::os::raw::c_int as std::os::raw::c_long {
                            wflag = *w as std::os::raw::c_int;
                            *w = 0 as std::os::raw::c_int as std::os::raw::c_char;
                            break ;
                        } else { w = w.offset(1) }
                    }
                    if wflag == 0 {
                        loop  {
                            w = w.offset(-1);
                            if !(oneof(*w,
                                       b".,;:?!\'\"])\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut std::os::raw::c_char) != 0) {
                                break ;
                            }
                        }
                        w = w.offset(1);
                        yflag = *w as std::os::raw::c_int
                    }
                    if !(w <= v) {
                        *w = 0 as std::os::raw::c_int as std::os::raw::c_char;
                        xflag = 0 as std::os::raw::c_int;
                        if lflag == 0 {
                            u = v;
                            while *u != 0 {
                                if fold(u) != 0 { xflag += 1 }
                                u = u.offset(1)
                            }
                        }
                        if !((yflag != 0 || xflag != 0 || wflag != 0) &&
                                 {
                                     u =
                                         phread(v,
                                                buf.as_mut_ptr().offset(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize));
                                     (u) !=
                                         buf.as_mut_ptr().offset(1 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                 }) {
                            u =
                                phpron(v,
                                       buf.as_mut_ptr().offset(1 as
                                                                   std::os::raw::c_int
                                                                   as isize))
                        }
                        *w = yflag as std::os::raw::c_char;
                        if *w as std::os::raw::c_int != 0 &&
                               u !=
                                   buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                               isize) {
                            /*pause for punct*/
                            let fresh5 = u; /*phoneme ,2-1*/
                            u = u.offset(1); /* temp */
                            *fresh5 = 0o1 as std::os::raw::c_int as std::os::raw::c_char;
                            let fresh6 = u;
                            u = u.offset(1);
                            *fresh6 = 0o1 as std::os::raw::c_int as std::os::raw::c_char
                        }
                    }
                }
                if u == buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) &&
                       wflag != 0 {
                    *w = wflag as std::os::raw::c_char
                }
                if u == buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) &&
                       sflag != 0 {
                    u =
                        phspell(t,
                                buf.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                            isize))
                }
                *buf.as_mut_ptr() = 0o174 as std::os::raw::c_int as std::os::raw::c_char;
                let fresh7 = u;
                u = u.offset(1);
                *fresh7 = 0o174 as std::os::raw::c_int as std::os::raw::c_char;
                let fresh8 = u;
                u = u.offset(1);
                *fresh8 = 0 as std::os::raw::c_int as std::os::raw::c_char;
                if vflag != 0 {
                    write(vs, buf.as_mut_ptr() as *const std::os::raw::c_void,
                          u.offset_from(buf.as_mut_ptr()) as
                              std::os::raw::c_long as size_t);
                }
                loop  {
                    let fresh9 = t;
                    t = t.offset(1);
                    if !(*fresh9 != 0) { break ; }
                }
            }
        }
    };
}
/* Decode s for listing */
#[no_mangle]
pub unsafe extern "C" fn decode(mut f: std::os::raw::c_int,
                                mut s: *mut std::os::raw::c_char) {
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut flag: std::os::raw::c_int = 0;
    flag = 1 as std::os::raw::c_int;
    loop  {
        let fresh10 = s;
        s = s.offset(1);
        c = *fresh10 as std::os::raw::c_int;
        if !(c != 0) { break ; }
        if flag != 0 {
            write(f,
                  b",\x00" as *const u8 as *const std::os::raw::c_char as
                      *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
            b = '3' as i32 - ((c & 0o377 as std::os::raw::c_int) >> 6 as std::os::raw::c_int);
            if c == 0o1 as std::os::raw::c_int {
                flag = 0 as std::os::raw::c_int;
                c = '%' as i32
            } else {
                if b != '2' as i32 {
                    write(1 as std::os::raw::c_int,
                          &mut b as *mut std::os::raw::c_int as *const std::os::raw::c_void,
                          1 as std::os::raw::c_int as size_t);
                }
                c = c & 0o77 as std::os::raw::c_int;
                c =
                    dencode(&mut *code.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                               isize),
                            &mut *code.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                               isize), c)
            }
        }
        while c != 0 {
            write(f, &mut c as *mut std::os::raw::c_int as *const std::os::raw::c_void,
                  1 as std::os::raw::c_int as size_t);
            c >>= 8 as std::os::raw::c_int
        }
    };
}
/*
 * Encode and decode from the first column of code to the second, and
 * the opposite.
 * Examples:
 * assert(dencode(&code[0],&code[1],'u3') == 034);
 * assert(dencode(&code[1],&code[0],034) == 'u3');
 */
#[no_mangle]
pub unsafe extern "C" fn dencode(mut at1: *mut std::os::raw::c_int,
                                 mut at2: *mut std::os::raw::c_int,
                                 mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    /*
	 * Through this cast search and results iterate over the corresponding
	 * table column.
	 */
    let mut t1: *mut decenc =
        at1 as *mut decenc; /*get to byte 0 of string store*/
    let mut t2: *mut decenc = at2 as *mut decenc; /*and put it out*/
    let mut i: std::os::raw::c_int = 0; /*new value of stop */
    let mut d: std::os::raw::c_int = 0; /* +1 saves wasted time looking up % */
    i = 0 as std::os::raw::c_int;
    loop  {
        d = (*t1.offset(i as isize)).x;
        if !(d != 0) { break ; }
        if c == d { break ; }
        i += 1
    }
    return (*t2.offset(i as isize)).x;
}
#[no_mangle]
pub unsafe extern "C" fn replace() -> std::os::raw::c_int {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut n: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    t = line.as_mut_ptr();
    u = buf.as_mut_ptr();
    let fresh11 = t;
    t = t.offset(1);
    if *fresh11 as std::os::raw::c_int != ',' as i32 { return 0 as std::os::raw::c_int }
    loop  {
        if *t as std::os::raw::c_int == '%' as i32 {
            let fresh12 = u;
            u = u.offset(1);
            *fresh12 = 0o1 as std::os::raw::c_int as std::os::raw::c_char;
            loop  {
                t = t.offset(1);
                *u = *t;
                if !(*u as std::os::raw::c_int != 0 &&
                         *u as std::os::raw::c_int != ' ' as i32) {
                    break ;
                }
                u = u.offset(1)
            }
            break ;
        } else {
            b = 1 as std::os::raw::c_int;
            if *t as std::os::raw::c_int <= '3' as i32 {
                if *t as std::os::raw::c_int >= '0' as i32 {
                    let fresh13 = t;
                    t = t.offset(1);
                    b = '3' as i32 - *fresh13 as std::os::raw::c_int
                }
            }
            n = 0 as std::os::raw::c_int;
            while *t as std::os::raw::c_int != ',' as i32 &&
                      *t as std::os::raw::c_int != ' ' as i32 &&
                      *t as std::os::raw::c_int != 0 as std::os::raw::c_int {
                let fresh14 = t;
                t = t.offset(1);
                i = *fresh14 as std::os::raw::c_int;
                if n != 0 { i <<= 8 as std::os::raw::c_int }
                n |= i
            }
            n =
                dencode(&mut *code.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                           isize),
                        &mut *code.as_mut_ptr().offset(1 as std::os::raw::c_int as
                                                           isize), n);
            if n != 0 {
                let fresh15 = u;
                u = u.offset(1);
                *fresh15 = (n + (b << 6 as std::os::raw::c_int)) as std::os::raw::c_char
            }
            if *t as std::os::raw::c_int != ',' as i32 &&
                   *t as std::os::raw::c_int != ' ' as i32 {
                break ;
            }
            t = t.offset(1)
        }
    }
    let fresh16 = u;
    u = u.offset(1);
    *fresh16 = 0 as std::os::raw::c_int as std::os::raw::c_char;
    phwrite(work.as_mut_ptr(), buf.as_mut_ptr());
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list(mut f: std::os::raw::c_int, mut s: *mut std::os::raw::c_char) {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if phread(s, buf.as_mut_ptr()) == buf.as_mut_ptr() { return }
    write(f,
          b" \x00" as *const u8 as *const std::os::raw::c_char as *const std::os::raw::c_void,
          1 as std::os::raw::c_int as size_t);
    t = s;
    while *t != 0 {
        let fresh17 = t;
        t = t.offset(1);
        write(f, fresh17 as *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    }
    write(f,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    decode(f, buf.as_mut_ptr());
    write(f,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut buf1: [std::os::raw::c_char; 100] = [0; 100];
    phread(work.as_mut_ptr(), buf1.as_mut_ptr());
    phwrite(name(), buf1.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn name() -> *mut std::os::raw::c_char {
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    u =
        &mut *line.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_char;
    while *u as std::os::raw::c_int == ' ' as i32 { u = u.offset(1) }
    t = buf.as_mut_ptr();
    while *u as std::os::raw::c_int != 0 &&
              {
                  let fresh18 = u;
                  u = u.offset(1);
                  *t = *fresh18;
                  (*t as std::os::raw::c_int) != ' ' as i32
              } {
        t = t.offset(1)
    }
    *t = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn readin(mut file: *mut std::os::raw::c_char) {
    let mut f: std::os::raw::c_int = 0;
    f = open(file, 0 as std::os::raw::c_int);
    if f < 0 as std::os::raw::c_int { diag(2 as std::os::raw::c_int); return }
    read(f, &mut ttop as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
         ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    read(f, table.as_mut_ptr() as *mut std::os::raw::c_void,
         (recsize * ttop) as size_t);
    read(f, &mut stop as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
         ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    read(f, strings.as_mut_ptr() as *mut std::os::raw::c_void, stop as size_t);
    close(f);
}
#[no_mangle]
pub unsafe extern "C" fn writo1(mut f: std::os::raw::c_int, mut u: *mut std::os::raw::c_int,
                                mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    j = *u;
    i = j;
    *u = n;
    k = 1 as std::os::raw::c_int;
    loop  {
        let fresh19 = i;
        i = i + 1;
        if !(strings[fresh19 as usize] != 0) { break ; }
        k += 1
    }
    write(f,
          &mut *strings.as_mut_ptr().offset(j as isize) as *mut std::os::raw::c_char
              as *const std::os::raw::c_void, k as size_t);
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn writeout(mut file: *mut std::os::raw::c_char) {
    let mut f: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    f = creat(file, 0o666 as std::os::raw::c_int as mode_t);
    if f < 0 as std::os::raw::c_int { diag(3 as std::os::raw::c_int); return }
    lseek(f,
          ((recsize * ttop) as
               std::os::raw::c_ulong).wrapping_add(::std::mem::size_of::<std::os::raw::c_int>()
                                               as
                                               std::os::raw::c_ulong).wrapping_add(::std::mem::size_of::<std::os::raw::c_int>()
                                                                               as
                                                                               std::os::raw::c_ulong)
              as __off_t, 0 as std::os::raw::c_int);
    write(f, strings.as_mut_ptr() as *const std::os::raw::c_void,
          1 as std::os::raw::c_int as size_t);
    n = 1 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i < ttop {
        n += writo1(f, &mut (*table.as_mut_ptr().offset(i as isize)).word, n);
        n += writo1(f, &mut (*table.as_mut_ptr().offset(i as isize)).phon, n);
        i += 1
    }
    lseek(f, 0 as std::os::raw::c_int as __off_t, 0 as std::os::raw::c_int);
    write(f, &mut ttop as *mut std::os::raw::c_int as *const std::os::raw::c_void,
          ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    write(f, table.as_mut_ptr() as *const std::os::raw::c_void,
          (recsize * ttop) as size_t);
    write(f, &mut n as *mut std::os::raw::c_int as *const std::os::raw::c_void,
          ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    close(f);
}
#[no_mangle]
pub unsafe extern "C" fn find(mut in_0: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut bot: std::os::raw::c_int = 0;
    let mut top: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut z: std::os::raw::c_int = 0;
    bot = 0 as std::os::raw::c_int;
    top = ttop;
    z = 0 as std::os::raw::c_int;
    loop  {
        i = (bot + top) / 2 as std::os::raw::c_int;
        if !(i > bot) { break ; }
        z =
            compare(in_0,
                    &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize)).word
                                                          as isize));
        if z == 0 as std::os::raw::c_int { break ; }
        if z < 0 as std::os::raw::c_int { top = i } else { bot = i }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn prefix(mut in_0: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut pref: std::os::raw::c_int = 0;
    let mut bot: std::os::raw::c_int = 0;
    let mut top: std::os::raw::c_int = 0;
    bot = 0 as std::os::raw::c_int;
    pref = bot;
    top = ttop;
    end = in_0.offset(1 as std::os::raw::c_int as isize);
    's_26:
        loop  {
            i = (bot + top) / 2 as std::os::raw::c_int;
            if !(i > bot) { break ; }
            loop  {
                s =
                    &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize)).word
                                                          as isize) as
                        *mut std::os::raw::c_char;
                u = in_0;
                loop  {
                    if (*u as std::os::raw::c_int) < *s as std::os::raw::c_int {
                        top = i;
                        continue 's_26 ;
                    } else {
                        let fresh20 = s;
                        s = s.offset(1);
                        if *u as std::os::raw::c_int == *fresh20 as std::os::raw::c_int {
                            if *u as std::os::raw::c_int == 0 as std::os::raw::c_int {
                                return i
                            }
                            if *s as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                u = u.offset(1)
                            } else {
                                bot = i;
                                pref = bot;
                                end = u.offset(1 as std::os::raw::c_int as isize);
                                continue 's_26 ;
                            }
                        } else {
                            if !(u <= end) { break ; }
                            bot = i;
                            continue 's_26 ;
                        }
                    }
                }
                i = (bot + i) / 2 as std::os::raw::c_int;
                if !(i > bot) { break ; }
            }
            bot += 1;
            end = end.offset(1)
        }
    return pref;
}
#[no_mangle]
pub unsafe extern "C" fn compare(mut a: *mut std::os::raw::c_char,
                                 mut b: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    while *a as std::os::raw::c_int == *b as std::os::raw::c_int {
        if *a as std::os::raw::c_int == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
        a = a.offset(1);
        b = b.offset(1)
    }
    return if (*a as std::os::raw::c_int) < *b as std::os::raw::c_int {
               -(1 as std::os::raw::c_int)
           } else { 1 as std::os::raw::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn phread(mut in_0: *mut std::os::raw::c_char,
                                mut out: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    i = find(in_0);
    if compare(in_0,
               &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).word
                                                     as isize)) ==
           0 as std::os::raw::c_int {
        s =
            &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i as
                                                                              isize)).phon
                                                  as isize) as
                *mut std::os::raw::c_char;
        loop  {
            let fresh21 = s;
            s = s.offset(1);
            *out = *fresh21;
            if !(*out != 0) { break ; }
            out = out.offset(1)
        }
    }
    *out = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn phwrite(mut in_0: *mut std::os::raw::c_char,
                                 mut out: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut z: std::os::raw::c_int = 0;
    i = find(in_0);
    z =
        compare(in_0,
                &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)).word
                                                      as isize));
    if 0 as std::os::raw::c_int != z {
        if *out as std::os::raw::c_int == 0 as std::os::raw::c_int { return }
        i += 1;
        if ttop >= 800 as std::os::raw::c_int { diag(4 as std::os::raw::c_int); return }
        j = ttop;
        while j > i {
            table[j as usize].word =
                table[(j - 1 as std::os::raw::c_int) as usize].word;
            table[j as usize].phon =
                table[(j - 1 as std::os::raw::c_int) as usize].phon;
            j -= 1
        }
        table[i as usize].word = stop;
        stash(in_0);
        ttop += 1
    } else if *out as std::os::raw::c_int == 0 as std::os::raw::c_int {
        j = i;
        while j < ttop {
            table[j as usize].word =
                table[(j + 1 as std::os::raw::c_int) as usize].word;
            table[j as usize].phon =
                table[(j + 1 as std::os::raw::c_int) as usize].phon;
            j += 1
        }
        ttop -= 1;
        return
    }
    table[i as usize].phon = stop;
    stash(out);
}
/* Add the passed string to the string table, updating stop */
#[no_mangle]
pub unsafe extern "C" fn stash(mut s: *mut std::os::raw::c_char) {
    while stop < 9500 as std::os::raw::c_int {
        let fresh22 = s; /*phoneme 2-0*/
        s = s.offset(1); /* ,0k */
        let fresh23 = stop;
        stop = stop + 1;
        strings[fresh23 as usize] = *fresh22;
        if strings[fresh23 as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int {
            return
        }
    }
    diag(5 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn diag(mut n: std::os::raw::c_int) {
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    p = diags[n as usize];
    while *p != 0 {
        let fresh24 = p;
        p = p.offset(1);
        write(1 as std::os::raw::c_int, fresh24 as *const std::os::raw::c_void,
              1 as std::os::raw::c_int as size_t);
    }
    write(1 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn phspell(mut in_0: *mut std::os::raw::c_char,
                                 mut out: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut c: [std::os::raw::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4],
                                 &mut [std::os::raw::c_char; 4]>(b"* \x00\x00");
    loop  {
        let fresh25 = in_0;
        in_0 = in_0.offset(1);
        c[1 as std::os::raw::c_int as usize] = *fresh25;
        if !(c[1 as std::os::raw::c_int as usize] != 0) { break ; }
        fold(&mut *c.as_mut_ptr().offset(1 as std::os::raw::c_int as isize));
        t = phread(c.as_mut_ptr(), out);
        if t != out {
            out = t
        } else {
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = 0o346 as std::os::raw::c_int as std::os::raw::c_char;
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = 0o367 as std::os::raw::c_int as std::os::raw::c_char
            /* ,0a2 */
        }
        if *in_0 != 0 {
            let fresh28 = out;
            out = out.offset(1);
            *fresh28 = 0o101 as std::os::raw::c_int as std::os::raw::c_char
        }
    }
    *out = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return out;
}
/*danger--reuses "line" to conserve space*/
#[no_mangle]
pub unsafe extern "C" fn phpron(mut in_0: *mut std::os::raw::c_char,
                                mut out: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut current_block: u64;
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut sout: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut sflag: std::os::raw::c_char = 0;
    let mut i: std::os::raw::c_int = 0;
    sout = out;
    *sout = 0 as std::os::raw::c_int as std::os::raw::c_char;
    t = line.as_mut_ptr().offset(2 as std::os::raw::c_int as isize);
    s = t;
    u = in_0;
    loop  {
        let fresh29 = u;
        u = u.offset(1);
        let fresh30 = s;
        s = s.offset(1);
        *fresh30 = *fresh29;
        if !(*fresh30 != 0) { break ; }
    }
    s = s.offset(-(2 as std::os::raw::c_int as isize));
    sflag = 0 as std::os::raw::c_int as std::os::raw::c_char;
    if fold(t) != 0 { out = phread(t, out); if sout != out { return out } }
    if !(s == t || vowel(t, s.offset(1 as std::os::raw::c_int as isize)) < t) {
        if eflag != 0 {
            /* handle english endings*/
            sflag = finals(t, &mut s); /*,s,s or ,z,s*/
            if sflag != 0 {
                out = phread(t, out);
                if sout != out {
                    let fresh31 = out;
                    out = out.offset(1);
                    *fresh31 =
                        if sflag as std::os::raw::c_int > 1 as std::os::raw::c_int {
                            0o140 as std::os::raw::c_int
                        } else { 0o155 as std::os::raw::c_int } as std::os::raw::c_char;
                    let fresh32 = out;
                    out = out.offset(1);
                    *fresh32 = 0o140 as std::os::raw::c_int as std::os::raw::c_char;
                    current_block = 11177140147542539012;
                } else { current_block = 2370887241019905314; }
            } else { current_block = 2370887241019905314; }
            match current_block {
                11177140147542539012 => { }
                _ => {
                    midu(t, s);
                    finale(t, &mut s);
                    mide(t, &mut s);
                    mids(t, s);
                    if sflag != 0 {
                        s = s.offset(1);
                        *s = 's' as i32 as std::os::raw::c_char
                    }
                    current_block = 2719512138335094285;
                }
            }
        } else { current_block = 2719512138335094285; }
        match current_block {
            11177140147542539012 => { }
            _ => {
                t = t.offset(-1);
                *t = '#' as i32 as std::os::raw::c_char;
                s = s.offset(1);
                *s = '#' as i32 as std::os::raw::c_char;
                s = s.offset(1);
                *s = 0 as std::os::raw::c_int as std::os::raw::c_char;
                while *t != 0 {
                    t = t.offset(-1);
                    *t = '%' as i32 as std::os::raw::c_char;
                    i = prefix(t);
                    if i == 0 as std::os::raw::c_int {
                        *sout = 0 as std::os::raw::c_int as std::os::raw::c_char;
                        return sout
                    }
                    u =
                        &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)).word
                                                              as isize) as
                            *mut std::os::raw::c_char;
                    while *u != 0 {
                        t = t.offset(1);
                        if tflag != 0 {
                            write(1 as std::os::raw::c_int, u as *const std::os::raw::c_void,
                                  1 as std::os::raw::c_int as size_t);
                        }
                        u = u.offset(1)
                    }
                    if tflag != 0 {
                        write(1 as std::os::raw::c_int,
                              b" \x00" as *const u8 as *const std::os::raw::c_char as
                                  *const std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t);
                    }
                    s =
                        &mut *strings.as_mut_ptr().offset((*table.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)).phon
                                                              as isize) as
                            *mut std::os::raw::c_char;
                    loop  {
                        let fresh33 = s;
                        s = s.offset(1);
                        *out = *fresh33;
                        if !(*out != 0) { break ; }
                        if *out as std::os::raw::c_int != 0o1 as std::os::raw::c_int {
                            out = out.offset(1)
                        } else {
                            /*do replacement*/
                            u = s;
                            while *u != 0 { u = u.offset(1) }
                            loop  {
                                u = u.offset(-1);
                                if !(u >= s) { break ; }
                                t = t.offset(-1);
                                *t = *u
                            }
                            break ;
                        }
                    }
                }
            }
        }
    }
    /*spell one-letter words and vowelless words*/
    *out = 0 as std::os::raw::c_int as std::os::raw::c_char; /* monosyllable in -e */
    return out; /*perhaps suffixed*/
}
#[no_mangle]
pub unsafe extern "C" fn finals(mut in_0: *mut std::os::raw::c_char,
                                mut ls: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_char {
    let mut end: *mut std::os::raw::c_char =
        0 as *mut std::os::raw::c_char; /*monosyllables in -y, */
    let mut val: std::os::raw::c_int = 0;
    end = *ls;
    val = 0 as std::os::raw::c_int;
    if *end as std::os::raw::c_int == 's' as i32 &&
           oneof(*end.offset(-(1 as std::os::raw::c_int) as isize),
                 b"us\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut std::os::raw::c_char) == 0 {
        let fresh34 = end;
        end = end.offset(-1);
        *fresh34 = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if *end as std::os::raw::c_int == '\'' as i32 {
            let fresh35 = end;
            end = end.offset(-1);
            *fresh35 = 0 as std::os::raw::c_int as std::os::raw::c_char
        }
        val =
            oneof(*end,
                  b"cfkpt\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut std::os::raw::c_char) + 1 as std::os::raw::c_int
    }
    if *end as std::os::raw::c_int == 'e' as i32 &&
           *end.offset(-(1 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
               'i' as i32 {
        let fresh36 = end;
        end = end.offset(-1);
        *fresh36 = 0 as std::os::raw::c_int as std::os::raw::c_char;
        *end = 'y' as i32 as std::os::raw::c_char
    }
    *ls = end;
    return val as std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn midu(mut in_0: *mut std::os::raw::c_char,
                              mut end: *mut std::os::raw::c_char) {
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    s = in_0;
    while s < end.offset(-(1 as std::os::raw::c_int as isize)) {
        t = s;
        if *t as std::os::raw::c_int == 'u' as i32 &&
               oneof(*s.offset(-(1 as std::os::raw::c_int) as isize), aeiou) == 0 {
            if !(oneof(*s.offset(1 as std::os::raw::c_int as isize), aeiouwxy) != 0) {
                if *s.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'r' as i32 &&
                       oneof(*s.offset(1 as std::os::raw::c_int as isize), bcdfgkpt)
                           != 0 {
                    s = s.offset(1)
                }
                if oneof((*s.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                              | 0o40 as std::os::raw::c_int) as std::os::raw::c_char, aeiouy)
                       != 0 {
                    *t = 'U' as i32 as std::os::raw::c_char
                }
            }
        }
        s = s.offset(1)
    }
    s = in_0;
    while s < end.offset(-(2 as std::os::raw::c_int as isize)) {
        t = s;
        if oneof(*t, aeo) != 0 {
            if !(oneof(*s.offset(1 as std::os::raw::c_int as isize),
                       b"aeiouwxy|\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut std::os::raw::c_char) != 0) {
                if th(s.offset(1 as std::os::raw::c_int as isize)) != 0 {
                    s = s.offset(1)
                }
                if *s.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'r' as i32 &&
                       *s.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                           'i' as i32 &&
                       oneof(*s.offset(1 as std::os::raw::c_int as isize), bcdfgkpt)
                           != 0 {
                    s = s.offset(1)
                }
                if oneof(*s.offset(2 as std::os::raw::c_int as isize),
                         b"ie\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut std::os::raw::c_char) != 0 &&
                       oneof((*s.offset(3 as std::os::raw::c_int as isize) as
                                  std::os::raw::c_int | 0o40 as std::os::raw::c_int) as
                                 std::os::raw::c_char, aou) != 0 ||
                       *s.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                           'i' as i32 &&
                           *s.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int
                               == 'e' as i32 &&
                           *s.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
                               == 'n' as i32 {
                    *t =
                        (*t as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as
                            std::os::raw::c_char
                }
            }
        }
        s = s.offset(1)
    }
    s = in_0;
    if *s as std::os::raw::c_int == 'y' as i32 { s = s.offset(1) }
    while oneof((*s as std::os::raw::c_int | 0o40 as std::os::raw::c_int) as std::os::raw::c_char,
                aeiouy) == 0 && s < end {
        s = s.offset(1)
    }
    if oneof(*s,
             b"iy\x00" as *const u8 as *const std::os::raw::c_char as
                 *mut std::os::raw::c_char) != 0 &&
           oneof(*s.offset(1 as std::os::raw::c_int as isize), aou) != 0 {
        *s = (*s as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as std::os::raw::c_char
    };
}
#[no_mangle]
pub static mut suff0: [*mut std::os::raw::c_char; 6] =
    [b"la\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"el\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"er\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"su\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"y\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     0 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub static mut suff1: [*mut std::os::raw::c_char; 16] =
    [b"elba\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"de\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"re\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gni\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tse\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ne\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ylba\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"yl\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ro\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"yre\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ye\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ssel\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ssen\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"luf\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tnem\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     0 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub static mut suff2: [*mut std::os::raw::c_char; 3] =
    [b"ci\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"laci\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     0 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub static mut suff3: [*mut std::os::raw::c_char; 2] =
    [b"e\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     0 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn finale(mut in_0: *mut std::os::raw::c_char,
                                mut ls: *mut *mut std::os::raw::c_char) {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut z: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    end = *ls;
    if *end as std::os::raw::c_int == 'e' as i32 && vowel(in_0, end) < in_0 {
        *end = 'E' as i32 as std::os::raw::c_char;
        return
    }
    t = suffix(in_0, end, suff0.as_mut_ptr());
    if t < end { t = longe(in_0, t) }
    if t == end || t < in_0 || vowel(in_0, t) >= in_0 ||
           *t as std::os::raw::c_int == 'h' as i32 {
        t = end;
        loop  {
            u = suffix(in_0, t, suff1.as_mut_ptr());
            if !(u != t) { break ; }
            insert(u.offset(1 as std::os::raw::c_int as isize), ls);
            t = u
        }
        u = suffix(in_0, t, suff2.as_mut_ptr());
        if u != t { insert(u.offset(1 as std::os::raw::c_int as isize), ls); return }
        u = suffix(in_0, t, suff3.as_mut_ptr());
        if u != t {
            if *u.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   'e' as i32 {
                return
            }
            insert(u.offset(1 as std::os::raw::c_int as isize), ls);
            t = u
        }
        if oneof(*t,
                 b"iuy\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut std::os::raw::c_char) != 0 && vowel(in_0, t) < in_0 {
            *t = (*t as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as std::os::raw::c_char;
            return
        }
        if oneof(*t.offset(if *t.offset(1 as std::os::raw::c_int as isize) as
                                  std::os::raw::c_int == '|' as i32 {
                               2 as std::os::raw::c_int
                           } else { 1 as std::os::raw::c_int } as isize),
                 b"aeio\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut std::os::raw::c_char) == 0 {
            return
        }
        t = longe(in_0, t);
        if t < in_0 ||
               oneof(*t.offset(1 as std::os::raw::c_int as isize),
                     b"cg\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut std::os::raw::c_char) != 0 && vowel(in_0, t) >= in_0 {
            return
        }
        if th(t.offset(1 as std::os::raw::c_int as isize)) != 0 {
            *t.offset(1 as std::os::raw::c_int as isize) = 'T' as i32 as std::os::raw::c_char;
            *t.offset(2 as std::os::raw::c_int as isize) = 'H' as i32 as std::os::raw::c_char
        }
    }
    if (t == in_0 || oneof(*t.offset(-(1 as std::os::raw::c_int) as isize), aeo) == 0)
           &&
           !(*t as std::os::raw::c_int == 'e' as i32 &&
                 *t.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                     'l' as i32) {
        *t = (*t as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as std::os::raw::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn mide(mut in_0: *mut std::os::raw::c_char,
                              mut ls: *mut *mut std::os::raw::c_char) {
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    end = *ls;
    u = in_0.offset(3 as std::os::raw::c_int as isize);
    while u < end.offset(-(2 as std::os::raw::c_int as isize)) {
        if *u as std::os::raw::c_int == 'e' as i32 {
            if !(u > in_0.offset(4 as std::os::raw::c_int as isize) &&
                     syltest(u.offset(1 as std::os::raw::c_int as isize),
                             b"aeiouy|\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut std::os::raw::c_char,
                             end) != 0 &&
                     *u.offset(-(1 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
                         'l' as i32 &&
                     oneof(*u.offset(-(2 as std::os::raw::c_int) as isize),
                           b"bdfgkpt\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut std::os::raw::c_char) != 0 &&
                     oneof(*u.offset(-(3 as std::os::raw::c_int) as isize),
                           b"bcdfgmnprst\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut std::os::raw::c_char) != 0)
               {
                if syltest(u.offset(1 as std::os::raw::c_int as isize),
                           b"aeinoruy|\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut std::os::raw::c_char, end)
                       != 0 &&
                       oneof(*u.offset(-(1 as std::os::raw::c_int) as isize),
                             b"aehiouwxy\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut std::os::raw::c_char) ==
                           0 &&
                       oneof(*u.offset(-(2 as std::os::raw::c_int) as isize),
                             b"aiouyU\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut std::os::raw::c_char) != 0 &&
                       oneof(*u.offset(-(3 as std::os::raw::c_int) as isize),
                             b"aeiu\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut std::os::raw::c_char) == 0 {
                    if *u.offset(-(3 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                           != 'o' as i32 {
                        let ref mut fresh37 =
                            *u.offset(-(2 as std::os::raw::c_int) as isize);
                        *fresh37 =
                            (*fresh37 as std::os::raw::c_int & !(0o40 as std::os::raw::c_int))
                                as std::os::raw::c_char
                    }
                } else { return }
            }
            insert(u.offset(1 as std::os::raw::c_int as isize), ls);
        }
        u = u.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mids(mut in_0: *mut std::os::raw::c_char,
                              mut end: *mut std::os::raw::c_char) {
    loop  {
        in_0 = in_0.offset(1);
        if !(in_0 < end) { break ; }
        if *in_0 as std::os::raw::c_int == 's' as i32 &&
               oneof((*in_0.offset(-(1 as std::os::raw::c_int) as isize) as
                          std::os::raw::c_int | 0o40 as std::os::raw::c_int) as std::os::raw::c_char,
                     b"aeiouy\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut std::os::raw::c_char) != 0 &&
               oneof((*in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int |
                          0o40 as std::os::raw::c_int) as std::os::raw::c_char,
                     b"aeimouy\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut std::os::raw::c_char) != 0 {
            *in_0 =
                (*in_0 as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as std::os::raw::c_char
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn syltest(mut in_0: *mut std::os::raw::c_char,
                                 mut s: *mut std::os::raw::c_char,
                                 mut end: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    if oneof((*in_0 as std::os::raw::c_int | 0o40 as std::os::raw::c_int) as std::os::raw::c_char, s)
           == 0 {
        loop  {
            in_0 = in_0.offset(1);
            if !(in_0 < end) { break ; }
            if *in_0 as std::os::raw::c_int == 'e' as i32 &&
                   *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '|' as i32 {
                break ;
            }
            if *in_0 as std::os::raw::c_int == '|' as i32 { break ; }
            if oneof((*in_0 as std::os::raw::c_int | 0o40 as std::os::raw::c_int) as
                         std::os::raw::c_char, aeiouy) != 0 {
                return 1 as std::os::raw::c_int
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut in_0: *mut std::os::raw::c_char,
                                mut ls: *mut *mut std::os::raw::c_char) {
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    end = *ls;
    if *in_0 as std::os::raw::c_int == 'e' as i32 {
        in_0 = in_0.offset(1);
        if *in_0 as std::os::raw::c_int == '|' as i32 { return }
    }
    end = end.offset(1);
    s = end;
    while s >= in_0 {
        *s.offset(1 as std::os::raw::c_int as isize) = *s;
        s = s.offset(-1)
    }
    *in_0 = '|' as i32 as std::os::raw::c_char;
    *ls = end;
}
#[no_mangle]
pub unsafe extern "C" fn suffix(mut in_0: *mut std::os::raw::c_char,
                                mut end: *mut std::os::raw::c_char,
                                mut s: *mut *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut u: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    loop  {
        u = *s;
        if u.is_null() { break ; }
        t = end.offset(1 as std::os::raw::c_int as isize);
        loop  {
            t = t.offset(-1);
            if !(*u as std::os::raw::c_int == *t as std::os::raw::c_int) { break ; }
            u = u.offset(1)
        }
        if *u as std::os::raw::c_int == 0 as std::os::raw::c_int {
            if vowel(in_0, t.offset(1 as std::os::raw::c_int as isize)) < in_0 {
                break ;
            }
            return t
        } else { s = s.offset(1) }
    }
    return end;
}
#[no_mangle]
pub unsafe extern "C" fn longe(mut in_0: *mut std::os::raw::c_char,
                               mut end: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    if th(end.offset(-(1 as std::os::raw::c_int as isize))) != 0 {
        end = end.offset(-1)
    }
    return if oneof((*end as std::os::raw::c_int | 0o40 as std::os::raw::c_int) as
                        std::os::raw::c_char, aeiouwxy) == 0 &&
                  { end = end.offset(-1); (oneof(*end, aeiouy)) != 0 } {
               end
           } else { in_0.offset(-(1 as std::os::raw::c_int as isize)) };
}
#[no_mangle]
pub unsafe extern "C" fn oneof(mut c: std::os::raw::c_char, mut l: *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    while *l != 0 {
        let fresh38 = l;
        l = l.offset(1);
        if c as std::os::raw::c_int == *fresh38 as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vowel(mut in_0: *mut std::os::raw::c_char,
                               mut end: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    loop  {
        end = end.offset(-1);
        if !(end >= in_0) { break ; }
        if oneof((*end as std::os::raw::c_int | 0o40 as std::os::raw::c_int) as std::os::raw::c_char,
                 aeiouy) != 0 {
            break ;
        }
    }
    return end;
}
#[no_mangle]
pub unsafe extern "C" fn fold(mut s: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    if 'A' as i32 <= *s as std::os::raw::c_int && *s as std::os::raw::c_int <= 'Z' as i32 {
        *s = (*s as std::os::raw::c_int ^ 0o40 as std::os::raw::c_int) as std::os::raw::c_char;
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th(mut s: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    return (*s as std::os::raw::c_int == 't' as i32) as std::os::raw::c_int &
               (*s.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                    'h' as i32) as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
