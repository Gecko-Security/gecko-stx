import styles from "@/app/ui/dashboard/scan/scan.module.css"
import Link from "next/link"
import Image from "next/image";

const ScanPage = () => {
    return (
        <div className={styles.container}>
            <div className={styles.top}></div>
            <Link href="/dashboard/scan/add">
                <button className={styles.addButton}>New Scan</button>
            </Link>
            <div className={styles.table}>
                <table className={styles.table}>
                    <thead>
                        <tr>
                            <td>Name</td>
                            <td>Address</td>
                            <td>Critical</td>
                            <td>High</td>
                            <td>Med</td>
                            <td>Low</td>
                            <td>Warnings</td>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div className={styles.scan}>
                                    <Image 
                                        src="/noavatar.png"
                                        alt=""
                                        width={40}
                                        height={40}
                                        className={styles.scanImage}
                                    />
                                    vulnbank
                                </div>
                            </td>
                            <td>ST3BK6FP3KM52GX4K9YPBB1J9S7K0R9WCZSVW5KPM.vulnbank</td>
                            <td>0</td>
                            <td>1</td>
                            <td>0</td>
                            <td>0</td>
                            <td>20</td>
                            <td>
                            <div className={styles.buttons}>
                                <Link href="/dashboard/scan/test">
                                    <button className={`${styles.button} ${styles.view}`}>View</button>
                                </Link>
                                    <button className={`${styles.button} ${styles.delete}`}>Delete</button>
                            </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    )
}

export default ScanPage