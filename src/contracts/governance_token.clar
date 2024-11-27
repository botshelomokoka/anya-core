;; Anya Governance Token Contract
;; Implements SIP-010 trait for fungible tokens

(impl-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

(define-fungible-token anya-token)

(define-constant contract-owner tx-sender)
(define-constant err-owner-only (err u100))
(define-constant err-not-token-owner (err u101))
(define-constant err-insufficient-balance (err u102))

;; SIP-010 Required Functions
(define-read-only (get-name)
    (ok "Anya Governance Token"))

(define-read-only (get-symbol)
    (ok "ANYA"))

(define-read-only (get-decimals)
    (ok u6))

(define-read-only (get-balance (account principal))
    (ok (ft-get-balance anya-token account)))

(define-read-only (get-total-supply)
    (ok (ft-get-supply anya-token)))

(define-read-only (get-token-uri)
    (ok (some "https://anya.org/token-metadata.json")))

(define-public (transfer (amount uint) (sender principal) (recipient principal) (memo (optional (buff 34))))
    (begin
        (asserts! (is-eq tx-sender sender) err-not-token-owner)
        (match (ft-transfer? anya-token amount sender recipient)
            response (begin
                (print memo)
                (ok true))
            error (err error))))

;; Governance specific functions
(define-read-only (get-voting-power (account principal))
    (ok (ft-get-balance anya-token account)))

(define-read-only (get-voting-power-at (account principal) (block uint))
    ;; TODO: Implement historical balance lookup
    (ok (ft-get-balance anya-token account)))

(define-public (mint (recipient principal) (amount uint))
    (begin
        (asserts! (is-eq tx-sender contract-owner) err-owner-only)
        (ft-mint? anya-token amount recipient)))

(define-public (burn (amount uint))
    (begin
        (asserts! (>= (ft-get-balance anya-token tx-sender) amount) err-insufficient-balance)
        (ft-burn? anya-token amount tx-sender)))

;; Delegation functionality
(define-map delegated-power principal principal)

(define-public (delegate (delegatee principal))
    (begin
        (map-set delegated-power tx-sender delegatee)
        (ok true)))

(define-read-only (get-delegatee (account principal))
    (ok (map-get? delegated-power account)))
