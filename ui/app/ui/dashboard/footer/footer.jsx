import styles from './footer.module.css'

const Footer = () => {
    return (
        <div className={styles.container}>
            <div className={styles.logo}>Gecko Security</div>
            <div className={styles.text}>Consesus Hackathon 2024</div>
        </div>
    )
}

export default Footer