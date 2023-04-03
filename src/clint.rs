use core::{ptr::null_mut, sync::atomic::AtomicPtr};

use aclint::SifiveClint;

pub static CLINT: AtomicPtr<SifiveClint> = AtomicPtr::new(null_mut());

pub(crate) fn init(base: usize) {
    CLINT.store(base as _, core::sync::atomic::Ordering::Release);
}

#[inline]
#[allow(clippy::needless_borrow)]
pub fn read_mtime() -> u64 {
    unsafe { (&*CLINT.load(core::sync::atomic::Ordering::Relaxed)).read_mtime() }
}
