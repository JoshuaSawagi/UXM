use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

unsafe extern "C" fn ike_quickdash(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
	if [
	*FIGHTER_STATUS_KIND_SPECIAL_S,
	*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH
	].contains(&status) {
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP){
			fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
		}
    }         
}




pub fn install() {
    Agent::new("ike")
    .on_line(Main, ike_quickdash)
    .install();
}