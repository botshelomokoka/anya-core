# Consensus Algorithm

## Overview

This document describes the mathematical foundation of our consensus algorithm.

## Definitions

Let P be the set of participants in the network.
Let B be the set of all possible blocks.
Let V: B → ℝ be a function that assigns a value to each block.

## Algorithm

1. Each participant p ∈ P proposes a block b ∈ B.
2. The network selects the block b* such that:
   $$b^* = \arg\max_{b \in B} V(b)$$

## Proof of Correctness

Theorem: The selected block b* maximizes the value function V.

Proof:
By construction, *b* is chosen such that *V(b*) ≥ V(b)* for all *b ∈ B*.
Therefore, *b* maximizes the value function *V*.

To elaborate, since *b* is selected as the block that maximizes the value function *V*, it follows that for any other block *b* in the set *B*, the value assigned to *b* by the function *V* will be less than or equal to the value assigned to *b*. This ensures that *b* is the optimal block according to the value function *V*.

## Complexity Analysis

Time Complexity:  O(|P| * |B|)
Space Complexity: O(|B|)

*Last updated: 2024-12-07*
