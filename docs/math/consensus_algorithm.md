# Consensus Algorithm

## Overview

This document describes the mathematical foundation of our consensus algorithm.

## Definitions

Let P be the set of participants in the network.
Let B be the set of all possible blocks.
Let V: B → ℝ be a function that assigns a value to each block.

## Algorithm

1. Each participant p ∈ P proposes a block b ∈ B.
2. The network selects the block   b*such that:
   b* = argmax_{b ∈ B} V(b)

## Proof of Correctness

Theorem: The selected block b* maximizes the value function V.

Proof:
By construction, b*is chosen such that V(b*) ≥ V(b) for all b ∈ B.
Therefore, b* maximizes the value function V.

## Complexity Analysis

Time Complexity:  O(|P| * |B|)
Space Complexity: O(|B|)
