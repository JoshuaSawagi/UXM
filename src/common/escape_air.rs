use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::SituationKind;
use crate::controls::consts::globals::PAD_FLAG;
use crate::controls::consts::globals::STICK_Y;
use crate::controls::consts::globals::STICK_X;
use crate::controls::consts::globals::CURRENT_FRAME;
use super::*;

//Escape Air
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    } 
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_escapeair_main as *const () as _))
}

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if frame <= 29.0 {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.05);
        }
        if (4.0..=11.0).contains(&frame) {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.00); // Soft gravity
        }
        if (12.0..=29.0).contains(&frame) {
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if frame > 29.0 {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_escapeair
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}