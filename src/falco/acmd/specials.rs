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

unsafe extern "C" fn game_falco_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 84, 50, 0, 110, 7.5, 0.0, 7.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}

unsafe extern "C" fn effect_falco_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0ecca63d69), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			EffectModule::preset_limit_num(fighter.module_accessor, 2);
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0fdc7fb0a0), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			macros::FLASH(fighter, 1, 1, 1, 0.627);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0fecc8ba2c), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
			macros::FLASH(fighter, 0, 1, 1, 0.431);
			macros::FLASH_FRM(fighter, 20, 0, 0.706, 0.392, 0);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0ecca63d69), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0d30ab52b6), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0fdc7fb0a0), true, false);
		}
}			

unsafe extern "C" fn sound_falco_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}



pub fn install() {
    Agent::new("falco")
    .acmd("game_speciallw", game_falco_speciallw, Priority::Low)
    .acmd("effect_speciallw", effect_falco_speciallw, Priority::Low)
    .acmd("sound_speciallw", sound_falco_speciallw, Priority::Low)

    .acmd("game_specialairlw", game_falco_speciallw, Priority::Low)
    .acmd("effect_specialairlw", effect_falco_speciallw, Priority::Low)
    .acmd("sound_specialairlw", sound_falco_speciallw, Priority::Low)
    .install();
}