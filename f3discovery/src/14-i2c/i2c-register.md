### 🧭 Key Registers (Master Mode)

| Register   | Role                                         |
| ---------- | -------------------------------------------- |
| `I2C_CR1`  | Control register 1 (enable, ACK, etc.)       |
| `I2C_CR2`  | Control register 2 (start, address, count)   |
| `I2C_ISR`  | Interrupt & status flags                     |
| `I2C_ICR`  | Interrupt clear register (clear NACKF, etc.) |
| `I2C_TXDR` | Transmit data register (write data)          |
| `I2C_RXDR` | Receive data register (read data)            |

### ✅ SOFTWARE Sets These

| Register | Field                    | Meaning                            |
| -------- | ------------------------ | ---------------------------------- |
| `CR1`    | `PE`                     | Peripheral enable bit              |
| `CR1`    | `TXIE`, `RXIE`           | Optional: enable TX/RX interrupt   |
| `CR2`    | `SADD[9:0]`              | Slave 7-bit address                |
| `CR2`    | `RD_WRN`                 | Direction: `0 = write`, `1 = read` |
| `CR2`    | `START`                  | Set to trigger START condition     |
| `CR2`    | `STOP`                   | Set to trigger STOP condition      |
| `CR2`    | `NBYTES`                 | Number of bytes in the transfer    |
| `TXDR`   | --                       | Byte to transmit                   |
| `ICR`    | `NACKCF`, `STOPCF`, etc. | Clear interrupt flags              |

### 👀 SOFTWARE Monitors These (Set by Hardware)

| Register | Field   | Meaning                                                    |
| -------- | ------- | ---------------------------------------------------------- |
| `ISR`    | `TXIS`  | TXDR is empty, ready for next byte                         |
| `ISR`    | `RXNE`  | RXDR is full, data ready to read                           |
| `ISR`    | `TC`    | Transfer Complete (all bytes sent/received)                |
| `ISR`    | `TCR`   | Transfer Complete (reload), used in segmented transfers    |
| `ISR`    | `NACKF` | NACK received from slave                                   |
| `ISR`    | `STOPF` | STOP condition detected                                    |
| `ISR`    | `BUSY`  | Bus is busy                                                |
| `ISR`    | `ADDR`  | Address matched (in slave mode) — usually unused by master |

### 🧠 Typical Flow (Master TX/RX)

**Master Write:**

1. Set PE = 1 in CR1
2. Set address + NBYTES + RD_WRN = 0 + START in CR2
3. Wait for TXIS, write data to TXDR
4. After all bytes, wait for TC, then optionally set STOP
5. Wait for STOPF, clear with ICR.STOPCF

**Master Read:**

1. Same setup, but RD_WRN = 1
2. Wait for RXNE, read from RXDR
3. Wait for TC, set STOP
4. Wait for STOPF, clear

### ✅ Example 1: Master Writes 2 Bytes to a Slave

**Scenario:**
Master sends: Start → Address (Write) → Byte1 → Byte2 → Stop

**Timeline of Communication:**
Master: START
Master: Send 0x50 << 1 | 0 (write)
Slave: ACK
Master: Send Byte1 = 0xAB
Slave: ACK
Master: Send Byte2 = 0xCD
Slave: ACK
Master: STOP

### ✅ Example 2: Master Reads 1 Byte from a Slave

**Scenario:**
Master sends: Start → Address (Read) → Byte ← Slave → NACK → Stop

**Timeline:**
Master: START
Master: Send 0x50 << 1 | 1 (read)
Slave: ACK
Slave: Sends Byte = 0x7F
Master: NACK (we're done reading)
Master: STOP
