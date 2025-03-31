# Engine
TODO List
- Resolve Branching Paths
  - Changing ruleset is bad because rules should be const
  - Adding "Switch" Keys that are either 0.0 or 1.0 for turning on or off reactions.
    - Expensive, would still require the other value to be computed
  - Manually setting "blocking" result keys to 0.0 to stop calculation, except for the correct branch, before calculation begins
    - Fragile, setting down-branch keys will by default result in the deletion of the "blocker" keys
  - Add Branch Dictionary that maps keys to keys, integrate into Rules and .get()
    - Branch Templating?
      - Writing the same Elemental Switch over and over and maintaining it would be annoying.
      - Rule Forwarding? Like, it creates the tag @ run-time based on a rule?

  - Currently the implementation for this is with mux nodes and converting types into f32 "indexes" for matching.
    - To be blunt, this implementation frustrates me because of using f32 for indexing, but it's the best I can think of right now.
