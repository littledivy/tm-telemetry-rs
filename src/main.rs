#[repr(C)]
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct STelemetry {
    pub header: SHeader,
    pub update_number: u32,
    pub game: SGameState,
    pub race: SRaceState,
    pub object: SObjectState,
    pub vehicle: SVehicleState,
    pub device: SDeviceState,
    pub player: SPlayerState,
}

#[repr(C)]
#[derive(Debug)]
pub struct SHeader {
    pub magic: [u8; 32],
    pub version: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SGameState {
    pub state: EGameState,
    pub gameplay_variant: [u8; 64],
    pub map_id: [u8; 64],
    pub map_name: [u8; 256],
    pub future: [u8; 128],
}

#[repr(C)]
#[derive(Debug)]
pub struct SRaceState {
    pub state: ERaceState,
    pub time: u32,
    pub nb_respawns: u32,
    pub nb_checkpoints: u32,
    pub checkpoint_times: [u32; 125],
    pub nb_checkpoints_per_lap: u32,
    pub nb_laps_per_race: u32,
    pub timestamp: u32,
    pub start_timestamp: u32,
    pub future: [u8; 16],
}

#[repr(C)]
#[derive(Debug)]
pub struct SObjectState {
    pub timestamp: u32,
    pub discontinuity_count: u32,
    pub rotation: Quat,
    pub translation: Vec3,
    pub velocity: Vec3,
    pub latest_stable_ground_contact_time: u32,
    pub future: [u8; 32],
}

#[repr(C)]
#[derive(Debug)]
pub struct SVehicleState {
    pub timestamp: u32,
    pub input_steer: f32,
    pub input_gas_pedal: f32,
    pub input_is_braking: bool,
    pub input_is_horn: bool,
    pub engine_rpm: f32,
    pub engine_cur_gear: i32,
    pub engine_turbo_ratio: f32,
    pub engine_free_wheeling: bool,
    pub wheels_is_ground_contact: [bool; 4],
    pub wheels_is_slipping: [bool; 4],
    pub wheels_damper_len: [f32; 4],
    pub wheels_damper_range_min: f32,
    pub wheels_damper_range_max: f32,
    pub rumble_intensity: f32,
    pub speed_meter: u32,
    pub is_in_water: bool,
    pub is_sparkling: bool,
    pub is_light_trails: bool,
    pub is_lights_on: bool,
    pub is_flying: bool,
    pub is_on_ice: bool,
    pub handicap: u32,
    pub boost_ratio: f32,
    pub future: [u8; 20],
}

#[repr(C)]
#[derive(Debug)]
pub struct SDeviceState {
    pub euler: Vec3,
    pub centered_yaw: f32,
    pub centered_altitude: f32,
    pub future: [u8; 32],
}

#[repr(C)]
#[derive(Debug)]
pub struct SPlayerState {
    pub is_local_player: bool,
    pub trigram: [u8; 4],
    pub dossard_number: [u8; 4],
    pub hue: f32,
    pub user_name: [u8; 256],
    pub future: [u8; 28],
}

#[repr(u32)]
#[derive(Debug)]
pub enum EGameState {
    Starting = 0,
    Menus,
    Running,
    Paused,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ERaceState {
    BeforeState = 0,
    Running,
    Finished,
}

fn main() {
    use std::os::windows::ffi::OsStrExt;

    unsafe {
        let map_file = winapi::um::memoryapi::OpenFileMappingW(
            winapi::um::memoryapi::FILE_MAP_READ,
            winapi::shared::minwindef::FALSE,
            std::ffi::OsStr::new("ManiaPlanet_Telemetry\0").encode_wide().collect::<Vec<_>>().as_ptr(),
        );
    
        assert!(!map_file.is_null());
    
        let map_view = winapi::um::memoryapi::MapViewOfFile(
            map_file,
            winapi::um::memoryapi::FILE_MAP_READ,
            0,
            0,
            4096,
        );
    
        assert!(!map_view.is_null());
        
        let telemetry = unsafe { &*(map_view as *const STelemetry) };
        
        println!("{:#?}", telemetry);
    }
}