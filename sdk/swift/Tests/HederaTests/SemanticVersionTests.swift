/*
 * ‌
 * Hedera Swift SDK
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

import XCTest

@testable import Hedera

internal final class SemanticVersionDescriptionTests: XCTestCase {
    internal func testBasic() {
        XCTAssertEqual(SemanticVersion(major: 1, minor: 2, patch: 3).description, "1.2.3")
    }

    internal func testWithPrerelease() {
        XCTAssertEqual(SemanticVersion(major: 3, minor: 1, patch: 4, prerelease: "15.92").description, "3.1.4-15.92")
    }

    internal func testWithBuild() {
        XCTAssertEqual(SemanticVersion(major: 1, minor: 41, patch: 1, build: "6535asd").description, "1.41.1+6535asd")
    }

    internal func testWithPrereleaseAndBuild() {
        XCTAssertEqual(
            SemanticVersion(major: 0, minor: 1, patch: 4, prerelease: "0.9a2", build: "sha.25531c").description,
            "0.1.4-0.9a2+sha.25531c")
    }
}
