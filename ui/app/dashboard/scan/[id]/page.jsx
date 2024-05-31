import styles from "@/app/ui/dashboard/scan/singleScan/singleScan.module.css"
import Image from 'next/image'

const SingleScanPage = () => {

    const formattedText = `
    vulnerability: unsafe authentication via tx-sender
--> contracts/vulnbank.clar:12
(asserts! (is-eq tx-sender sender) err-not-token-owner)
                 ^~~~~~
The misuse of tx-sender in Clarity smart contracts can be exploited by malicious contracts to perform unauthorized actions by making it appear that the original user authorized them.

informational: unchecked data
--> contracts/vulnbank.clar:13
(try! (ft-transfer? vulnerable-token amount sender recipient))
                                     ^~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:10
(define-public (transfer (amount uint) (sender principal) (recipient principal))
                          ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:13
(try! (ft-transfer? vulnerable-token amount sender recipient))
                                                    ^~~~~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:10
(define-public (transfer (amount uint) (sender principal) (recipient principal))
                                                           ^~~~~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:19
(map-set accounts {holder: tx-sender} {amount: (+ balance (to-int amount))})
                                                ^~~~~~~~~~~~~~~~~~~~~~~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:17
(define-public (deposit (amount uint))
                         ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:30
(map-set accounts {holder: tx-sender} {amount: (- balance (to-int amount))})
                                                ^~~~~~~~~~~~~~~~~~~~~~~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:25
(define-public (withdrawal-unsafe (amount uint))
                                   ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:31
(as-contract (stx-transfer? amount tx-sender customer))
                            ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:60
(try! (check-balance amount))
                     ^~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:55
(define-public (withdrawal-callee-filter (amount uint))
                                          ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:61
(map-set accounts {holder: tx-sender} {amount: (- balance (to-int amount))})
		                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:62
(as-contract (stx-transfer? amount tx-sender customer))
                            ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:79
(map-set accounts {holder: from} {amount: balance})
                            ^~~~
informational: untrusted input
--> contracts/vulnbank.clar:73
(define-public (take (amount uint) (from principal))
                                    ^~~~
informational: unchecked data
--> contracts/vulnbank.clar:79
(map-set accounts {holder: from} {amount: balance})
                                         ^~~~~~~
informational: untrusted input
--> contracts/vulnbank.clar:73
(define-public (take (amount uint) (from principal))
                      ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:80
(as-contract (stx-transfer? amount tx-sender banker))
                            ^~~~~~
informational: unchecked data
--> contracts/vulnbank.clar:91
(map-set accounts {holder: from} {amount: balance})
                            ^~~~
informational: untrusted input
--> contracts/vulnbank.clar:87
(define-public (take-after-time (amount uint) (from principal))
                                               ^~~~
    
    
    
    
    
    
    `


    return (
        <div className={styles.container}>
          <div className={styles.infoContainer}>
            <div className={styles.imgContainer}>
              <Image src={"/clarity.png"} alt="" fill />
            </div>
            ST3BK6FP3KM9Y...CZSVW5KPM.vulnbank
          </div>
          <div className={styles.formContainer}>
            <form className={styles.form}>
              <input type="hidden" name="id" />
              <label className={styles.label}>
                <pre>{formattedText}</pre>
              </label>
            </form>
          </div>
        </div>
      );
}

export default SingleScanPage