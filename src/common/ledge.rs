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
use crate::common::skyline_smash::app;
use crate::controls::consts::FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID;
static mut LEDGE_POS: [smash::phx::Vector3f; 8] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 }; 8];

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]
unsafe extern "C" fn ledge_cling(fighter: &mut app::BattleObjectModuleAccessor) -> bool {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = GroundModule::hang_cliff_pos_3f(fighter);

    for i in 0..8 {
        if i == entry_id || LEDGE_POS[i].x == 0.0 {
            continue;
        }

        if (pos.x - LEDGE_POS[i].x).abs() < 1.0 && (pos.y - LEDGE_POS[i].y).abs() < 1.0 {
            return false;
        }
    }

    original!()(fighter)
}

#[skyline::hook(replace = GroundModule::entry_cliff)]
unsafe fn entry_cliff(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = GroundModule::hang_cliff_pos_3f(fighter);
    LEDGE_POS[entry_id] = pos;

    original!()(fighter);
}

#[skyline::hook(replace = GroundModule::leave_cliff)]
unsafe fn leave_cliff(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

    original!()(fighter);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffWait)]
unsafe fn status_end_cliffwait(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ![*FIGHTER_STATUS_KIND_CLIFF_WAIT, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_JUMP1].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    }
    call_original!(fighter)
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_CliffCatch)]
unsafe fn status_pre_cliffcatch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_CLIFF_ROBBED);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatch)]
unsafe fn status_end_cliffcatch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatchMove)]
unsafe fn status_end_cliffcatchmove(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    }
    original!()(fighter)
}
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffClimb)]
unsafe fn status_end_cliffclimb(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffEscape)]
unsafe fn status_end_cliffescape(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump1)]
unsafe fn status_end_cliffjump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump2)]
unsafe fn status_end_cliffjump2(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffJump3)]
unsafe fn status_end_cliffjump3(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}


pub fn install() {
    skyline::install_hooks!(
        ledge_cling,
        entry_cliff,
        leave_cliff,
        status_pre_cliffcatch,
        status_end_cliffcatch,
        status_end_cliffwait,
        status_end_cliffcatchmove,
        status_end_cliffclimb,
        status_end_cliffescape,
        status_end_cliffjump1,
        status_end_cliffjump2,
        status_end_cliffjump3
    );
}