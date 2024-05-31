import Card from "../ui/dashboard/card/card"
import styles from "../ui/dashboard/dashboard.module.css"
import Transactions from "../ui/dashboard/transactions/transactions"
import Rightbar from "../ui/dashboard/rightbar/rightbar"

const Dashboard = () => {
    return (
        <div className={styles.wrapper}>
            <div className={styles.main}>
                <div className={styles.cards}> 
                    <Card title="Vulnerable Bank" time="Last Updated: 30/05/2024, 7:10:02 AM" number="SP6527H2G38..." detail="2" positive={false} />
                    <Card title="Test Contract" time="Last Updated: 30/05/2024, 7:10:02 AM" number="SP2470N2A31..." detail="1" positive={false} />
                    <Card title="ALEX Token" time="Last Updated: 30/05/2024, 7:10:02 AM" number="SP1YK770QXS..." detail="" positive={true} />
                </div>
                <Transactions/>
            </div>
            <div className={styles.side}>
                <Rightbar/>
            </div>
        </div>
    )
}

export default Dashboard