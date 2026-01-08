//! 实现获取玩家指定 shard 的全部资源

use std::{collections::HashMap, error::Error};

use screeps_rust_api::{RoomObject, ScreepsApi, ScreepsError, ScreepsResult, screeps_api_from_env};

#[tokio::main]
async fn main() -> ScreepsResult<()> {
    let api = screeps_api_from_env!().unwrap();
    let res = query_res(&api, "6g3y", "all").await;
    match res {
        Ok(res) => {
            print_resources(&res);
        }
        Err(e) => {
            if let ScreepsError::Http(e) = e {
                println!("{} {:?}", e, e.source())
            }
        }
    }

    Ok(())
}

/// 查询玩家指定shard具有的资源
/// 参数：
/// - username: 玩家名称
/// - target_shard: 目标 shard，传 `all` 表示所有 shard
async fn query_res(
    api: &ScreepsApi,
    username: &str,
    target_shard: &str,
) -> ScreepsResult<HashMap<String, HashMap<String, i32>>> {
    let mut result = HashMap::new();

    // 先根据玩家信息查玩家的 id
    let user_info = api.get_user_info_by_name(username).await?;
    if user_info.base_data.ok.unwrap() != 1 {
        return Err(ScreepsError::Api("玩家不存在".to_string()));
    }

    let user_id = user_info.user.unwrap()._id;
    // 再根据玩家 id 查玩家所有房间
    let user_rooms = api.get_user_rooms(&user_id).await?;
    if user_rooms.base_data.ok.unwrap() != 1 {
        return Err(ScreepsError::Api("玩家没有房间".to_string()));
    }

    // 收集所有需要查询的房间和 shard 信息
    let mut room_shard_pairs = Vec::new();
    for (shard, rooms) in user_rooms.shards.unwrap().iter() {
        if target_shard != "all" && shard != target_shard {
            continue;
        }
        println!("开始处理 shard: {}", shard);
        for room in rooms {
            room_shard_pairs.push((room.clone(), shard.clone()));
        }
    }

    // 创建所有 future
    let futures: Vec<_> = room_shard_pairs
        .iter()
        .map(|(room, shard)| api.get_room_objects(room, shard))
        .collect();

    // 执行所有请求
    let responses = futures::future::join_all(futures).await;
    // 处理响应
    for (response, (room, shard)) in responses.into_iter().zip(room_shard_pairs.iter()) {
        match response {
            Ok(room_objects) => {
                if room_objects.base_data.ok.unwrap() != 1 {
                    eprintln!(
                        "Failed to fetch objects for room {} in shard {}, reason: {}",
                        room,
                        shard,
                        room_objects.base_data.error.unwrap()
                    );
                    continue;
                }
                let shard_res_map = result.entry(shard.clone()).or_insert_with(HashMap::new);
                for room_object in room_objects.objects.unwrap() {
                    match room_object {
                        RoomObject::Storage(storage) => {
                            for (resource_type, amount) in storage.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Terminal(terminal) => {
                            for (resource_type, amount) in terminal.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Factory(link) => {
                            for (resource_type, amount) in link.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to fetch objects for room {} in shard {}: {}",
                    room, shard, e
                );
                return Err(e);
            }
        }
    }

    Ok(result)
}

fn format_number(num: i32) -> String {
    if num >= 1_000_000 {
        format!("{:.2}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.2}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}

fn print_resources(resources: &HashMap<String, HashMap<String, i32>>) {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    玩家资源统计");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut shards: Vec<_> = resources.keys().collect();
    shards.sort();

    for shard in shards {
        let shard_res = &resources[shard];
        println!("📍 Shard: {}", shard);
        println!("┌─────────────────────────────────────────────────────────────┐");

        let categories: Vec<(&str, Vec<String>)> = vec![
            ("基础资源", vec!["energy", "power", "ops"].iter().map(|s| s.to_string()).collect()),
            ("基础矿物", vec!["H", "O", "L", "K", "Z", "U", "X", "G"].iter().map(|s| s.to_string()).collect()),
            ("基础化合物", vec!["OH", "ZK", "UL", "GHO2", "UH2O", "KH2O", "UHO2", "LHO2", "KHO2", "XUH2O", "XHO2", "XKH2O", "XZHO2", "XGHO2", "XLH2O", "XLHO2", "XGH2O", "XZH2O", "KH", "ZH", "UH", "LH", "GH", "ZO", "KO", "UO", "LO", "GO"].iter().map(|s| s.to_string()).collect()),
            ("压缩资源", vec!["utrium_bar", "lemergium_bar", "keanium_bar", "zynthium_bar", "ghodium_melt", "oxidant", "reductant", "purifier", "battery"].iter().map(|s| s.to_string()).collect()),
            ("高级资源", vec!["composite", "crystal", "liquid", "wire", "switch", "transistor", "microchip", "circuit", "device", "fixture", "frame", "hydraulics", "machine", "organism", "organoid", "tissue", "muscle", "essence", "spirit", "phlegm", "mist", "biomass", "metal", "silicon", "alloy", "tube", "cell", "fiber", "wire", "condensate", "concentrate", "extract", "emanation"].iter().map(|s| s.to_string()).collect()),
        ];

        for (category_name, resource_types) in &categories {
            let mut has_resources = false;
            for res_type in resource_types {
                if let Some(amount) = shard_res.get(res_type) {
                    if *amount > 0 {
                        has_resources = true;
                        break;
                    }
                }
            }

            if has_resources {
                println!("│  {}", category_name);
                println!("├─────────────────────────────────────────────────────────────┤");

                let mut res_list: Vec<_> = resource_types.iter()
                    .filter_map(|res_type| {
                        shard_res.get(res_type).map(|amount| (res_type.as_str(), *amount))
                    })
                    .filter(|(_, amount)| *amount > 0)
                    .collect();

                res_list.sort_by(|a, b| b.1.cmp(&a.1));

                for (res_type, amount) in res_list {
                    let formatted_num = format_number(amount);
                    println!("│  {:<12} {:>15}", res_type, formatted_num);
                }
                println!("├─────────────────────────────────────────────────────────────┤");
            }
        }

        let total_energy = shard_res.get("energy").unwrap_or(&0);
        let total_power = shard_res.get("power").unwrap_or(&0);
        println!("│  汇总:");
        println!("│  Energy: {:>15}  Power: {:>15}", format_number(*total_energy), format_number(*total_power));
        println!("└─────────────────────────────────────────────────────────────┐");
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════\n");
}
