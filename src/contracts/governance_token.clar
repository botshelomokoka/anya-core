;; Anya Governance Token Contract
;; Implements SIP-010 trait for fungible tokens with Bitcoin-inspired supply

(impl-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Token Configuration
(define-fungible-token anya-token u21000000) ;; Exact Bitcoin supply

;; Constants
(define-constant contract-owner tx-sender)
(define-constant MAX_SUPPLY u21000000)
(define-constant INITIAL_BLOCK_REWARD u50)
(define-constant HALVING_INTERVAL u210000)

;; Error Codes
(define-constant ERR-OWNER-ONLY (err u100))
(define-constant ERR-NOT-TOKEN-OWNER (err u101))
(define-constant ERR-INSUFFICIENT-BALANCE (err u102))
(define-constant ERR-SUPPLY-LIMIT (err u103))

;; SIP-010 Required Functions
(define-read-only (get-name)
    (ok "Anya Governance Token"))

(define-read-only (get-symbol)
    (ok "AGT"))

(define-read-only (get-decimals)
    (ok u6))

(define-read-only (get-balance (account principal))
    (ok (ft-get-balance anya-token account)))

(define-read-only (get-total-supply)
    (ok (ft-get-supply anya-token)))

(define-read-only (get-token-uri)
    (ok (some "https://anya.ai/token/agt-metadata.json")))

;; Governance Token Specific Functions
(define-read-only (get-voting-power (account principal))
    (ok (ft-get-balance anya-token account)))

;; Token Transfer with Governance Considerations
(define-public (transfer (amount uint) (sender principal) (recipient principal) (memo (optional (buff 34))))
    (begin
        (asserts! (is-eq tx-sender sender) ERR-NOT-TOKEN-OWNER)
        (match (ft-transfer? anya-token amount sender recipient)
            response (begin
                (print memo)
                (ok true))
            error (err error))))

;; Minting with Supply Cap
(define-public (mint (recipient principal) (amount uint))
    (begin
        (asserts! (is-eq tx-sender contract-owner) ERR-OWNER-ONLY)
        (asserts! (<= (+ (ft-get-supply anya-token) amount) MAX_SUPPLY) ERR-SUPPLY-LIMIT)
        (ft-mint? anya-token amount recipient)))

;; Burning Mechanism
(define-public (burn (amount uint))
    (begin
        (asserts! (>= (ft-get-balance anya-token tx-sender) amount) ERR-INSUFFICIENT-BALANCE)
        (ft-burn? anya-token amount tx-sender)))

;; Delegation Mechanism
(define-map delegated-power principal principal)

(define-public (delegate (delegatee principal))
    (begin
        (map-set delegated-power tx-sender delegatee)
        (ok true)))

(define-read-only (get-delegatee (account principal))
    (ok (map-get? delegated-power account)))
