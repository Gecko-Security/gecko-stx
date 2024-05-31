import Image from "next/image";
import styles from "./transactions.module.css";

const Transactions = () => {
  return (
    <div className={styles.container}>
      <h2 className={styles.title}>Latest Tests</h2>
      <table className={styles.table}>
        <thead>
          <tr>
            <td>Name</td>
            <td>Chain</td>
            <td>Date</td>
            <td>Vulnearbilities</td>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>
              <div className={styles.user}>
                <Image
                  src="/NOCC.png"
                  alt=""
                  width={40}
                  height={40}
                  className={styles.userImage}
                />
                NoCodeClarity-Token (NOCC)
              </div>
            </td>
            <td>
              <span className={`${styles.status} ${styles.pending}`}>
                Mainnet
              </span>
            </td>
            <td>29.05.2024</td>
            <td>0</td>
          </tr>
          <tr>
            <td>
              <div className={styles.user}>
                <Image
                  src="/Alex.png"
                  alt=""
                  width={40}
                  height={40}
                  className={styles.userImage}
                />
                Alex
              </div>
            </td>
            <td>
              <span className={`${styles.status} ${styles.pending}`}>Mainnet</span>
            </td>
            <td>29.05.2024</td>
            <td>0</td>
          </tr>
          <tr>
            <td>
              <div className={styles.user}>
                <Image
                  src="/noavatar.png"
                  alt=""
                  width={40}
                  height={40}
                  className={styles.userImage}
                />
                vulnbank
              </div>
            </td>
            <td>
              <span className={`${styles.status} ${styles.done}`}>
                Testnet
              </span>
            </td>
            <td>29.05.2024</td>
            <td>1</td>
          </tr>
          <tr>
            <td>
              <div className={styles.user}>
                <Image
                  src="/BTJ.png"
                  alt=""
                  width={40}
                  height={40}
                  className={styles.userImage}
                />
                Bitjoy (BTJ)
              </div>
            </td>
            <td>
              <span className={`${styles.status} ${styles.pending}`}>
                Mainnet
              </span>
            </td>
            <td>29.05.2024</td>
            <td>0</td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default Transactions;