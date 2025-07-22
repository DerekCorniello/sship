# Protocol Opcodes

`sship` uses a minimal, binary protocol built on top of a multiplexed SSH channel. Each message starts with a 2-byte header: first 4 bits = **OpCode**, remaining 12 bits = **payload length**, followed by the payload. This protocol is designed to be resilient, resumable, extensible, and highly debuggable.

---

## Design Philosophy

* **No Versioning:** Deliberately omit version fields in normal communication to avoid unnecessary complexity. Incompatible protocol changes should happen on distinct code branches or via separate handshake OpCodes (e.g. `0x0` and `0x1`). Simplicity > flexibility in small tools.

* **Payload Simplicity:** Everything is binary, fixed-size when possible. Strings are length-prefixed (`u16`), no JSON, no protobuf, no bloat. This keeps parsing and debugging straightforward and low-overhead.

* **Endian:** All multi-byte values are encoded in **little-endian** for consistency and to simplify implementation on little-endian architectures (dominant on modern hardware).

* **Resumability:** Transfers can resume mid-stream via file offset tracking and hash validation. This avoids restarting large transfers from scratch and enables robustness over flaky networks.

* **Stream Multiplexing:** Multiple logical streams (e.g., data, metadata, control) multiplexed over a single SSH channel optimize resource usage and keep the implementation simple.

* **Flow Control:** Sliding window and congestion control are delegated to the underlying TCP/SSH transport layer. The protocol focuses on transfer semantics, integrity, and resumability without reinventing flow control.

* **Separation of Concerns:** Data packets (`Send Data`) and integrity verification packets (`Hash Check`) are sent separately to allow asynchronous validation, modular retransmission, and easier error handling.

* **Fixed-Size Headers:** Using fixed-size headers with a 2-byte combined OpCode and payload length ensures quick parsing and efficient packet framing without complex length-delimited schemes.

* **Security Through SSH:** The protocol relies on SSH’s built-in encryption and authentication, avoiding complexity of designing a secure transport while benefiting from well-tested cryptographic guarantees.

* **Simple Handshake:** Minimal handshake with pairing codes and boolean accept flags to keep connection setup straightforward, suitable for both manual and automated pairing.

* **Explicit Error Signaling:** Dedicated error opcode with human-readable messages enables clear and debuggable failure reporting without ambiguity.

* **Limited Payload Size:** Payload length limited to 4095 bytes per packet (12-bit length field) balances efficiency with resource constraints, especially for embedded or constrained environments.

* **Minimal External Dependencies:** Avoids third-party serialization or protocols, making the implementation portable, lightweight, and easy to audit.

* **Debug-Friendly:** Protocol format is designed to be easily inspected with hex editors or packet analyzers (Wireshark), aiding development and troubleshooting.

* **Extensibility:** Reserved bits and opcodes allow for future protocol extensions without breaking compatibility.



---

## Opcode Table

| OpCode (hex) | Name                  | Payload Description                                                | Purpose & Usage                                                                 |
| ------------ | --------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| `0x0`        | **Handshake Init**    | Pairing code (`u8 len + bytes`)                                    | Sent by sender to initiate transfer. Pairing code format: "XXXX-YYYY"           |
| `0x1`        | **Handshake Ack**     | Match (`bool`)                                                     | Receiver approves or denies handshake.                                          |
| `0x2`        | **Send Data**         | Chunk Offset (`u64`), Data (`[u8]`)                                | Sends raw file data at given offset. Data length determined by header.          |
| `0x3`        | **Ack Data**          | Offset (`u64`)                                                     | Acknowledges receipt up to offset (inclusive).                                  |
| `0x4`        | **Request Data**      | StartOffset (`u64`), OffsetEnd (`u64`)                             | Requests a specific chunk from the sender (inclusive).                          |
| `0x5`        | **Request Resume**    | Resume Offset (`u64`)                             | Asks peer to continue transfer from offset.                                     |
| `0x6`        | **Resume Ack**        | Resume Offset (`u64`), Accept Flag (`bool`)       | Accepts resume request, syncs both ends.                                        |
| `0x7`        | **Hash Check**        | SHA-256 Digest (`[u8; 32]`), Offset (`u64`), Length (`u32`)        | Propose digest verification of received chunk.                                  |
| `0x8`        | **Hash Ack**          | Match (`bool`)                                                     | Receiver confirms hash match or failure.                                        |
| `0x9`        | **Error**             | Code (`u8`), Length (`u8`), UTF-8 string                           | Sends human-readable error string.                                              |
| `0xA`        | **Ping**              | None                                                               | Sender wants a response—e.g. latency check or keepalive.                        |
| `0xB`        | **Pong**              | None                                                               | Response to `0xA`.                                                              |
| `0xC`        | **Metadata**          | Name (`u8 len + bytes`), Size (`u64`), Mode (`u16`), mtime (`u64`) | Provides file info before transfer. Multiple files = multiple `0xC`.            |
| `0xD`        | **Transfer Complete** |                                                                    | Signals successful end of transfer.                                             |
| `0xE`        | **Cancel**            | Reason (`u8 len + bytes`)                                          | Abort file or stream. Optional reason.                                          |

---

# Opcode Descriptions

## OpCode `0x0` Handshake Init — Table Description

| Offset | Size    | Type       | Field               | Description                                       |
|--------|---------|------------|---------------------|---------------------------------------------------|
| 0      | 2 bytes | u16 (LE)   | Header              | Bits 0-3: OpCode `0x0`, Bits 4-15: Payload length |
| 2      | 1 byte  | u8         | Pairing Code Length | Length of pairing code bytes                      |
| 3      | N bytes | UTF-8 bytes| Pairing Code        | Sender pairing code string "XXXX-YYYY" format     |

---

### `0x0` Handshake Init — Visual Byte Layout

```  
+------------------+----------------------+----------------+
| Header (u16)     | Pairing Code Length  | Pairing Code   |
+------------------+----------------------+----------------+
| 2 bytes          | 1 byte               | variable       |
+------------------+----------------------+----------------+
```

---

## OpCode `0x1` Handshake Ack — Table Description

| Offset | Size   | Type  | Field       | Description                                |
|--------|--------|-------|-------------|--------------------------------------------|
| 0      | 1 byte | bool  | Accept Flag | `0x1` = accept handshake, `0x0` = reject   |

---

### `0x1` Handshake Ack — Visual Byte Layout

```  
+--------+-------------+
| 0x1    | Accept Flag |
+--------+-------------+
| 1 byte | 1 byte      |
+--------+-------------+
```

---

## OpCode `0x2` Send Data — Table Description

| Offset | Size    | Type        | Field        | Description                                       |
|--------|---------|-------------|--------------|---------------------------------------------------|
| 0      | 2 bytes | u16 (LE)    | Header       | Bits 0-3: OpCode `0x2`, Bits 4-15: Payload length |
| 2      | 8 bytes | u64 (LE)    | Chunk Offset | Byte offset of this chunk in the file             |
| 10     | N bytes | raw bytes   | Data         | Actual binary chunk data (length from header)     |

---

### `0x2` Send Data — Visual Byte Layout

```  
+------------------+------------------+--------------+
| Header (u16)     | Chunk Offset (u64)| Data        |
+------------------+------------------+--------------+
| 2 bytes          | 8 bytes          | variable     |
+------------------+------------------+--------------+
```

---

## OpCode `0x3` Ack Data — Table Description

| Offset | Size    | Type      | Field       | Description                                    |
|--------|---------|-----------|-------------|------------------------------------------------|
| 0      | 1 byte  | u8        | OpCode      | Fixed opcode byte: `0x3`                       |
| 1      | 8 bytes | u64 (LE)  | Offset      | Acknowledged byte offset (inclusive)           |

---

### `0x3` Ack Data — Visual Byte Layout

```  
+--------+----------------+
| 0x3    | Offset (u64)   |
+--------+----------------+
| 1 byte | 8 bytes        |
+--------+----------------+
```

---

## OpCode `0x4` Request Data — Table Description

| Offset | Size    | Type      | Field         | Description                                                               |
|--------|---------|-----------|---------------|---------------------------------------------------------------------------|
| 0      | 1 byte  | u8        | OpCode        | Fixed opcode byte: `0x4`                                                  |
| 1      | 8 bytes | u64 (LE)  | OffsetStart   | Starting byte offset of the requested chunk (inclusive)                  |
| 9      | 8 bytes | u64 (LE)  | OffsetEnd     | Ending byte offset of the requested chunk (inclusive)                    |

---

### `0x4` Request Data — Visual Byte Layout

```
+--------+----------------------+------------------------+
| 0x4    | OffsetStart (u64 LE) | OffsetEnd (u64 LE)     |
+--------+----------------------+------------------------+
| 1 byte |        8 bytes       |        8 bytes         |
+--------+----------------------+------------------------+
```

## OpCode `0x5` Request Resume — Table Description

| Offset | Size    | Type     | Field         | Description                        |
|--------|---------|----------|---------------|------------------------------------|
| 0      | 1 byte  | u8       | OpCode        | Fixed opcode byte: `0x5`           |
| 1      | 8 bytes | u64 (LE) | Resume Offset | Offset where resuming is requested |

---

### `0x5` Request Resume — Visual Byte Layout

```  
+--------+------------------+
| 0x5    | Resume Offset    |
+--------+------------------+
| 1 byte | 8 bytes          |
+--------+------------------+
```

---

## OpCode `0x6` Resume Ack — Table Description

| Offset | Size    | Type     | Field       | Description                           |
|--------|---------|----------|-------------|---------------------------------------|
| 0      | 1 byte  | u8       | OpCode      | Fixed opcode byte: `0x6`              |
| 1      | 8 bytes | u64 (LE) | Resume Offset | Confirmed resume offset             |
| 9      | 1 byte  | bool     | Accept Flag | `0x1` = accept resume, `0x0` reject   |

---

### `0x6` Resume Ack — Visual Byte Layout

```  
+--------+---------------+-------------+
| 0x6    | Resume Offset | Accept Flag |
+--------+---------------+-------------+
| 1 byte | 8 bytes       | 1 byte      |
+--------+---------------+-------------+
```

---

## OpCode `0x7` Hash Check — Table Description

| Offset | Size     | Type       | Field       | Description                               |
|--------|----------|------------|-------------|-------------------------------------------|
| 0      | 1 byte   | u8         | OpCode      | Fixed opcode byte: `0x7`                  |
| 1      | 32 bytes | [u8;32]    | SHA-256     | Hash digest to verify                     |
| 33     | 8 bytes  | u64 (LE)   | Offset      | Start byte offset of hashed data          |
| 41     | 4 bytes  | u32 (LE)   | Length      | Length of hashed data                     |

---

### `0x7` Hash Check — Visual Byte Layout

```  
+--------+-----------------------------------+------------+---------+
| 0x7   | SHA-256 Digest (32 bytes)          | Offset     | Length  |
+--------+-----------------------------------+------------+---------+
| 1 byte | 32 bytes                          | 8 bytes    | 4 bytes |
+--------+-----------------------------------+------------+---------+
```

---

## OpCode `0x8` Hash Ack — Table Description

| Offset | Size   | Type  | Field     | Description                       |
|--------|--------|-------|-----------|-----------------------------------|
| 0      | 1 byte | u8    | OpCode    | Fixed opcode byte: `0x8`          |
| 1      | 1 byte | bool  | Match     | `0x1` = hash matches, `0x0` no    |

---

### `0x8` Hash Ack — Visual Byte Layout

```  
+--------+-------+
| 0x8   | Match |
+--------+-------+
| 1 byte | 1 byte|
+--------+-------+
```

---

## OpCode `0x9` Error — Table Description

| Offset | Size    | Type       | Field         | Description                              |
|--------|---------|------------|---------------|------------------------------------------|
| 0      | 1 byte  | u8         | OpCode        | Fixed opcode byte: `0x9`                 |
| 1      | 1 byte  | u8         | Error Code    | Numeric error code (TBD)                 |
| 2      | 1 byte  | u8         | Msg Length    | Length of error message                  |
| 3      | N bytes | UTF-8 bytes| Message       | Human-readable error string              |

---

### `0x9` Error — Visual Byte Layout

```  
+--------+------------+------------+-------------------+
| 0x9    | Error Code | Msg Length | Message           |
+--------+------------+------------+-------------------+
| 1 byte | 1 byte     | 1 byte     | variable          |
+--------+------------+------------+-------------------+
```

---

## OpCode `0xA` Ping — Table Description

| Offset | Size   | Type | Field  | Description                         |
|--------|--------|------|--------|-------------------------------------|
| 0      | 1 byte | u8   | OpCode | Fixed opcode byte: `0xA`            |

---

### `0xA` Ping — Visual Byte Layout

```  
+--------+
| 0xA    |
+--------+
| 1 byte |
+--------+
```

---

## OpCode `0xB` Pong — Table Description

| Offset | Size   | Type | Field  | Description                         |
|--------|--------|------|--------|-------------------------------------|
| 0      | 1 byte | u8   | OpCode | Fixed opcode byte: `0xB`            |

---

### `0xB` Pong — Visual Byte Layout

```  
+--------+
| 0xB    |
+--------+
| 1 byte |
+--------+
```

---

## OpCode `0xC` Metadata — Table Description

| Offset | Size      | Type       | Field         | Description                          |
|--------|-----------|------------|---------------|--------------------------------------|
| 0      | 1 byte    | u8         | OpCode        | Fixed opcode byte: `0xC`             |
| 1      | 1 byte    | u8         | Name Length   | Length of filename                   |
| 2      | N bytes   | UTF-8 bytes| Name          | File name, UTF-8 encoded             |
| 2 + N  | 8 bytes   | u64 (LE)   | Size          | File size in bytes                   |
| 10 + N | 2 bytes   | u16 (LE)   | Mode          | POSIX file mode bits                 |
| 12 + N | 8 bytes   | u64 (LE)   | mtime         | Last modified UNIX timestamp         |

---

### `0xC` Metadata — Visual Byte Layout

```  
+--------+------------+-----------------+---------+--------+----------+
| 0xC   | Name Len   | Name            | Size    |  Mode  |  mtime   |
+--------+------------+-----------------+---------+--------+----------+
| 1 byte | 1 byte     | variable        | 8 bytes |2 bytes | 8 bytes  |
+--------+------------+-----------------+---------+--------+----------+
```

---

## OpCode `0xD` Transfer Complete — Table Description

| Offset | Size    | Type   | Field  | Description                             |
|--------|---------|--------|--------|-----------------------------------------|
| 0      | 1 byte  | u8     | OpCode | Fixed opcode byte: `0xD`                |

---

### `0xD` Transfer Complete — Visual Byte Layout

```  
+--------+
| 0xD    |
+--------+
| 1 byte |
+--------+
```

---

## OpCode `0xE` Cancel — Table Description

| Offset | Size      | Type       | Field         | Description                          |
|--------|-----------|------------|---------------|--------------------------------------|
| 0      | 1 byte    | u8         | OpCode        | Fixed opcode byte: `0xE`             |
| 1      | 1 byte    | u8         | Reason Length | Length of optional reason string     |
| 2      | N bytes   | UTF-8 bytes| Reason        | Optional cancellation reason message |

---

### `0xE` Cancel — Visual Byte Layout

```  
+--------+--------------+----------------+
| 0xE    | Reason Length| Reason         |
+--------+--------------+----------------+
| 1 byte | 1 byte       | variable       |
+--------+--------------+----------------+
```


#### Field Notes

* **Header (u16):** First 4 bits = OpCode (0x0-0xF), remaining 12 bits = payload length (0-4095 bytes)
* **File Offset (u64):** Used in `Send Data`, `Ack Data`, `Resume`, etc. Allows for files up to 16 exabytes and enables sparse, out-of-order chunks with full resumability.
* **Lengths:** Strings are length-prefixed with 8-bit unsigned integers for this protocol.
* **Pairing Codes:** Generated internally using RNG in "XXXX-YYYY" format (9 bytes including hyphen).
* **Maximum Payload:** 4095 bytes per message (limited by 12-bit length field)

---

# Example Scenarios



---

## Error Handling

If a file hash fails:

```binary
Receiver:
0x8 [match: false]
0x9 [code: 0x2, message: "Hash mismatch"]
Sender:
0xE ["Corrupt chunk"]
```

---

## Reserved OpCodes

* `0xF` is reserved for future use.
* Lower 4 bits in all opcodes are reserved for future subtypes or flags.

---

## Implementation Notes

* **Flow Control:** TCP-style sliding window flow control is handled at the SSH transport layer
* **Security:** Pairing code validation and generation handled internally 
* **Maximum Chunk Size:** Implementation-dependent (limited by u16 length field = 65535 bytes max)
* **Resume Granularity:** TBD - may support mid-chunk or chunk-boundary resume only

---

## Summary

This protocol is designed to:

* Keep things small and tight (great for embedded systems)
* Require no third-party serialization formats
* Be easy to debug with a hex editor or Wireshark
* Be resumable, cancelable, and efficient
* Work over any bidirectional stream (SSH, pipes, sockets)
* Leverage SSH's built-in flow control and security features
