#![allow(unused_imports, dead_code, unused_variables)]
use std::{io::{Cursor, Read, Seek}, fs::DirBuilder};

use byteorder::{ReadBytesExt, LE};
use glam::*;

mod common;
use crate::common::{read_float3, read_quat, read_half_quat, read_half3};

#[derive(Debug, Clone, Copy)]
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

fn main_paj_rgis() {
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
    println!("{:x}", file.position());
    let _seperate_storage = file.read_u32::<LE>().unwrap();
    let _base_size = file.read_u32::<LE>().unwrap();
    let anim: Vec<()> = Vec::new();
    for _ in 0..anim_count {
        let name = read_string(&mut file);
        let anim_root_name = read_string(&mut file);
        let _bone_count = file.read_u16::<LE>().unwrap();
        for _ in 0.._bone_count {
            file.read_u32::<LE>().unwrap();
        }
        println!("{:x}", file.position());
        std::process::exit(0);
    }
}

fn main_rgis_sub() {
    let content = std::fs::read("00000097.dat").unwrap();
    let mut file = std::io::Cursor::new(content);
    let name = read_string(&mut file);
    let root_name = read_string(&mut file);
    let bone_count = file.read_u16::<LE>().unwrap();
    let mut bone_names: Vec<String> = Vec::new();
    for _ in 0..bone_count {
        bone_names.push(read_string(&mut file));
    }
    dbg!(bone_names);
    let fps = file.read_u32::<LE>().unwrap();
    let is_loop = file.read_u8().unwrap() != 0;
    let has_scaled = file.read_u8().unwrap() != 0;
    let prs_flags = file.read_u16::<LE>().unwrap();
    let accum_flags = file.read_u32::<LE>().unwrap();
    let pack_prs_flags = file.read_u8().unwrap();
    let bone_separate_flags = file.read_u8().unwrap();
    let key_count = file.read_u16::<LE>().unwrap();
    dbg!(key_count);
    for _ in 0..key_count {
        let key_time = file.read_f32::<LE>().unwrap();
    }
    for i in 0..bone_count {
        let has_pos_keys = file.read_u8().unwrap();
        let has_rot_keys = file.read_u8().unwrap();
        let has_scale_keys = file.read_u8().unwrap();
        let euler_flags = file.read_u8().unwrap();
        println!("{}: {:x} {} {} {} {}", i, file.position(), has_pos_keys, has_rot_keys, has_scale_keys, euler_flags);
        assert_eq!(euler_flags, 0);
        if has_pos_keys != 0{
            for _ in 0..key_count {
                let trans = read_float3(&mut file);
            }
        } else {
            read_float3(&mut file);
        }
        if has_rot_keys != 0 {
            for _ in 0..key_count {
                let quat = read_half_quat(&mut file);
                println!("{:?}", quat);
            }
        } else {
            let quat = read_half_quat(&mut file);
        }
        if has_scale_keys != 0 {
            for _ in 0..key_count {
                read_half3(&mut file);
            }
        } else {
            read_half3(&mut file);
        }
    }

}

fn main_mj_rgis() {
    let content = std::fs::read("008_res8-006228-02B74CE4-0000013E.rgis").unwrap();
    let mut file = std::io::Cursor::new(content);
    file.seek(std::io::SeekFrom::Start(4)).unwrap();
    let _version = file.read_u32::<LE>().unwrap();
    let anim_count = file.read_u16::<LE>().unwrap();
    dbg!(anim_count);
    let bone_count = file.read_u32::<LE>().unwrap();
    let mut bone_names: Vec<String> = Vec::new();
    for _ in 0..bone_count {
        bone_names.push(read_string(&mut file));
    }
    dbg!(bone_names);
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
    let _seperate_storage = file.read_u16::<LE>().unwrap();
    let _base_size = file.read_u16::<LE>().unwrap();
    if true {
        let _name = read_string(&mut file);
        let _anim_root_name = read_string(&mut file);
        let bone_count = file.read_u16::<LE>().unwrap();
        for _ in 0..bone_count {
            read_string(&mut file);
        }
        let _sample_fps = file.read_u32::<LE>().unwrap();
        let _is_loop = file.read_u8().unwrap() != 0;
        let _has_scaled = file.read_u8().unwrap() != 0;
        let _prs_flags = file.read_u16::<LE>().unwrap();
        let _accum_flags = file.read_u32::<LE>().unwrap();
        let _pack_prs_flags = file.read_u8().unwrap();
        let _bone_separate_flags = file.read_u8().unwrap();
        let key_count = file.read_u16::<LE>().unwrap();
        dbg!(key_count);
        for _ in 0..key_count {
            let key_time = file.read_f32::<LE>().unwrap();
            dbg!(key_time);
        }
        println!("{:x}", file.position());
        for _ in 0..key_count {
            let mut content = [0u8; 16];
            file.read(&mut content).unwrap();
            println!("{:?}", content);
        }
        println!("{:x}", file.position());
    }
    file.seek(std::io::SeekFrom::Start(0x427)).unwrap();
    print!("------------");
    if false {
        let _name = read_string(&mut file);
        let _anim_root_name = read_string(&mut file);
        let bone_count = file.read_u16::<LE>().unwrap();
        for _ in 0..bone_count {
            read_string(&mut file);
        }
        let _sample_fps = file.read_u32::<LE>().unwrap();
        let _is_loop = file.read_u8().unwrap() != 0;
        let _has_scaled = file.read_u8().unwrap() != 0;
        let _prs_flags = file.read_u16::<LE>().unwrap();
        let _accum_flags = file.read_u32::<LE>().unwrap();
        let _pack_prs_flags = file.read_u8().unwrap();
        let _bone_separate_flags = file.read_u8().unwrap();
        let key_count = file.read_u16::<LE>().unwrap();
        dbg!(key_count);
        println!("{:x}", file.position());
        for _ in 0..key_count {
            let key_time = file.read_f32::<LE>().unwrap();
            dbg!(key_time);
        }
        for _ in 0..key_count {
            let mut content = [0u8; 16];
            file.read(&mut content).unwrap();
            println!("{:?}", content);
        }
        println!("{:x}", file.position());
    }
}

fn main() {
    main_rgis_sub();
}