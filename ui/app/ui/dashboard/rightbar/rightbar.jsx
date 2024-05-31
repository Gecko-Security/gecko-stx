import Image from "next/image";
import styles from "./rightbar.module.css";
import { CiCirclePlus } from "react-icons/ci";

const Rightbar = () => {
  return (
    <div className={styles.container}>
      <div className={styles.item}>
        <div className={styles.bgContainer}>
          <Image className={styles.bg} src="/code_scan.png" alt="" fill />
        </div>
        <div className={styles.text}>
          <span className={styles.notification}>Code Scanned</span>
          <h3 className={styles.title}>
            5342 Lines
          </h3>
          <span className={styles.subtitle}></span>
          <p className={styles.desc}>
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
            Reprehenderit eius libero perspiciatis recusandae possimus.
          </p>
          <button className={styles.button}>
            <CiCirclePlus />
            New Scan
          </button>
        </div>
      </div>
      <div className={styles.item}>
      <div className={styles.bgContainer}>
          <Image className={styles.bg} src="/monitored_projects.png" alt="" fill />
        </div>
        <div className={styles.text}>
          <span className={styles.notification}>Monitred Projects</span>
          <h3 className={styles.title}>
            6 Projects
          </h3>
          <span className={styles.subtitle}></span>
          <p className={styles.desc}>
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
            Reprehenderit eius libero perspiciatis recusandae possimus.
          </p>
          <button className={styles.button}>
            <CiCirclePlus />
            New Scan
          </button>
        </div>
      </div>
    </div>
  );
};

export default Rightbar;