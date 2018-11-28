use crate::main;

#[repr(C)]
struct Argv {
    argv_magic: u32,
    command_line: *const u8,
    length: u32,
    argc: u32,
    argv: *const *const u8,
}

extern {
    static __system_argv: *const Argv;
    fn SYS_PreMain();
}

#[no_mangle]
pub unsafe extern fn __crtmain() {
    SYS_PreMain();
    //main((*__system_argv).argc, (*__system_argv).argv)
    main()
}
