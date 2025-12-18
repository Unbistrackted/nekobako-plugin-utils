use serde::de::DeserializeOwned;
use serde::Serialize;

use skyline::hooks::{getRegionAddress, Region};
use skyline_config::{SdCardStorage, StorageHolder};

const BUILD_INFO: [u8; 20] = [
    0x76, 0x16, 0xf8, 0x96, 0x3d, 0xac, 0xcd, 0x70, 0xe2, 0x0f, 0xf3, 0x90, 0x4e, 0x13, 0x36, 0x7f,
    0x96, 0xf2, 0xd9, 0xb3,
];

#[macro_export]
macro_rules! is_enabled {
    ($config:expr) => {{
        if unsafe { !$crate::has_same_build_info() } {
            false
        } else if !$config.is_enabled {
            false
        } else {
            true
        }
    }};
}

pub fn get_or_generate_config<T>(plugin_name: &str) -> T
where
    T: DeserializeOwned + Serialize + Default,
{
    let path = format!(
        "atmosphere/contents/01006A300BA2C000/romfs/skyline/config/{}",
        plugin_name
    );

    let sd_storage = SdCardStorage::new(path);
    let mut storage_holder = StorageHolder::new(sd_storage);

    //Creates the config if it doesn't exist
    if !storage_holder.get_flag("config.yaml") {
        let default_config = T::default();

        storage_holder
            .set_field_yaml("config.yaml", &default_config)
            .unwrap();
    }

    storage_holder.get_field_yaml("config.yaml").unwrap()
}

//From ReSwitched's Discord Server, last 0x1000 bytes in .rodata contains build info just after "GNU\x00", and .rodata is located before .data so I'm using that as an offset (Thanks DCNick3 and Masa!)
pub unsafe fn has_same_build_info() -> bool {
    let data_adress = getRegionAddress(Region::Data) as usize;
    let scan = core::slice::from_raw_parts((data_adress - 0x1000) as *const u8, 0x1000);

    let gnu_end_pos = match scan.windows(4).position(|w| w == b"GNU\x00") {
        Some(pos) => pos + 4,
        None => return false,
    };

    let build_info = &scan[gnu_end_pos..gnu_end_pos + 20]; // In the decompilation BUILD INFO had 20 bytes

    build_info == BUILD_INFO.as_slice()
}
