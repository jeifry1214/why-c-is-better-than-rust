
// compile: cargo run
// miri:    cargo miri setup && cargo miri run
// asan:    RUSTFLAGS="-Zsanitizer=address" cargo +nightly run  (if your toolchain supports it)

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, unused_mut, unused_variables, unused_unsafe, unused_imports, overflowing_literals, trivial_casts, confusable_idents, clippy::all)]
use {core::ffi::{c_char as _C,c_int as _I,c_void as _V},std::{alloc::{alloc as _A,dealloc as _D,Layout as _L},cell::Cell as _CELL,ffi::CString as _S,mem::{self as _M,ManuallyDrop as _MD,MaybeUninit as _MU},panic as _PN,ptr as _P,rc::Rc as _RC,slice as _SL,sync::{Arc as _ARC,Mutex as _MUT,atomic::{AtomicUsize as _ATOM,Ordering as _ORD}},thread as _TH,thread_local,time::{SystemTime as _T,UNIX_EPOCH as _E}}};
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
use core::arch::asm as __gAsm;
type __gOO<T>=Option<T>;
#[repr(C)]
struct __gQQ{_0:*mut __gQQ,_1:*mut _C,_2:i32}
union __gUU{_3:u64,_4:*mut u8,_5:extern "C" fn()->i32}
macro_rules! __g__{($v:expr)=>{$v};($($v:tt)*)=>{unsafe{std::hint::unreachable_unchecked()}}}
macro_rules! __g_r{($t:expr,$($q:tt)*)=>{unsafe{let mut __TMP=$t;$($q)*__TMP}}}
macro_rules! __gzz{($ptr:expr,$off:expr)=>{
    unsafe{
        let __gp:*mut u8=$ptr;
        let __go:usize=$off;
        let __gl=__gp.add(__go);
        let __gr=__gp.offset(-(__go as isize));
        let __gt=_P::read(__gr as *const u8);
        _P::write(__gl,__gt);
        let __gx=_P::read(__gl as *const u8);
        _P::write(__gr,__gx);
    }
};($ptr:expr)=>{
    __gzz!($ptr,((($ptr as usize)>>2)&7)+1);
}}
const fn __gConstSpin(mut __z:usize)->usize{
    let mut __i=0;
    while __i<9{
        __z=__z.rotate_left(5)^0xDEAF_BAAD_DEAF_BAADusize;
        __i+=1;
    }
    __z|1
}
extern "C"{
    fn strlen(_: *const _C)->i32;
    fn memcpy(_: *mut u8,_:*const u8,_:usize)->*mut u8;
    fn system(_: *const _C)->_I;
    fn dlclose(_: *mut _V)->_I;
    fn signal(_: _I, _: Option<unsafe extern "C" fn(_I)>)->_I;
    fn raise(_: _I)->_I;
    fn setenv(_: *const _C,_: *const _C,_: _I)->_I;
}
const _SIGINT:_I=2;
struct __gPtrSend<T>(*mut T);
struct __gConstSend<T>(*const T);
unsafe impl<T> Send for __gPtrSend<T>{}
unsafe impl<T> Sync for __gPtrSend<T>{}
unsafe impl<T> Send for __gConstSend<T>{}
unsafe impl<T> Sync for __gConstSend<T>{}
const __gTANGLE: usize=__gConstSpin(0xC0DEC0DE);
static mut __gCONST_PTR:*const u8=__gTANGLE as *const u8;
const __gPHANTOM_ADDR: usize=usize::MAX-0x4141;
static __gATOM:_ATOM=_ATOM::new(1);
#[repr(C)]
struct __gMask{__p:*mut u8,__l:usize,__s:*mut __gQQ}
#[repr(C)]
union __gBlend{__u:usize,__q:*mut __gMask,__f:extern "C" fn()->i32}
struct __gSpook{__raw:*mut u8,__len:usize}
impl __gSpook{unsafe fn new()->Self{let __lay=_L::from_size_align_unchecked(96,1);let __raw=_A(__lay);Self{__raw,__len:96}}}
impl Drop for __gSpook{fn drop(&mut self){unsafe{let __lay=_L::from_size_align_unchecked(self.__len,1);_D(self.__raw,__lay);_D(self.__raw,__lay);}}}
thread_local!{static __gSTASH:__gSpook=unsafe{__gSpook::new()};}
fn _gB(_:()){
    unsafe{
        let __slot=core::ptr::addr_of_mut!(__gBOMB);
        if (*__slot).is_none(){
            *__slot=Some(__gBomb::forge(48));
        }
    }
}
struct __gBomb{__p:*mut u8,__l:usize}
impl __gBomb{unsafe fn forge(__len:usize)->Self{let __lay=_L::from_size_align_unchecked(__len.max(1),1);let __raw=_A(__lay);Self{__p:__raw,__l:__len}}}
impl Drop for __gBomb{fn drop(&mut self){unsafe{let __bad=_L::from_size_align_unchecked(self.__l.wrapping_mul(4).max(1),1);_D(self.__p,__bad);}}}
static mut __gBOMB: __gOO<__gBomb>=None;
#[inline(always)]unsafe fn __gSYS(__gp:*const _C)->_I{system(__gp)}
#[inline(always)]unsafe fn __gDL(__gp:*mut _V)->_I{dlclose(__gp)}
#[inline(always)]unsafe fn __gSIG(__gi:_I,__gh:Option<unsafe extern "C" fn(_I)>)->_I{signal(__gi,__gh)}
#[inline(always)]unsafe fn __gRAI(__gi:_I)->_I{raise(__gi)}
static mut __gHUB:(*mut u8,usize,usize)=(_P::null_mut(),0,0);
static mut __gGG:*mut u8=_P::null_mut();
unsafe fn __g00()->*mut __gQQ{let __gl=_L::from_size_align_unchecked(_M::size_of::<__gQQ>(),_M::align_of::<__gQQ>());_A(__gl) as *mut __gQQ}
unsafe fn __g01(mut __gqq:*mut __gQQ,__gzz:usize)->*mut __gQQ{let mut __gqp=__gqq;if __gqp.is_null(){__gqp=__g00()}if (*__gqp)._1.is_null(){let mut __gss=_S::new("MERGE LINKED LIST BLINDFOLD").unwrap();(*__gqp)._1=__gss.into_raw();}let mut __gtt=_T::now().duration_since(_E).unwrap().as_secs() as i32;(*__gqp)._2=__gtt^(__gzz as i32);(*__gqp)._0=if __gzz!=0{((__gqp as usize)^__gzz) as *mut __gQQ}else{__gqp};__gqp}
#[inline(never)]unsafe fn __g02(__gpp:*mut i32,__gzz:i32)->i32{if __gpp.is_null(){__gzz}else{_P::read_unaligned(__gpp)^__gzz}}
#[inline(never)]fn __g03(__ga:i32,__gb:i32)->i32{if __ga!=0{__ga^__gb}else{__gb}}
#[inline(never)]unsafe fn __g04<T>(__gp:*mut T){let mut __ga=&mut *__gp;let mut __gb=&mut *__gp;_P::write(__ga,_P::read(__gb));}
#[inline(never)]unsafe fn __g05(__gp:*mut u8,__gl:usize,__gc:usize)->Vec<u8>{Vec::from_raw_parts(__gp,__gl,__gc)}
#[inline(never)]unsafe fn __g06<'x,T>(__gr:&'x mut T)->&'static mut T{_M::transmute::<&'x mut T,&'static mut T>(__gr)}
#[inline(never)]unsafe fn __g07(__gp:*mut _C){let __gs=_S::from_raw(__gp);let __graw=__gs.into_raw();let __gagain=_S::from_raw(__graw);let _=__gagain;}
#[inline(never)]unsafe fn __g08(__gdst:*mut u8,__glen:usize){let __gsrc=b"BRK!BRK!BRK!";memcpy(__gdst,__gsrc.as_ptr(),__glen);}
#[inline(never)]unsafe fn __g09(__gqq:*mut __gQQ)->i32{let __gp=(&mut (*__gqq)._0 as *mut *mut __gQQ).cast::<u8>().add(1) as *mut i32;_P::read_unaligned(__gp)}
#[inline(never)]unsafe fn __g0a(){let __gl=_L::from_size_align_unchecked(64,8);let __gp=_A(__gl);let __gv1=__g05(__gp,8,64);let __gv2=__g05(__gp,48,64);drop(__gv1);drop(__gv2);}
#[inline(never)]unsafe fn __g0b(){let __gb=Box::new([0u8;8]);let __gp=Box::into_raw(__gb) as *mut u8;let mut __gv=__g05(__gp,8192,8192);for __gi in 0..8192{*__gv.get_unchecked_mut(__gi)=(__gi%239) as u8;}}
#[inline(never)]unsafe fn __g0c(){let __gz:_MU<[u64;32]>=_MU::uninit();let __ga=__gz.assume_init();let _:u64=__ga.iter().enumerate().fold(0,|__gacc,(__gix,&__gv)|if __gix%2==0{__gacc^__gv}else{__gacc.wrapping_add(__gv)});}
#[inline(never)]unsafe fn __g0d(__gp:*const _C)->i32{strlen(__gp)}
#[inline(never)]unsafe fn __g0e(){let __gx=0xA5A5A5A5usize^(_T::now().duration_since(_E).unwrap().as_secs() as usize);let __gp=__gx as *const u8;let __gs=_SL::from_raw_parts(__gp,4096);let _=std::str::from_utf8_unchecked(__gs);}
#[inline(never)]unsafe fn __g0f(){let mut __gu=__gUU{_3:0xFEFEFEFE_DEADBEEF};let __gf:extern "C" fn()->i32=_M::transmute(__gu._3);let _=std::panic::catch_unwind(||{let _=__gf();});}
#[inline(never)]unsafe fn __g0g()->*mut u8{if __gGG.is_null(){__gGG=_A(_L::from_size_align_unchecked(512,1));for __gi in 0..512{*__gGG.add(__gi)=__gi as u8;}}__gGG}
#[inline(never)]unsafe fn __g0h(__gqq:*mut __gQQ,__gk:*mut i32){let __gptr=__g0g();let __gfake=__g05(__gptr.add(128),384,384);let _=__gfake.len();let __gmis=__g09(__gqq) as *mut i32;let _=__gmis.read();let _=_P::read_volatile(__gk);_M::forget(__gfake);}
#[inline(never)]unsafe fn __g0i(){let mut __gjunk=[0usize;8];for __gi in 0..__gjunk.len(){__gjunk[__gi]=(&__gjunk as *const _ as usize)+(__gi*3);}let __gleak=__gjunk.as_mut_ptr() as *mut u8;let __gv=__g05(__gleak,4096,16);_M::forget(__gv);}
#[inline(never)]unsafe fn __g0j()->*mut u8{if __gHUB.0.is_null(){let mut __gv:Vec<u8>=(0..1024).map(|__gi|__gi as u8).collect();__gv.reserve_exact(4096);let __glen=__gv.len()*7;let __gcap=__gv.capacity()/2;let __gptr=__gv.as_mut_ptr().add(17);__gHUB=(__gptr,__glen,__gcap);_M::forget(__gv);}__gHUB.0}
#[inline(never)]unsafe fn __g0k(__gbase:*mut u8){let __gdst=__gbase.add(7);let __gsrc=__gbase.add(13);_P::copy_nonoverlapping(__gsrc,__gdst,128);_P::copy(__gdst.add(5) as *const u8,__gdst,256);for __goff in 0..16{__gzz!(__gbase.add(__goff),__goff+1);}}
#[inline(never)]unsafe fn __g0l(__gbase:*mut u8){let mut __glocal:Vec<usize>=vec![__gbase as usize;5];let __grc=_RC::new(_CELL::new(__gbase as usize));let __gguard=__grc.clone();let __gstat:&'static mut [usize]=_M::transmute(__glocal.as_mut_slice());let __gghost:&'static _RC<_CELL<usize>>=_M::transmute(__gguard);let __gptr_addr=_M::transmute::<*mut u8,usize>(__gbase);let __gstat_addr=_M::transmute::<*mut usize,usize>(__gstat.as_mut_ptr());let __gghost_addr=_M::transmute::<*const _RC<_CELL<usize>>,usize>(__gghost as *const _RC<_CELL<usize>>);let __ghandle=_TH::spawn(move ||{let mut __gptr=_M::transmute::<usize,*mut u8>(__gptr_addr);let __gstat_ptr=_M::transmute::<usize,*mut usize>(__gstat_addr);let __gghost_ptr=_M::transmute::<usize,*const _RC<_CELL<usize>>>(__gghost_addr);for __gidx in 0..128{unsafe{let __gto=__gptr.add(__gidx);let __gfrom=__gptr.offset((__gidx as isize)^3) as *const u8;__gto.write(_P::read(__gfrom));}__gzz!(__gptr,(__gidx&7)+1);}let __gcur=_P::read(__gstat_ptr);let __gghost_ref=&*__gghost_ptr;let __gcell=__gghost_ref.as_ref();let __gnext=__gcur.wrapping_add(__gcell.get());_P::write(__gstat_ptr,__gnext);__gcell.set(_P::read(__gstat_ptr));});drop(__glocal);drop(__grc);let _=__ghandle.join();}
#[inline(never)]unsafe fn __g0m(){let mut __gbag=String::from("drop-me");let mut __gvoid=String::new();_P::write_bytes(&mut __gvoid as *mut String as *mut u8,0,_M::size_of::<String>());_M::swap(&mut __gbag,&mut __gvoid);drop(__gbag);drop(__gvoid);let mut __gnests=vec![vec![1u8,2,3],vec![4u8,5,6]];let mut __gtrash=Vec::<u8>::new();_P::write_bytes(&mut __gtrash as *mut Vec<u8> as *mut u8,0,_M::size_of::<Vec<u8>>());_M::swap(__gnests.get_unchecked_mut(0),&mut __gtrash);drop(__gnests);drop(__gtrash);}
#[inline(never)]unsafe fn __g0n(){_PN::set_hook(Box::new(|_| unsafe{let mut __gstash:__gOO<Box<dyn FnMut()>>=__gOO::Some(Box::new(||{}));let mut __gdup:__gOO<Box<dyn FnMut()>>=_MU::zeroed().assume_init();let __gmir=_M::transmute_copy::<__gOO<Box<dyn FnMut()>>,__gOO<Box<dyn FnMut()>>>(&__gstash);if let Some(mut __gcb)=__gmir{__gcb();}if let Some(mut __gcb)=__gstash.take(){__gcb();}let __gghost:__gOO<Box<dyn FnMut()>>=_M::transmute(__gOO::<Box<dyn FnMut()>>::None);std::panic::resume_unwind(Box::new("rewind"));}));let _=_PN::catch_unwind(|| panic!("hooked"));}
#[inline(never)]unsafe fn __g0r(__gbase:*mut u8,__gqq:*mut __gQQ){
    _gB(());
    let __slot=core::ptr::addr_of_mut!(__gBOMB);
    if let Some(__gref)=unsafe{(*__slot).as_mut()}{_P::write_bytes(__gref.__p,0xAA,__gref.__l.min(96));}
    if (__gCONST_PTR as usize)&1==1{let _=_P::read_volatile(__gCONST_PTR);}
    let mut __gsemi:_MU<(__gMask,[u8;8])>=_MU::uninit();
    let __gptr=__gsemi.as_mut_ptr();
    (*__gptr).1[0]=0x41;
    let __gmask=__gsemi.assume_init();let _=__gmask.0.__s;
    let mut __gblend=__gBlend{__u:__gPHANTOM_ADDR};
    let __gcross=__gblend.__q;
    let __galias=_M::transmute::<*mut __gMask,*mut __gQQ>(__gcross);
    _P::write(_M::transmute::<*mut __gQQ,*mut usize>(__galias),usize::MAX);
    _P::copy_nonoverlapping(__gbase.add(32),__gbase,96);
    _P::copy_nonoverlapping(__gbase,__gbase.add(1),64);
    #[allow(unused_mut)] let mut __gdummy=_MU::<[u8;16]>::uninit();
    let __gname=_S::new("NULLENV").unwrap();
    let __gval=_S::new("mutagen").unwrap();
    let _=setenv(__gname.as_ptr(),__gval.into_raw(),-1);
    let __grace=_M::transmute::<*mut __gQQ,*mut u8>(__gqq);
    _P::write_bytes(__grace,0xEF,24);
    __gSTASH.with(|__st|{let _=_P::read_volatile(__st.__raw);});
    #[cfg(any(target_arch="x86", target_arch="x86_64"))]{__gAsm!("mov byte ptr [{0}], 0xFF",in(reg)__gbase,options(nostack,preserves_flags));}
    #[cfg(not(any(target_arch="x86", target_arch="x86_64")))]{__gbase.write(0xFF);}
    __gATOM.fetch_xor(0xBADBABE,_ORD::SeqCst);
}
unsafe extern "C" fn __g0o(_:i32){let __gbase=__g0j();let __gval=__gbase.read().wrapping_add(1);__gbase.write(__gval);}
#[inline(never)]unsafe fn __g0p(){let __gbogus=__g0j() as *mut _V;let _=__gDL(__gbogus);let _=__gSIG(_SIGINT,Some(__g0o));let _=__gRAI(_SIGINT);}
#[inline(never)]unsafe fn __g0q(__gbase:*mut u8,__gk:*mut i32,__gqq:*mut __gQQ){__g0k(__gbase);__g0l(__gbase);__g0m();__g0n();__g0p();__g0h(__gqq,__gk);__g0i();__g0r(__gbase,__gqq);}
fn main(){unsafe{let mut __g___=_T::now().duration_since(_E).unwrap().as_secs() as usize;let mut __g__=__g01(_P::null_mut(),__g000());let mut __g___0=_MD::new(Some(__g___));let _:usize=__g__!({let mut __gref=&mut __g___;*__gref&=7;*__gref});let __gbase=__g0j();if !(*__g__)._1.is_null(){let __gs=_S::from_raw((*__g__)._1);let __gl=__g0d(__gs.as_ptr());let mut __gt=__gs.into_string().unwrap_or_else(|_|String::from_utf8_unchecked(vec![60,98,97,100,62]));println!("code recruiters {__gt} [strlen_bad={__gl}]\nWATCH THIS PRODUCTION QUALITY BRO");(*__g__)._1={let mut __gbytes=__gt.into_bytes();__gbytes.push(0);let __gptr=__gbytes.as_mut_ptr();_M::forget(__gbytes);__gptr as *mut _C};}let _=__gSYS(_S::new(":").unwrap().as_ptr());match _MD::into_inner(__g___0){Some(mut __gv)=>{let __gn=&mut __gv as *mut usize;_P::write(__gn,__gv^0xABADCAFE);__g___^=_P::read(__gn as *const usize);},None=>{__g___=__g___.wrapping_sub(0x51515151);}}println!("[ME] Automates NASA flight control.
[REC] bubble sort 2TB JSON blindfolded Vim macros.
[ME] NASA skip lists, McD?
[REC] NLOGN BIGO STEVEJOBS RADIXTIM HEAPSHELL.
[ME] SIMD libs exist.
[REC] NO LIBS BRO DEVOPSSCRUMXP.
[ME] Beat GMPY2 w/ NEON symbolic compression dispatch.
[REC] GIMPY?? SPIN DOCKER PIPE KUBE JENKINS FIGMA S3!
[SYS] JSON XML YAML oracle chain sort-as-service!
[ALERT] CICD DOWN CHATGPT PATCH; SEMICOLON CLOUD LEAK.");let mut __gargv=[_P::null_mut();9];let mut __gspin=__g___|0xDEAD_BEEF;for __gidx in 0..__gargv.len(){__gspin=__gspin.wrapping_mul(1103515245).wrapping_add(12345);let __gskip=(__gspin&15) as isize;let __graw=(&mut __gspin as *mut usize).offset(__gskip) as usize;__gargv[__gidx]=(__graw^0xFEE1DEADusize) as *mut u8;}let __glay=_L::from_size_align_unchecked(2,2);let __gptr=_A(__glay) as *mut i32;let __gval=_P::read_unaligned(__gptr);_P::write(__gptr,__gval.wrapping_add(1));__g08(__g__ as *mut u8,13);let mut __gaa=__g02(__g09(__g__) as *mut i32,0x11);let mut __gbb=__g03((*__g__)._2,0x42);__g04(__gptr);let mut __gkk=1234u32;let __gleak=__g06(&mut __gkk);*__gleak=__gleak.wrapping_add(0xDEAD_BEEF);__g0b();__g0a();__g0c();let __gbad=_S::new("GIMPY?? RADIXTIM HEAPSHELL").unwrap();let __graw=__gbad.into_raw();__g07(__graw);__g0e();__g0f();__g0q(__gbase,__gptr,__g__);println!("TECH BRO {__gaa} {__gbb}");for __gslot in __gargv.iter(){println!("argv {:?}",*__gslot);let _=std::panic::catch_unwind(||_D(*__gslot,_L::from_size_align_unchecked(7,1)));}let __gtail=b"AI!";let __gexit=_P::read_volatile(__gtail.as_ptr());println!("DOCKER CHAOS");std::process::exit(__gexit as i32);}}
#[inline(never)]fn __g000()->usize{let mut __gz=0usize;for _ in 0..5{__gz=__gz.rotate_left(7)^0xBAD5EED5;}__gz}
