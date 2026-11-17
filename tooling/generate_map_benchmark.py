#!/usr/bin/env python3
"""Render public map workload adapters and calculate independent result fingerprints."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORKLOADS = ("construct-overwrite-query-delete-walk", "complete-traversal", "immediate-stop")


def source(variant: str) -> str:
    """Preserve the same workload while selecting only the public storage operations."""
    if variant == "stable-map":
        module = "Data.Mutable.Map"
        adapter = """  -- | Name the tested façade without changing its captured key policy.
  data Store r where
    -- | Retain the complete public map owner.
    StoreValue :: MutableMap r Nat Nat -> Store r

  -- | Construct an empty public map with the normal natural-key policy.
  store-new :: forall r. Unit ->{Mut r} Store r
  store-new unit = StoreValue (mutable-map-new ())

  -- | Preserve checked insertion refusal in the workload result.
  store-set :: forall r. Store r -> Nat -> Nat ->{Mut r} Bool
  store-set map key value = case map of StoreValue actual -> case mutable-map-set actual key value of Ok unit -> True; Err error -> False

  -- | Delete an existing binding through the checked public operation.
  store-delete :: forall r. Store r -> Nat ->{Mut r} Bool
  store-delete map key = case map of StoreValue actual -> case mutable-map-delete actual key of
    Ok value -> case value of Some previous -> True; None -> False
    Err error -> False

  -- | Read one binding without materializing a snapshot.
  store-lookup :: forall r. Store r -> Nat ->{Mut r} Option Nat
  store-lookup map key = case map of StoreValue actual -> mutable-map-lookup actual key

  -- | Observe the current binding count.
  store-size :: forall r. Store r ->{Mut r} Nat
  store-size map = case map of StoreValue actual -> mutable-map-size actual

  -- | Traverse public stable identities with the callback's termination result.
  store-all :: forall r. Store r -> (Nat -> Nat ->{Mut r} Bool) ->{Mut r} Bool
  store-all map yield = case map of StoreValue actual -> mutable-map-all actual yield"""
    elif variant == "hash-table":
        module = "Data.Mutable.HashTable"
        adapter = """  -- | Retain the existing public hash table as a baseline owner.
  data Store r where
    -- | Own the complete existing hash table.
    StoreValue :: HashTable r Nat Nat -> Store r

  -- | Construct the baseline through its public empty constructor.
  store-new :: forall r. Unit ->{Mut r} Store r
  store-new unit = StoreValue (hash-table-new ())

  -- | Insert bounded inputs through the baseline's public update operation.
  store-set :: forall r. Store r -> Nat -> Nat ->{Mut r} Bool
  store-set map key value = case map of StoreValue actual -> let saved = hash-table-insert actual key value in True

  -- | Require the deleted existing binding to be returned.
  store-delete :: forall r. Store r -> Nat ->{Mut r} Bool
  store-delete map key = case map of StoreValue actual -> case hash-table-delete actual key of Some previous -> True; None -> False

  -- | Read one baseline binding through public lookup.
  store-lookup :: forall r. Store r -> Nat ->{Mut r} Option Nat
  store-lookup map key = case map of StoreValue actual -> hash-table-lookup actual key

  -- | Read the baseline's public count.
  store-size :: forall r. Store r ->{Mut r} Nat
  store-size map = case map of StoreValue actual -> hash-table-size actual

  -- | Stop at the first False without observing a container's private storage.
  visit :: forall r. List (Nat, Nat) -> (Nat -> Nat ->{Mut r} Bool) ->{Mut r} Bool
  visit values yield = case values of Nil -> True; Cons pair rest -> case pair of (key,value) -> if yield key value then visit rest yield else False

  -- | Include the public value-snapshot cost required for a stopping baseline traversal.
  store-all :: forall r. Store r -> (Nat -> Nat ->{Mut r} Bool) ->{Mut r} Bool
  store-all map yield = case map of StoreValue actual -> visit (hash-table-to-list actual) yield"""
    else:
        raise ValueError(variant)
    return (ROOT / "tooling/map-benchmark.cinch.in").read_text().replace("@IMPORT@", "import " + module).replace("@ADAPTER@", adapter)


def expected(mode: int, size: int, iterations: int) -> int:
    """Compute the complete result from independent integer bindings and callback counts."""
    def contribution(key: int, value: int) -> int:
        """Use the workload's explicitly declared commutative observation."""
        return key * 131 + value

    checksum = 0
    prepared = {key: key * 3 + 7 for key in range(size)} if mode else {}
    for iteration in range(iterations):
        if mode == 0:
            bindings = {key: key * 5 + iteration + 11 for key in range(size)}
            queried = sum(contribution(key, value) for key, value in bindings.items())
            remaining = {key: value for key, value in bindings.items() if key % 2}
            checksum += queried + sum(contribution(key, value) for key, value in remaining.items()) + 2 * len(remaining)
        else:
            key = iteration % size if size else 0
            if size:
                prepared[key] = key * 3 + 7 + iteration
            total = sum(contribution(key, value) for key, value in prepared.items()) if mode == 1 else 0
            calls = len(prepared) if mode == 1 else min(1, len(prepared))
            checksum += total + calls + len(prepared) + prepared.get(key, 0)
    return checksum + len(prepared)
