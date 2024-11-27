;; Anya DAO Contract
;; Implements core DAO functionality for governance

(use-trait ft-trait 'SP3FBR2AGK5H9QBDH3EEN6DF8EK8JY7RX8QJ5SVTE.sip-010-trait-ft-standard.sip-010-trait)

;; Constants
(define-constant contract-owner tx-sender)
(define-constant proposal-duration u10080) ;; ~1 week in blocks
(define-constant timelock-delay u2880) ;; ~2 days in blocks
(define-constant proposal-threshold u100000000000) ;; 100 tokens
(define-constant quorum-votes u500000000000) ;; 500 tokens

;; Error codes
(define-constant err-owner-only (err u100))
(define-constant err-insufficient-balance (err u101))
(define-constant err-invalid-proposal (err u102))
(define-constant err-proposal-exists (err u103))
(define-constant err-proposal-expired (err u104))
(define-constant err-already-voted (err u105))
(define-constant err-not-executable (err u106))

;; Data structures
(define-map proposals
    uint
    {
        proposer: principal,
        title: (string-ascii 100),
        description: (string-utf8 1000),
        start-block: uint,
        end-block: uint,
        execution-block: uint,
        votes-for: uint,
        votes-against: uint,
        executed: bool,
        canceled: bool
    })

(define-map votes
    {proposal-id: uint, voter: principal}
    {support: bool, votes: uint})

(define-map action-queue
    uint
    {
        target: principal,
        function: (string-ascii 128),
        args: (list 10 (string-ascii 100))
    })

;; Governance token
(define-data-var governance-token principal 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.anya-token)

;; Read-only functions
(define-read-only (get-proposal (proposal-id uint))
    (map-get? proposals proposal-id))

(define-read-only (get-votes (proposal-id uint) (voter principal))
    (map-get? votes {proposal-id: proposal-id, voter: voter}))

(define-read-only (get-actions (proposal-id uint))
    (map-get? action-queue proposal-id))

;; Public functions
(define-public (set-governance-token (token principal))
    (begin
        (asserts! (is-eq tx-sender contract-owner) err-owner-only)
        (var-set governance-token token)
        (ok true)))

(define-public (propose 
    (title (string-ascii 100))
    (description (string-utf8 1000))
    (target principal)
    (function (string-ascii 128))
    (args (list 10 (string-ascii 100))))
    (let
        ((proposal-id (+ (default-to u0 (get-last-proposal-id)) u1))
         (start-block block-height)
         (end-block (+ block-height proposal-duration))
         (execution-block (+ end-block timelock-delay)))
        
        ;; Check proposer has enough tokens
        (asserts! (>= (contract-call? (var-get governance-token) get-balance tx-sender)
                     proposal-threshold)
                 err-insufficient-balance)
        
        ;; Create proposal
        (map-set proposals proposal-id
            {
                proposer: tx-sender,
                title: title,
                description: description,
                start-block: start-block,
                end-block: end-block,
                execution-block: execution-block,
                votes-for: u0,
                votes-against: u0,
                executed: false,
                canceled: false
            })
        
        ;; Queue actions
        (map-set action-queue proposal-id
            {
                target: target,
                function: function,
                args: args
            })
        
        (ok proposal-id)))

(define-public (cast-vote (proposal-id uint) (support bool))
    (let
        ((proposal (unwrap! (get-proposal proposal-id) err-invalid-proposal))
         (voter-balance (unwrap! (contract-call? (var-get governance-token)
                                               get-voting-power-at
                                               tx-sender
                                               (get start-block proposal))
                               err-insufficient-balance)))
        
        ;; Check proposal is active
        (asserts! (< block-height (get end-block proposal)) err-proposal-expired)
        (asserts! (is-none (get-votes proposal-id tx-sender)) err-already-voted)
        
        ;; Record vote
        (map-set votes
            {proposal-id: proposal-id, voter: tx-sender}
            {support: support, votes: voter-balance})
        
        ;; Update vote totals
        (map-set proposals proposal-id
            (merge proposal
                {votes-for: (if support
                              (+ (get votes-for proposal) voter-balance)
                              (get votes-for proposal)),
                 votes-against: (if support
                                 (get votes-against proposal)
                                 (+ (get votes-against proposal) voter-balance))}))
        
        (ok true)))

(define-public (execute (proposal-id uint))
    (let
        ((proposal (unwrap! (get-proposal proposal-id) err-invalid-proposal))
         (actions (unwrap! (get-actions proposal-id) err-invalid-proposal)))
        
        ;; Check execution conditions
        (asserts! (>= block-height (get execution-block proposal)) err-not-executable)
        (asserts! (not (get executed proposal)) err-not-executable)
        (asserts! (not (get canceled proposal)) err-not-executable)
        (asserts! (> (get votes-for proposal) (get votes-against proposal)) err-not-executable)
        (asserts! (>= (get votes-for proposal) quorum-votes) err-not-executable)
        
        ;; Execute action
        (contract-call? (get target actions)
                       (get function actions)
                       (get args actions))
        
        ;; Mark as executed
        (map-set proposals proposal-id
            (merge proposal {executed: true}))
        
        (ok true)))

(define-private (get-last-proposal-id)
    (map-get? proposals u0))
