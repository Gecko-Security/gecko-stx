"use client";

import { useState } from "react";
import styles from "@/app/ui/dashboard/scan/addScan/addScan.module.css";

const AddPage = () => {
    const [selectedOption, setSelectedOption] = useState("general");

    const handleSelectChange = (event) => {
        setSelectedOption(event.target.value);
    };

    return (
        <div className={styles.container}>
            <form action="" className={styles.form}>
                <input type="text" placeholder="Name" name="title" required />
                <select name="input" id="input" onChange={handleSelectChange} value={selectedOption}>
                    <option value="general">Setup Contract</option>
                    <option value="deployed">Deployed Contract</option>
                    <option value="upload">Upload Contract</option>
                </select>
                {selectedOption === "deployed" && (
                    <input className={styles.address} type="text" placeholder="Contract Address" name="address" />
                )}
                {selectedOption === "upload" && (
                    <textarea name="code" id="code" rows="16" placeholder="Contract Code" />
                )}
                <button type="scan">Scan</button>
            </form>
        </div>
    );
};

export default AddPage;
