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

use hedera::PrivateKey;

fn main() {
    // Generate a Ed25519 key
    // This is the current recommended default for Hedera

    let private = PrivateKey::generate_ed25519();
    let public = private.public_key();

    println!("ed25519 private = {private}");
    println!("ed25519 public = {public}");

    // Generate a ECDSA(secp256k1) key
    // This is recommended for better compatibility with Ethereum

    let private = PrivateKey::generate_ecdsa_secp256k1();
    let public = private.public_key();

    println!("ecdsa(secp256k1) private = {private}");
    println!("ecdsa(secp256k1) public = {public}");
}
