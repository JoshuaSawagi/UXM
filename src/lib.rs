#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

#[cfg(feature = "main_nro")]
use std::{fs, path::Path};
use skyline_web::dialog_ok::DialogOk;

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
extern crate lazy_static;

pub static mut FIGHTER_MANAGER: usize = 0;

use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;
use std::sync::atomic::{AtomicBool, Ordering};
use skyline::hooks::InlineCtx;


pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("we are on Emulator");
            return true;
        } else {
            println!("we are not on Emulator");
            return false;
        }
    }
}


extern "C" {
	fn change_version_string(arg: u64, string: *const c_char);
}
pub fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.module.isLoaded {
        return;
    }

    if info.name == "common" {
        skyline::install_hooks!(
            cpu::dmg_fly_main,
            cpu::dmg_fly_roll_main,
            cpu::dmg_main,
            cpu::dmg_air_main
        );
    }
}


unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}


static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3810684, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x374779C, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}
  
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_str = unsafe { skyline::from_c_str(string) };
	if original_str.contains("Ver.") {
        let s_ver = match std::fs::read_to_string("sd:/ultimate/mods/Ultimate S Arcropolis/version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::dialog_ok::DialogOk::ok(
                        "Ultimate S Version unknown!",
                    );
                }

                String::from("UNKNOWN")
            }
        };
		let version_str = format!("{} / Ultimate S {}\0", original_str, s_ver);
		call_original!(arg, skyline::c_str(&version_str))
	} else {
		call_original!(arg, string)
	}
}




mod util;
mod controls;
mod common;
mod cpu;
mod duckhunt;
mod falco;
mod samus;
mod fox;
mod marth;
mod roy;
mod ike;
mod wolf;
mod lucina;

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "ultimate-s"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);
#[no_mangle]
pub extern "C" fn is_ultimate_s() {}

#[no_mangle]
pub extern "C" fn main() {

    /*if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }*/
	
	//Common
    if !is_on_ryujinx(){
        println!("We're on switch! Yay!");
        unsafe {
                OFFSET1 = calc_nnsdk_offset() + 0x429d60;
                OFFSET2 = calc_nnsdk_offset() + 0x26e94;
        }
        
        skyline::install_hooks!(
                set_interval_1,
                set_interval_2,
                run_scene_update,
                vsync_count_thread,
        );
    }
    skyline::install_hooks!(change_version_string_hook);
	nro::add_hook(nro_hook).unwrap();






	
	
	util::install();
	common::install();
	controls::install();
	cpu::install();
    duckhunt::install();
    falco::install();
    fox::install();
    samus::install();
    marth::install();
    roy::install();
    ike::install();
    wolf::install();
    lucina::install();
    
    println!("added chars installed");

	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500+ 0xc80 + 0x20).nop();
    skyline::patching::Patch::in_text(0x2844128+ 0xc80 + 0x20).nop();
}
