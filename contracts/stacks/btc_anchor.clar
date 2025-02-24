(define-read-only (verify-btc-tx 
    (block-header (buff 80))
    (merkle-proof { 
        tx-hash: (buff 32), 
        index: uint, 
        path: (list 32 (buff 32)) 
    }))
  (ok bool)
  (try! (btc-verify-block-header block-header))
  (btc-verify-tx-inclusion 
    block-header 
    (get tx-hash merkle-proof)
    (get index merkle-proof)
    (get path merkle-proof))) 