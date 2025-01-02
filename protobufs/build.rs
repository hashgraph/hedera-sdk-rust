/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let temp_dir = out_dir.join("proto_temp");
    let google_dir = temp_dir.join("google/protobuf");

    // Clean and create directories
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&google_dir)?;

    let cwd = env::current_dir()?;
    let proto_root = cwd.join("protobufs");
    let services_dir = proto_root.join("services");
    let platform_dir = proto_root.join("platform");

    // Copy service and platform protos to temp directory
    if services_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
        copy_proto_files(&services_dir, &temp_dir)?;
    }
    if platform_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
        copy_proto_files(&platform_dir, &temp_dir)?;
    }

    println!("cargo:rerun-if-changed={}", services_dir.display());
    println!("cargo:rerun-if-changed={}", platform_dir.display());

    let mut protos = Vec::new();
    collect_proto_files(&temp_dir, &mut protos)?;

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config.out_dir(&temp_dir);
    config.include_file("mod.rs");

    let google_parent = google_dir.parent().unwrap().to_path_buf();
    config.compile_protos(&protos, &[&temp_dir, &google_parent])?;

    Ok(())
}

fn copy_proto_files(src_dir: &Path, dst_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
            let content = fs::read_to_string(&path)?;
            let patched_content = patch_proto_content(&content)?;
            let dst_path = dst_dir.join(path.file_name().unwrap());
            fs::write(dst_path, patched_content)?;
        } else if path.is_dir() {
            let dst_subdir = dst_dir.join(path.file_name().unwrap());
            fs::create_dir_all(&dst_subdir)?;
            copy_proto_files(&path, &dst_subdir)?;
        }
    }
    Ok(())
}

fn collect_proto_files(dir: &Path, protos: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
            protos.push(path);
        } else if path.is_dir() {
            collect_proto_files(&path, protos)?;
        }
    }
    Ok(())
}

fn patch_proto_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    
    // Find all import lines
    let mut import_lines = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("import") {
            import_lines.push((i, line.clone()));
        }
    }

    // Update imports to use correct paths
    for (i, line) in import_lines {
        if line.contains("\"google/protobuf/") {
            lines[i] = line.replace("\"google/protobuf/", "\"google/protobuf/");
        } else {
            lines[i] = line.replace("\"", "\"");
        }
    }

    Ok(lines.join("\n"))
}
