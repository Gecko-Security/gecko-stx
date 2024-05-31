;; vulnerable bank 

(define-constant err-insufficient-balance (err u1))
(define-constant err-unauthorized (err u2))
(define-map accounts { holder: principal } { amount: int })
(define-fungible-token vulnerable-token u100000)
(define-constant err-not-token-owner (err u4001))

;; bug0: misuse of `tx-sender` during authentication
(define-public (transfer (amount uint) (sender principal) (recipient principal))
  (begin
    (asserts! (is-eq tx-sender sender) err-not-token-owner)
    (try! (ft-transfer? vulnerable-token amount sender recipient))
    (ok true)))

;; bug1: untrusted input `amount` used to change state
(define-public (deposit (amount uint))
    (let ((balance (default-to 0 (get amount (map-get? accounts {holder: tx-sender})))))
        (map-set accounts {holder: tx-sender} {amount: (+ balance (to-int amount))})
        (stx-transfer? amount tx-sender (as-contract tx-sender))
    )
)

;; bug2: untrusted input `amount` used to change state
(define-public (withdrawal-unsafe (amount uint)) 
    (let (
          (balance (default-to 0 (get amount (map-get? accounts {holder: tx-sender}))))
          (customer tx-sender)
         )
        (map-set accounts {holder: tx-sender} {amount: (- balance (to-int amount))})
        (as-contract (stx-transfer? amount tx-sender customer))
    )
)

(define-public (withdrawal (amount uint))
    (let (
          (balance (default-to 0 (get amount (map-get? accounts {holder: tx-sender}))))
          (customer tx-sender)
         )
        (asserts! (>= balance (to-int amount)) err-insufficient-balance)
        (map-set accounts {holder: tx-sender} {amount: (- balance (to-int amount))})
        (as-contract (stx-transfer? amount tx-sender customer))
    )
)

;; Check that the amount is less than or equal to the balance.
(define-private (check-balance (amount uint))
    (let ((balance (default-to 0 (get amount (map-get? accounts {holder: tx-sender})))))
        (asserts! (<= (to-int amount) balance) err-insufficient-balance)
        (ok true)
    )
)

;;bug3: untrusted input `amount`
(define-public (withdrawal-callee-filter (amount uint))
    (let (
          (balance (default-to 0 (get amount (map-get? accounts {holder: tx-sender}))))
          (customer tx-sender)
         )
        (try! (check-balance amount))
        (map-set accounts {holder: tx-sender} {amount: (- balance (to-int amount))})
        (as-contract (stx-transfer? amount tx-sender customer))
    )
)

(define-read-only (get-balance)
    (default-to 0 (get amount (map-get? accounts {holder: tx-sender})))
)

(define-data-var bank-owner principal tx-sender)

;; bug4: owner can take money
(define-public (take (amount uint) (from principal))
    (let (
          (balance (- (default-to 0 (get amount (map-get? accounts {holder: from}))) (to-int amount)))
          (banker tx-sender)
        )
        (asserts! (is-eq tx-sender (var-get bank-owner)) err-unauthorized)
        (map-set accounts {holder: from} {amount: balance})
        (as-contract (stx-transfer? amount tx-sender banker))
    )
)

(define-data-var expiration-height uint u100)

;; bug5: after a certian block time any user can take money
(define-public (take-after-time (amount uint) (from principal))
    (let ((balance (- (default-to 0 (get amount (map-get? accounts {holder: from}))) (to-int amount))))
        (asserts! (>= balance (to-int amount)) err-insufficient-balance)
        (asserts! (> block-height (var-get expiration-height)) err-unauthorized)
        (map-set accounts {holder: from} {amount: balance})
        (stx-transfer? amount (as-contract tx-sender) tx-sender)
    )
)