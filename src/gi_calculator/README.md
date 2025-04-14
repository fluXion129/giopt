An implementation of Calculator for Genshin.

# Structure

## GCK

Genshin Calculator Keys. Enum that contains all possible keys in a Genshin Calculator.

Divided into Branch and Leaf Key types.

## GI_RULES

Genshin Calculator Rules implementation.

There are occasionally multiple uses of a single key in Genshin. This is currently not supported by my Calculator implementation.
The workaround that I am currently using is that there are helper functions that ensure these related keys are set correctly.
This is not ideal because it leads to the possibility of incorrect usage, especially of the set function.
Perhaps I need to study directional graphs a bit? Realistically I can implement this just by changing parents contain Vecs but
I don't love that idea for some reason. Perhaps it almost seems too much. Anyways.
