use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use std::{fs, path::Path};
use smash::phx::Vector2f;
use crate::controls::consts::globals::STICK_X;
use smash::app::sv_system;
use smash::app::BattleObjectModuleAccessor;
use crate::common::skyline_smash::app::utility;

//=================================================================
//== SHIELD STOPS
//=================================================================
unsafe extern "C" fn shield_stop(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
	if [
	*FIGHTER_STATUS_KIND_DASH,
	*FIGHTER_STATUS_KIND_TURN_DASH,
	].contains(&status) {
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD){
			fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
		}
	}             
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, shield_stop)
	.install();
    skyline::install_hooks!(
    );
}