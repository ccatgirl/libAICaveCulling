mod test_utils;

mod region_tests {
    use crate::test_utils::{write_world, write_world_dc};
    use lodestone_common::util::McVersion;
    use lodestone_common::util::McVersion::Classic0_0_12a;
    use lodestone_java::alpha::AlphaLevel;
    use lodestone_java::anvil::Anvil;
    use lodestone_java::classic::classic_world::CWLevel;
    use lodestone_java::classic::mcgalaxy_lvl::MCGLevel;
    use lodestone_java::classic::mine_v1::MineV1Level;
    use lodestone_java::classic::mine_v2::MineV2Level;
    use lodestone_java::indev::IndevLevel;
    use lodestone_java::mcregion::Region;
    use lodestone_level::level::Level;
    use std::fs;
    use std::fs::{create_dir_all, exists, remove_dir_all, File};
    use std::io::{Read, Write};
    use std::path::Path;
    use std::time::Instant;

    #[test]
    fn region_to_block_map() {
        log::set_max_level(log::LevelFilter::Debug);
        let level = read_mcregion_level();

        let bitmap_start = Instant::now();
        let map = level.generate_bitmap();

        println!("Writing");
        let mut of = File::create(format!(
            "../../internal_tests/map/{}-{}_{}.raw",
            "RegionTest",
            level.get_block_width(),
            level.get_block_length()
        ))
        .unwrap();
        of.write_all(&map).unwrap();
        of.flush().unwrap();

        let bitmap_elapsed = bitmap_start.elapsed();
        println!("Bitmap creation time {:?}", bitmap_elapsed);
    }

    #[test]
    fn region_to_mine_v1() {
        log::set_max_level(log::LevelFilter::Debug);
        let level = read_mcregion_level();

        println!("Writing Mine V1 world");
        let mv1_start = Instant::now();
        let mv1 = level.write_minev1(Classic0_0_12a);
        write_world(mv1, "RegionTest.mine", "minev1");
        let mv1_end = mv1_start.elapsed();
        println!("Mine V1: {:?}", mv1_end);
    }

    #[test]
    fn region_to_mine_v2() {
        log::set_max_level(log::LevelFilter::Debug);
        let mut level = read_mcregion_level();

        println!("Writing Mine V2 world");
        let mv2_start = Instant::now();

        let mv2: Vec<u8> = level.write_minev2(McVersion::Classic0_0_14a);
        write_world(mv2, "RegionTest.mine", "minev2");
        let mv2_end = mv2_start.elapsed();

        println!("Mine V2: {:?}", mv2_end);
    }

    #[test]
    fn region_to_classic_world() {
        log::set_max_level(log::LevelFilter::Debug);
        let mut level = read_mcregion_level();

        println!("Writing ClassicWorld world");
        let cw_start = Instant::now();

        let cw: Vec<u8> = level.write_cw(McVersion::Classic0_30);
        write_world_dc(cw, "RegionTest.cw", "cw");
        let cw_end = cw_start.elapsed();

        println!("ClassicWorld: {:?}", cw_end);
    }

    #[test]
    fn region_to_mcgalaxy() {
        log::set_max_level(log::LevelFilter::Debug);
        let level = read_mcregion_level();

        println!("Writing MCG world");
        let mcg_start = Instant::now();
        let mcg = level.write_mcgalaxy_level(McVersion::Classic0_30);
        write_world_dc(mcg, "RegionTest.lvl", "lvl");

        let mcg_end = mcg_start.elapsed();

        println!("MCGalaxy: {:?}", mcg_end);
    }

    #[test]
    fn region_to_indev() {
        log::set_max_level(log::LevelFilter::Debug);
        let mut level = read_mcregion_level();

        println!("Writing Indev world");
        let indev_start = Instant::now();
        let indev = level.write_indev(McVersion::Infdev20100630);
        write_world_dc(indev, "RegionTest.mclevel", "indev");

        let indev_end = indev_start.elapsed();

        println!("Indev: {:?}", indev_end);
    }

    #[test]
    fn region_to_alpha() {
        log::set_max_level(log::LevelFilter::Debug);
        let mut level = read_mcregion_level();

        // so we do not write into existing folder
        println!("Writing Alpha world");
        let alpha_dir = Path::new("../../internal_tests/alpha/dst/RegionTest/");
        if exists(alpha_dir).unwrap() {
            remove_dir_all(alpha_dir).expect("Failed to delete alpha dir");
        }

        create_dir_all(alpha_dir).expect("Failed to create alpha dir");

        let alpha_start = Instant::now();
        level.write_alpha_dir(McVersion::Alpha1_2_6, alpha_dir);
        let alpha_end = alpha_start.elapsed();

        println!("Alpha: {:?}", alpha_end);
    }

    #[test]
    fn region_to_anvil() {
        log::set_max_level(log::LevelFilter::Debug);
        let level = read_mcregion_level();

        println!("Writing Anvil world");
        let anvil_dir = Path::new("../../internal_tests/anvil/dst/RegionTest/");
        if exists(anvil_dir).unwrap() {
            remove_dir_all(anvil_dir).expect("Failed to delete anvil dir");
        }

        create_dir_all(anvil_dir).expect("Failed to create anvil dir");
        let anvil_start = Instant::now();
        level.write_anvil_dir(anvil_dir);
        let anvil_end = anvil_start.elapsed();
        println!("Anvil: {:?}", anvil_end);
    }

    fn read_mcregion_level() -> Level {
        log::set_max_level(log::LevelFilter::Debug);
        let mut level = Level::new();

        println!("Reading level");
        let path = "../../internal_tests/regions/src/NewWorld/";
        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let file = File::open(entry.path());
                    match file {
                        Ok(mut file) => {
                            let mut buffer = Vec::new();
                            let content = file.read_to_end(&mut buffer);
                            match content {
                                Ok(_sz) => {
                                    level.read_mcr_into_existing(McVersion::Release1_1, buffer);
                                }
                                Err(e) => {
                                    println!("  read error: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  open error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  entry error: {:?}", e);
                }
            }
        }

        level
    }
}
