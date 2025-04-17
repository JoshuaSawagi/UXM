/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Attack Air, used to permit momentum transfer for aerials
#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackAir)]
unsafe extern "C" fn status_pre_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

//Status Attack Air Main Common, used for continual platform drops and ECB Shifts
#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        /* START OF NEW ADDITIONS */
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if prev_status_kind == *FIGHTER_STATUS_KIND_PASS {
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                GroundModule::set_passable_check(boma, true);
            }
        }
        /* END OF NEW ADDITIONS */
        if !CancelModule::is_enable_cancel(boma) {
            if MotionModule::is_end(boma) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(boma) {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Init, inherits the initial motion of double jump
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if ![*FIGHTER_KIND_YOSHI, *FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS].contains(&fighter_kind) {
        call_original!(fighter)
    }
    else {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        fighter.sub_attack_air_kind();
        if [hash40("jump_aerial_f"), hash40("jump_aerial_b")].contains(&motion_kind) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                } 
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                }
            }
            else {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        } 
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let _ = fighter.sub_attack_air_uniq_process_init();
        0.into()
    }
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Exec, inherits the momentum of double jump
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND 
    && [*FIGHTER_KIND_YOSHI, *FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS].contains(&fighter_kind)
    && MotionModule::frame_2nd(fighter.module_accessor) >= 2.0
    && fighter.global_table[CURRENT_FRAME].get_f32() <= 5.0
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attackair,
            status_attackair_main_common,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}