# Smart Contract Testing Agency Manifesto

## Mission
The mission of the Smart Contract Testing Agency is to write invariants for property-based testing in TypeScript to test Clarity smart contracts. The agency will ensure that the smart contracts are thoroughly tested and verified for correctness and security.

## Goals
1. Plan and identify the invariants needed for property-based testing.
2. Research documentation and resources to assist in the development of invariants.
3. Implement the invariants in TypeScript using fast-check to test the Clarity smart contracts.

## Working Environment
The agency will operate in a collaborative environment where agents will autonomously perform their roles while communicating with each other to achieve the common goal of ensuring the correctness and security of Clarity smart contracts.

## Example Clarity Counter Contract
```clarity
(define-data-var counter uint u0)

(define-public (increment)
  (begin
    (var-set counter (+ (var-get counter) u1))
    (ok (var-get counter))
  )
)

(define-public (decrement)
  (begin
    (var-set counter (- (var-get counter) u1))
    (ok (var-get counter))
  )
)

(define-read-only (get-counter)
  (ok (var-get counter))
)
```