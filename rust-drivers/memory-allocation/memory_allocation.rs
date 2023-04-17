// SPDX-License-Identifier: GPL-2.0i

//! Rust memory allocation sample

use kernel::prelude::*;
use kernel::{
    mm::virt::Area,
    miscdev::Registration,
    file::{self, File}
};

use alloc::alloc::{alloc, dealloc, Layout};

module! {
    type: AllocationDriver,
    name: "memory_allocation",
    author: "Noe Terrier",
    description: "Rust memory allocation example module",
    license: "GPL",
}

struct RustFile;

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &File) -> Result<Box<Self>> {
        Ok(Box::try_new(Self {})?)
    }

    fn mmap(
        _this: &Self,
        _file: &File,
        vma: &mut Area,
    ) -> Result {
        const PAGE_SIZE : usize = 4096;

        pr_info!("Begin of mmap");
        let size = vma.end() - vma.start();
        
        // size must be positive
        if (size <= 0) {
            pr_err!("End is smaller than start");
            return Err(EINVAL);
        }

        if(vma.start() % PAGE_SIZE != 0 || vma.end() % PAGE_SIZE != 0) {
            pr_err!("End or/Start is/are not page-aligned.");
            return Err(EINVAL);
        }

        let layout = match Layout::from_size_align(size, PAGE_SIZE) {
            Ok(l) => l,
            Err(_) => return Err(EINVAL),
        };

        // allocation functions are unsafe
        unsafe {
            let ptr = alloc(layout);
            
            if ptr.is_null() {
                pr_info!("Allocation failed\n");
                return Err(EINVAL)
            }

            // test that allocation succeed
            *(ptr as *mut u16) = 42;
            assert_eq!(*(ptr as *mut u16), 42);
            
            dealloc(ptr, layout);
            pr_info!("Allocation succeed"); // ?? this is never printed
        }
        
        Ok(())
    }
}



// module definition and registration
struct AllocationDriver {
    _dev: Pin<Box<Registration<RustFile>>>,
}


impl kernel::Module for AllocationDriver {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("memory_allocation: (init)\n");
        
        Ok(Self {
            _dev: Registration::new_pinned(fmt!("{name}"), ())?,
        })
    }
}

impl Drop for AllocationDriver {
    fn drop(&mut self) {
        pr_info!("memory_allocation: (exit)\n");
    }
}
