use crate::bsp;
use crate::{info, print, println};
use cortex_a::{asm, regs::*};

#[repr(C)]
pub struct ELFHeader {
    pub magic: u32,
    pub elf: [u8; 12],
    pub etype: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader {
    pub ptype: u32,
    pub flags: u32,
    pub off: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
}

const ELF_PROG_LOAD: u32 = 1;
const ELF_PROG_FLAG_EXEC: u32 = 1;
const ELF_PROG_FLAG_WRITE: u32 = 2;
const ELF_PROG_FLAG_READ: u32 = 4;
const ELF_MAGIC: u32 = 0x464C457F;

pub fn run_elf<const N: usize>(a: &'static [u8; N]) {
    /* TODO: Use something safer */
    // peek head of byte array to get ELF information
    let elfhdr = & unsafe { core::mem::transmute::<&[u8], &[ELFHeader]>(a) } [0];
    if elfhdr.magic != ELF_MAGIC {
        info!("wrong magic number!");
        return;
    }

        /*
        for(i=0, off=elf.phoff; i<elf.phnum; i++, off+=sizeof(ph)){
        if(readi(ip, 0, (uint64)&ph, off, sizeof(ph)) != sizeof(ph))
        goto bad;
        if(ph.type != ELF_PROG_LOAD)
        continue;
        if(ph.memsz < ph.filesz)
        goto bad;
        if(ph.vaddr + ph.memsz < ph.vaddr)
        goto bad;
        if((sz = uvmalloc(pagetable, sz, ph.vaddr + ph.memsz)) == 0)
        goto bad;
        if(ph.vaddr % PGSIZE != 0)
        goto bad;
        if(loadseg(pagetable, ph.vaddr, ip, ph.off, ph.filesz) < 0)
        goto bad;
    }
    */
    let mut proghdr = unsafe {
        let offset_u8 = (&a[0] as *const u8).offset(elfhdr.phoff as isize);
        offset_u8 as *const ProgramHeader
    };
    for i in 0..elfhdr.phnum {
        let hdr : &ProgramHeader = unsafe {
            let hdr = &*proghdr;
            proghdr = proghdr.offset(1);
            hdr
        };
        if hdr.ptype != ELF_PROG_LOAD {
            /*
            unsafe {
                let mut x = proghdr as *const u8;
                for i in 0..16 {
                    print!("{:X} ", *x);
                    x = x.offset(1);
                }
            }
            */
            info!("unsupported program segment");
            continue;
        }
        info!("loading segment...");
        
    }
    // Set up a simulated exception return.
    //
    // First, fake a saved program status, where all interrupts were masked and SP_EL0 was used as a
    // stack pointer.
    SPSR_EL1.write(
        SPSR_EL1::D::Masked
            + SPSR_EL1::A::Masked
            + SPSR_EL1::I::Masked
            + SPSR_EL1::F::Masked
            + SPSR_EL1::M::EL1h,
    );

    // Second, let the link register point to init().
    ELR_EL1.set(crate::runtime_init::runtime_init as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it.
    SP_EL0.set(bsp::BOOT_CORE_STACK_START);

    // Use `eret` to "return" to EL1. This will result in execution of `reset()` in EL1.
    asm::eret()
}
