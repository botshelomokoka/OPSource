(define-constant ERR_UNAUTHORIZED (err u100))

(define-public (execute-governance-proposal (proposal (string-ascii 256)))
  (asserts! (is-eq (contract-caller?) .gov-address) ERR_UNAUTHORIZED)
  (match (parse-proposal proposal)
    (proposal-struct 
      (begin
        (try! (validate-proposal proposal-struct))
        (execute-approved-proposal proposal-struct)))
    (err ERR_INVALID_PROPOSAL)
  )
)

(define-data-var total-stacked uint u0)

(define-public (delegate-stx (amount uint) (pool principal))
  (let ((current (stx-get-balance tx-sender)))
    (asserts! (>= current amount) ERR_INSUFFICIENT_BALANCE)
    (asserts! (is-contract pool) ERR_INVALID_POOL)
    
    (var-set total-stacked (+ (var-get total-stacked) amount))
    (ok (stx-transfer? amount tx-sender pool))
  )
) 