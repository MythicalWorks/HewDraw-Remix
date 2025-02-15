// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn duck_jump_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY {
        if frame > 20.0 {
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END, true);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    duck_jump_cancel(boma, status_kind, cat[0], frame);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame <= 16.0 {
            MotionModule::set_rate(boma, 2.0);
        }
        if frame > 16.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_DUCKHUNT )]
pub fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		duckhunt_frame(fighter)
    }
}

pub unsafe fn duckhunt_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}