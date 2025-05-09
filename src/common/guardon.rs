#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
unsafe extern "C" fn sub_ftStatusUniqProcessGuardOn_initStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    if FighterUtil::is_valid_just_shield(fighter.module_accessor)
    && ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) < 6 
    && ControlModule::get_trigger_count_prev(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) > 20 {
        let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
        let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
        let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
        WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64u64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_common)]
unsafe extern "C" fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
    let shield_min_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_min_frame"));
    WorkModule::set_int(fighter.module_accessor, shield_min_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_on"), 0.0, 1.0, false, 0.0, false, false);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
        MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
        fighter.sub_ftStatusUniqProcessGuardFunc_updateShield(true.into());
    }
    fighter.sub_guard_cont_pre();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_on_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_guard_on_uniq as *const () as _));
}


#[skyline::hook(replace = L2CFighterCommon_sub_guard_on_uniq)]
unsafe extern "C" fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        fighter.FighterStatusGuard__landing_effect_control();
    }
    else {
        if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            let just_guard_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if just_guard_frame == 0 {
                ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
                let guard_type = if FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) {
                    *SHIELD_TYPE_GUARD
                }
                else {
                    *SHIELD_TYPE_UNDEFINED
                };
                ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(guard_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
                if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
                    ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
                }
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK) {
            let shield_dec1 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_dec1"));
            let shield_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_frame"), 0);
            let decrease = shield_dec1 / shield_frame;
            WorkModule::sub_float(fighter.module_accessor, decrease, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_health = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_health_min = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN);
        if shield_health < shield_health_min {
            WorkModule::set_float(fighter.module_accessor, shield_health_min, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if 0 < shield_min_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        }
        let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        if 0 < catch_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            if catch_frame <= 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOn_Main)]
unsafe extern "C" fn status_GuardOn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }
    if fighter.sub_status_guard_on_main_air_common().get_bool() {
        return 0.into();
    }
    if fighter.sub_guard_cont().get_bool() {
        return 0.into();
    }
    if fighter.status_guard_main_common().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_end_guard_on_common)]
unsafe extern "C" fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_STATUS_KIND_GUARD
    && status_kind != *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield"), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield_smoke"), true, true);
    }
    else if !param_1.get_bool() {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftStatusUniqProcessGuardOn_initStatus_common,
            sub_status_guard_on_common,
            sub_guard_on_uniq,
            status_GuardOn_Main,
            sub_status_end_guard_on_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}


























use crate::common::FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH;
use crate::common::WorkModule;
use crate::common::L2CFighterCommon_sub_guard_on_uniq;
use crate::common::L2CFighterCommon;
use crate::common::notify_event_msc_cmd;
use crate::common::lua_args;
use crate::common::effect;
use smash::lib::LuaConst;
use crate::common::L2CValue;
use crate::common::L2CFighterCommon_status_GuardOn_Main;
use crate::common::FIGHTER_STATUS_KIND_GUARD;
use crate::common::MotionModule;
use crate::common::L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common;
use crate::common::FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME;
use crate::common::hash40;
use crate::common::ShieldModule;
use crate::common::FIGHTER_REFLECTOR_GROUP_JUST_SHIELD;
use crate::common::SHIELD_STATUS_NORMAL;
use crate::common::skyline_smash::app::ShieldStatus;
use crate::common::FIGHTER_SHIELD_KIND_GUARD;
use crate::common::SHIELD_TYPE_JUST_SHIELD;
use smash::app::ShieldType;
use crate::common::FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME;
use crate::common::CONTROL_PAD_BUTTON_GUARD;
use crate::common::ControlModule;
use crate::common::skyline_smash::app::FighterUtil;
use crate::common::ReflectorModule;
use crate::common::L2CFighterCommon_sub_status_guard_on_common;
use crate::common::L2CFighterCommon_bind_address_call_sub_guard_on_uniq;
use crate::controls::consts::globals::SUB_STATUS;
use crate::common::StopModule;
use crate::common::Hash40;
use crate::common::FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION;
use crate::common::FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME;
use crate::common::FIGHTER_KIND;
use crate::common::SHIELD_TYPE_GUARD;
use crate::common::SHIELD_TYPE_UNDEFINED;
use crate::common::FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK;
use crate::common::FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD;
use crate::common::FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN;
use crate::common::FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME;
use crate::common::FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN;
use crate::common::FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT;
use crate::controls::consts::globals::STATUS_KIND;
use crate::common::FIGHTER_STATUS_KIND_GUARD_DAMAGE;
use crate::common::L2CFighterCommon_sub_status_end_guard_on_common;
use crate::common::MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND;