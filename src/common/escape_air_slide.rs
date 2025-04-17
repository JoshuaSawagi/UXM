use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use crate::util::*;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use std::sync::LazyLock;
use std::collections::HashSet;
use crate::common::skyline_smash::app::BattleObjectModuleAccessor;

static mut IS_WAVEDASH: [bool; 8] = [false; 8];  
static mut WAVEDASH_DONE: [bool; 8] = [false; 8];  

const TRACTION_MAX: f32 = 0.086;
const TRACTION_HIGH: f32 = 0.075;
const TRACTION_MID: f32 = 0.06;
const TRACTION_LOW: f32 = 0.054;
const TRACTION_MIN: f32 = 0.045;

// Returns traction values for different fighters  
fn get_wd_length(fighter_kind: i32) -> f32 {
    let max = [
        *FIGHTER_KIND_FOX, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_GEKKOUGA,
        *FIGHTER_KIND_FALCO, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_INKLING,
        *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_JACK, *FIGHTER_KIND_SONIC,
    ];
    let high = [
        *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_IKE, *FIGHTER_KIND_CLOUD,
        *FIGHTER_KIND_CHROM, *FIGHTER_KIND_ROY, *FIGHTER_KIND_LUCINA,
        *FIGHTER_KIND_MARTH, *FIGHTER_KIND_KEN, *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_TANTAN,
    ];
    let mid = [
        *FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB, *FIGHTER_KIND_LINK, *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_SHULK,
        *FIGHTER_KIND_MIIENEMYF, *FIGHTER_KIND_MIIENEMYG, *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_ZELDA,
        *FIGHTER_KIND_ELEMENT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_PICKEL,
        *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_BUDDY,
    ];
    let low = [
        *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_WARIO, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_SIMON,
        *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_PEACH, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_KOOPAG,
        *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_SHIZUE,
    ];
    let min = [
        *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_FUSHIGISOU, *FIGHTER_KIND_LIZARDON, *FIGHTER_KIND_ZENIGAME,
        *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PTRAINER,
        *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_MASTER, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO,
        *FIGHTER_KIND_ICE_CLIMBER, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_DEMON,
    ];

    if max.contains(&fighter_kind) { return TRACTION_MAX; }
    if high.contains(&fighter_kind) { return TRACTION_HIGH; }
    if low.contains(&fighter_kind) { return TRACTION_LOW; }
    if min.contains(&fighter_kind) { return TRACTION_MIN; }

    TRACTION_MID
}

// Applies wavedash mechanics  
unsafe extern "C" fn wavedash(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if status_kind == *FIGHTER_STATUS_KIND_LANDING {
        // Apply wavedash sliding when landing
        let desired_brake = get_wd_length(fighter_kind);
        let brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
        let speed = get_speed_x(boma) * PostureModule::lr(boma);
        let mut added_speed = brake - desired_brake;

        if speed < 0.0 { added_speed *= -1.0; }
        if (speed <= 0.0 && (speed + added_speed) > 0.0) || (speed >= 0.0 && (speed + added_speed) < 0.0) {
            added_speed = 0.0;
        }

        let speed = smash::phx::Vector3f { x: added_speed, y: 0.0, z: 0.0 };
        KineticModule::add_speed(boma, &speed);
    }

    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        IS_WAVEDASH[entry_id] = true;
    }

    if ![*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
        IS_WAVEDASH[entry_id] = false;
        WAVEDASH_DONE[entry_id] = false;
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Exec, wavedash)
        .install();
    skyline::install_hooks!(
    );
}
