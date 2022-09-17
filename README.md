# xpct

*Pronounced "expect" or however else you want to pronounce it.*

xpct is an extensible assertion library for Rust. It's designed to be
batteries-included and test framework agnostic.

xpct is highly extensible. In addition to allowing you to write custom
matchers, it separates the logic of matchers from how they format their output,
meaning you can:

1. Hook into existing formatters to write custom matchers with pretty output
   without having to worry about formatting.
2. Customize the formatting of existing matchers without having to reimplement
   their logic.