use std::io::{Cursor, Read, Seek};

use byteorder::{ReadBytesExt, LE};
use glam::*;

mod common;
use crate::common::{read_float3, read_quat};

struct BoneTran {
    pos: Vec3,
    rot: Quat,
    scale: Vec3,
}
fn read_string(file: &mut Cursor<Vec<u8>>) -> String {
    let mut buf = [0u8; 32];
    file.read_exact(&mut buf).unwrap();
    let name = std::str::from_utf8(&buf).unwrap().to_owned();
    let name = name.replace('\0', "");
    name.replace(' ', "_")
}

fn main() {
    let content = std::fs::read("00000007.rgis").unwrap();
    let mut file = std::io::Cursor::new(content);
    file.seek(std::io::SeekFrom::Start(4)).unwrap();
    let _version = file.read_u32::<LE>().unwrap();
    let anim_count = file.read_u16::<LE>().unwrap();
    let bone_count = file.read_u32::<LE>().unwrap();
    let mut bone_names: Vec<String> = Vec::new();
    for _ in 0..bone_count {
        bone_names.push(read_string(&mut file));
    }
    let mut bone_trans = Vec::new();
    for _ in 0..bone_count {
        let pos = read_float3(&mut file);
        let rot = read_quat(&mut file);
        let scale = read_float3(&mut file);
        bone_trans.push(BoneTran {
            pos,
            rot,
            scale,
        });
    }
    dbg!(file.position());
    let _seperate_storage = file.read_u32::<LE>().unwrap();
    let _base_size = file.read_u32::<LE>().unwrap();
    let anim: Vec<()> = Vec::new();
    for _ in 0..anim_count {
        let name = read_string(&mut file);
        let anim_root_name = read_string(&mut file);
        let _bone_count = file.read_u16::<LE>().unwrap();

    }

}
