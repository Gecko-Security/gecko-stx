import styles from './card.module.css';
import Image from "next/image";
import { MdSupervisedUserCircle } from 'react-icons/md';

const Card = ({ title, time, number, detail, positive }) => {
    const detailClass = positive ? styles.positive : styles.negative;

    return (
        <div className={styles.container}>
            <Image className={styles.img} src="/clarity.png" alt="" width={60} height={60} />
            <div className={styles.texts}>
                <span className={styles.title}>{title}</span>
                <span className={styles.time}>{time}</span>
                <span className={styles.number}>{number}</span>
                <span className={styles.detail}>
                    <span className={detailClass}>{positive ? "No Vulnerability" : " Vulnerability:"}</span> {detail}
                </span>
            </div>
        </div>
    );
};

export default Card;

