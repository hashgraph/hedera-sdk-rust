/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

import Foundation

// sadly the GRPC `ConnectionBackoff` doesn't support `maxElapsedTime`.
// new MacOS versions support `Clock` (protocol) which obsoletes this method
// adapted from the Rust [`backoff`](https://github.com/ihrwein/backoff) crate, which is licensed under the MIT/Apache 2.0 licenses.
internal struct LegacyExponentialBackoff {
    internal static let defaultMaxElapsedTime: TimeInterval = 900
    internal init(
        initialInterval: TimeInterval = 0.5,
        randomizationFactor: Double = 0.5,
        multiplier: Double = 1.5,
        maxInterval: TimeInterval = 60,
        maxElapsedTime: Limit<TimeInterval> = .limited(defaultMaxElapsedTime),
        startTime: Date = Date()
    ) {
        self.currentInterval = initialInterval
        self.initialInterval = initialInterval
        self.randomizationFactor = randomizationFactor
        self.multiplier = multiplier
        self.maxInterval = maxInterval
        self.startTime = startTime
        self.maxElapsedTime = maxElapsedTime
    }

    internal enum Limit<T> {
        case unlimited
        case limited(T)

        internal static func optional(_ value: T?) -> Self {
            switch value {
            case .some(let value):
                return .limited(value)
            case .none:
                return .unlimited
            }
        }
    }

    internal var currentInterval: TimeInterval
    internal let initialInterval: TimeInterval
    internal let randomizationFactor: Double
    internal let multiplier: Double
    internal let maxInterval: TimeInterval
    internal var startTime: Date
    internal let maxElapsedTime: Limit<TimeInterval>

    /// The amount of time that has elapsed since ``startTime``.
    ///
    /// This will return a different value every time it's called.
    internal var elapsedTime: TimeInterval {
        startTime.distance(to: Date())
    }

    internal var randomRange: Range<Double> {
        (1 - randomizationFactor)..<(1 + randomizationFactor)
    }

    internal mutating func reset() {
        startTime = Date()
        currentInterval = initialInterval
    }

    private static func randomValueFromInterval(
        randomizationFactor: Double,
        randomValue: Double,
        currentInterval: TimeInterval
    ) -> TimeInterval {
        let delta = randomizationFactor * currentInterval
        let minInterval = currentInterval - delta
        let maxInterval = currentInterval + delta

        // Get a random value from the range [minInterval, maxInterval].
        // The formula used below has a +1 nano because if the `minInterval` is 1 and the `maxInterval` is 3 then
        // we want a 33% chance for selecting either 1, 2 or 3.
        let diff = maxInterval - minInterval
        return minInterval + (randomValue * (diff + 1e-9))

    }

    private mutating func incrementCurrentInterval() {
        if currentInterval > maxInterval / multiplier {
            currentInterval = maxInterval
        } else {
            currentInterval *= multiplier
        }
    }

    internal mutating func next() -> TimeInterval? {
        let elapsedTime = self.elapsedTime

        if maxElapsedTime.expired(elapsedTime) {
            return nil
        }

        let randomValue = Double.random(in: 0..<1)

        let outputInterval = Self.randomValueFromInterval(
            randomizationFactor: randomizationFactor,
            randomValue: randomValue,
            currentInterval: currentInterval
        )

        self.incrementCurrentInterval()

        // if we'll expire before the `output` time elapses then obviously we expire.
        if maxElapsedTime.expired(elapsedTime + outputInterval) {
            return nil
        }

        return outputInterval
    }
}

extension LegacyExponentialBackoff.Limit {
    internal func expired(_ elapsed: T) -> Bool where T: Comparable {
        // `.unlimited` never expires.
        guard case .limited(let maxElapsed) = self else {
            return false
        }

        return elapsed > maxElapsed
    }
}
