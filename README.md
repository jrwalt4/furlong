# furlong

## Overview

furlong is a units library with static dimensional analysis. The API attempts to mirror the [boost/units](http://boost.org/libs/units), but until specialization lands there are limitations to the type system (see [conversions](#conversions))

## Units

Every unit consists of a set of `BaseDimension`s and `BaseUnit`s. The base dimensions are:
- Mass
- Length
- Time
- Current (not currently implemented)
- Light (not currently implemented)
- Temperature (not currently implemented)
- Amount (not currently implemented)

## Quantities

Actual values are stored as `Qnty` that have a value and a unit. `Qnty`s can be added/subtracted to other `Qnty`s with the same dimension and can be multiplied/divided by any other `Qnty` resulting in a new `Qnty` with dimenions that is the sum of each `Qnty`'s dimensions. 

## Conversions

Because generic specializations are not stable in Rust, `BaseUnit`s cannot define individual conversation between eachother, but must define a conversion to a "universal base" for its dimension (`BaseUnit::MULTIPLIER`). This way, every conversion take 2 steps: multiply by the source unit's `BaseUnit::MULTIPLIER` to convert to the "universal base", then divide by the target unit's `BaseUnit::MULTIPLIER` to convert from "universal base" to the target unit. The "universal base"s are the metric/si units:

- Mass = grams
- Length = meters
- Time = seconds
- Current = ampere (not currently implemented)
- Light = candela (not currently implemented)
- Temperature = kelvin (not currently implemented)
- Amount = mole (not currently implemented)
