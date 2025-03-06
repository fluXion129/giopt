# Engine
TODO List
- Resolve Branching Paths
  - Changing ruleset is bad because rules should be const
  - Adding "Switch" Keys that are either 0.0 or 1.0 for turning on or off reactions.
    - Expensive, would still require the other value to be computed
  - Manually setting "blocking" result keys to 0.0 to stop calculation, except for the correct branch, before calculation begins
    - Fragile, setting down-branch keys will by default result in the deletion of the "blocker" keys
