# 🔧 Log Generator

A high-performance Rust application that generates realistic, enterprise-grade log files at scale. Built for stress-testing and validating log aggregation platforms such as Splunk, Kibana, Datadog, Grafana Loki, and similar tools.

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build & Run](#build--run)
- [Usage](#usage)
  - [Web UI](#web-ui)
  - [API Endpoints](#api-endpoints)
- [Log Format](#log-format)
- [Log Level Distribution](#log-level-distribution)
- [Message Variety](#message-variety)
- [File Rotation](#file-rotation)
- [Directory Structure](#directory-structure)
- [Architecture](#architecture)
- [Configuration Limits](#configuration-limits)
- [Performance](#performance)
- [Tech Stack](#tech-stack)

---

## Overview

When developing or evaluating an enterprise logging platform, you need **a lot** of realistic log data — gigabytes or even terabytes of it — structured exactly how production microservices would emit it. Manually creating test data is tedious and unrealistic. This tool solves that problem.

Log Generator spins up a lightweight web server with a clean UI, lets you configure the number of simulated microservices and the total volume of logs to produce, and then generates structured, varied, realistic log files at maximum speed using Rust's concurrency primitives.

---

## Features

- **Web-based UI** — Dark-themed, responsive dashboard served at `http://localhost:3000`. No external frontend dependencies.
- **Configurable microservices** — Simulate 1 to 1,000 independent microservices, each writing to its own directory.
- **Configurable volume** — Generate anywhere from 1 MB to 1 TB+ of log data.
- **Realistic service names** — The first 30 services get real-world names (`auth-service`, `payment-service`, `order-service`, etc.). Beyond 30, services are named `microservice-N`.
- **Enterprise log format** — ISO 8601 timestamps, structured key-value fields, trace IDs, span IDs, thread identifiers.
- **Weighted log levels** — INFO, WARN, and ERROR at a configurable 500:10:3 ratio, matching real-world production distributions.
- **Massive message variety** — 10+ log categories with randomized fields drawn from large pools of realistic values. Over 100 unique message templates combined with randomized metadata yield virtually no repeated log lines.
- **Size-based file rotation** — When a log file reaches the configured maximum size, it is archived with a timestamp and a new file is started — mirroring how enterprise logging frameworks (Log4j, Logback, etc.) handle rotation.
- **Real-time progress tracking** — Live progress bar, bytes written, target size, per-service completion, and status indicators.
- **Cancellation support** — Stop generation at any time via the UI.
- **High performance** — Each microservice runs on its own OS thread via `tokio::task::spawn_blocking`, with buffered I/O for maximum throughput.
- **Single binary** — The HTML UI is embedded at compile time. No static files to deploy.

---

## Getting Started

### Prerequisites

- **Rust** 1.75 or later — [Install Rust](https://rustup.rs/)

### Build & Run

```bash
# Clone the repository
git clone <repo-url>
cd log-generator

# Build and run (debug mode)
cargo run

# Or build an optimized release binary
cargo build --release
./target/release/log-generator      # Linux/macOS
.\target\release\log-generator.exe  # Windows
```

On startup you'll see:

```
╔══════════════════════════════════════════╗
║       Log Generator is running!          ║
║  Open http://localhost:3000 in browser    ║
╚══════════════════════════════════════════╝
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

---

## Usage

### Web UI

The web interface provides three configuration fields:

| Field | Description | Default | Range |
|-------|-------------|---------|-------|
| **Number of Microservices** | How many independent services to simulate | 10 | 1–1,000 |
| **Total Log Volume (MB)** | Total size of all generated log data combined | 100 | 1–1,048,576 (1 TB) |
| **Max File Size Before Rotation (MB)** | Maximum size of a single log file before it's archived and a new one is created | 100 | 1–10,240 (10 GB) |

**Controls:**
- **▶ Start Generation** — Begins generating logs. The button is disabled while generation is active.
- **■ Stop** — Sends a cancellation signal. All service writers will stop at the next log line.

**Progress Panel:**
- Live progress bar with percentage
- Bytes written vs. target
- Services completed vs. total
- Status badge: `IDLE` → `RUNNING` → `COMPLETE` (or `STOPPED` if cancelled)

### API Endpoints

The application also exposes a REST API for programmatic control:

#### `POST /api/start`

Start log generation.

**Request Body:**
```json
{
  "num_services": 10,
  "total_size_mb": 1024,
  "file_max_size_mb": 100
}
```

**Response (200):**
```json
{
  "message": "Started generating 1024 MB of logs across 10 services"
}
```

**Error Responses:**
- `400` — Invalid parameters (zero services, zero size, etc.)
- `409` — Generation is already running

#### `POST /api/stop`

Stop the current generation run.

**Response (200):**
```json
{
  "message": "Stop signal sent. Generation will halt shortly."
}
```

#### `GET /api/progress`

Get current generation progress.

**Response (200):**
```json
{
  "running": true,
  "bytes_written": 536870912,
  "target_bytes": 1073741824,
  "percent": 50.0,
  "services_total": 10,
  "services_done": 3
}
```

---

## Log Format

Every log line follows a consistent structured format:

```
{timestamp} [{level}] [{service}] [trace_id={trace_id}] [span_id={span_id}] [thread={thread}] {message_template} | {structured_details}
```

**Example log lines:**

```
2026-02-19T03:37:04.611Z [INFO] [auth-service] [trace_id=b4acabb0d3a4f9a54a296e46dba4dec5] [span_id=0a06039c63823ba1] [thread=worker-82] Request completed successfully | method=GET path=/api/v1/users status=200 latency_ms=42 client_ip=192.168.1.105 user_agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" response_bytes=15320

2026-02-19T03:37:04.612Z [WARN] [order-service] [trace_id=c009c253e79abd0bb188ff5ba94520de] [span_id=0e83a4602975ea8e] [thread=worker-13] Slow query detected | operation=VACUUM table=transactions query_time_ms=1404 threshold_ms=500 rows_scanned=884336 missing_index=true

2026-02-19T03:37:04.611Z [ERROR] [user-service] [trace_id=0fc0d3c06d7369e637520b72e7de0963] [span_id=bb458d8b05680aef] [thread=worker-97] Connection refused by upstream | error_type=ValidationException user_id=447531ef-2928-496e-957d-3d6b59f52884 client_ip=145.165.5.123 failure_count=7 account_locked=false
```

**Fields:**
| Field | Description |
|-------|-------------|
| `timestamp` | ISO 8601 UTC with millisecond precision |
| `level` | `INFO`, `WARN`, or `ERROR` |
| `service` | Name of the simulated microservice |
| `trace_id` | 32-character hex distributed trace ID |
| `span_id` | 16-character hex span ID |
| `thread` | Simulated worker thread name (`worker-1` through `worker-127`) |
| `message_template` | Human-readable event description |
| `structured_details` | Key-value pairs with contextual metadata |

---

## Log Level Distribution

Log levels are weighted to match realistic production ratios:

| Level | Weight | Approximate % | Description |
|-------|--------|---------------|-------------|
| `INFO` | 500 | ~97.5% | Normal operational messages |
| `WARN` | 10 | ~1.9% | Degradation warnings, threshold alerts |
| `ERROR` | 3 | ~0.6% | Failures, exceptions, critical issues |

This means for every ~513 log lines, you'll see roughly 500 INFO, 10 WARN, and 3 ERROR entries.

---

## Message Variety

Log messages are generated from **10 distinct categories**, each with multiple templates and randomized field values:

| Category | Example Message | Randomized Fields |
|----------|----------------|-------------------|
| **HTTP Requests** | `Request completed successfully` | Method, path (45+ endpoints), status code, latency, client IP, user agent, response bytes |
| **Database Queries** | `Database query executed` | Operation (SELECT, INSERT, etc.), table (30+ tables), rows affected, query time |
| **Cache Operations** | `Cache operation completed` | Cache key (16+ keys), hit/miss, TTL, size, region |
| **Queue Processing** | `Message published to queue` | Queue name (16+ queues), depth, consumer lag, partition, message size |
| **External Service Calls** | `External service call succeeded` | Service name (25+ services like Stripe, AWS S3, Redis), latency, status, circuit breaker state |
| **Health Checks** | `Health check passed` | Uptime, CPU %, memory %, GC pause, active threads, open connections |
| **Auth Events** | `User authentication successful` | User UUID, action (login, logout, MFA, etc.), client IP, session duration, auth provider |
| **Batch Jobs** | `Batch job completed` | Job UUID, items processed, duration, success rate, next run time |
| **Feature Flags** | `Feature flag evaluated` | Flag name, enabled/disabled, variant, user segment |
| **Metrics** | `Metrics flushed to collector` | Metrics count, flush duration, dropped count, destination, batch size |

**WARN messages** include: slow queries, high memory utilization, connection pool saturation, rate limit warnings, certificate expiration, queue depth alerts, disk space warnings, and more.

**ERROR messages** include: connection failures, circuit breaker trips, retry exhaustion, authentication failures, deadlocks, timeout errors, OOM events, and data integrity failures. ~40% of ERROR messages include **Java-style stack traces** for added realism.

The combination of templates, random field values, UUIDs, IP addresses, and timestamps ensures that **log lines are virtually never repeated**.

---

## File Rotation

The application implements enterprise-style size-based log file rotation:

1. Each microservice starts writing to a file named with the current timestamp:
   ```
   2026-02-19_03-42-06.log
   ```

2. When the file reaches the configured maximum size, it is:
   - Flushed and closed
   - Renamed to an archived name with an incrementing index:
     ```
     2026-02-19_03-42-06_0001.log
     2026-02-19_03-42-07_0002.log
     2026-02-19_03-42-07_0003.log
     ```
   - A new active log file is created

3. The active (current) file always has the most recent timestamp without an index suffix.

This mirrors how frameworks like Log4j's `RollingFileAppender`, Logback's `SizeBasedTriggeringPolicy`, and Python's `RotatingFileHandler` work in production.

---

## Directory Structure

Generated logs are organized by microservice under a `logs/` directory:

```
logs/
├── auth-service/
│   ├── 2026-02-19_03-42-06_0001.log      # Archived (rotated)
│   ├── 2026-02-19_03-42-06_0002.log      # Archived (rotated)
│   ├── 2026-02-19_03-42-07_0003.log      # Archived (rotated)
│   └── 2026-02-19_03-42-08.104094300.log  # Active (current)
├── user-service/
│   ├── 2026-02-19_03-42-06_0001.log
│   ├── ...
│   └── 2026-02-19_03-42-08.039541600.log
├── order-service/
│   └── ...
├── payment-service/
│   └── ...
├── inventory-service/
│   └── ...
└── ... (one directory per configured service)
```

This structure makes it straightforward to point a log shipper (Filebeat, Fluentd, Vector, etc.) at the `logs/` directory and have it discover and ingest logs per service.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   Browser                        │
│              http://localhost:3000                │
└──────────────────┬──────────────────────────────┘
                   │  HTTP
┌──────────────────▼──────────────────────────────┐
│              Axum Web Server                     │
│                (server.rs)                       │
│  GET /           → Embedded HTML UI              │
│  POST /api/start → Validate & spawn generators   │
│  POST /api/stop  → Set cancel flag               │
│  GET /api/progress → Return atomic counters      │
└──────────────────┬──────────────────────────────┘
                   │  Arc<GeneratorState>
┌──────────────────▼──────────────────────────────┐
│           Generator Engine                       │
│             (generator.rs)                       │
│  Spawns N tokio::spawn_blocking tasks            │
│  Each task:                                      │
│    1. Creates RotatingWriter for its service dir  │
│    2. Loops generating random log lines          │
│    3. Updates shared AtomicU64 byte counter      │
│    4. Stops when target reached or cancelled     │
└───────┬─────────┬─────────┬─────────────────────┘
        │         │         │   (one thread per service)
   ┌────▼──┐ ┌───▼───┐ ┌───▼───┐
   │ auth  │ │ user  │ │ order │  ...
   │service│ │service│ │service│
   └───┬───┘ └───┬───┘ └───┬───┘
       │         │         │
  ┌────▼──┐ ┌───▼───┐ ┌───▼───┐
  │Rotating│ │Rotating│ │Rotating│
  │Writer  │ │Writer  │ │Writer  │
  │(rot.rs)│ │(rot.rs)│ │(rot.rs)│
  └───┬────┘ └───┬───┘ └───┬───┘
      │          │         │
      ▼          ▼         ▼
   logs/      logs/     logs/
   auth-svc/  user-svc/ order-svc/
```

### Module Breakdown

| Module | File | Responsibility |
|--------|------|----------------|
| **main** | `src/main.rs` | Entry point. Initializes shared state, starts Axum server on port 3000. |
| **server** | `src/server.rs` | HTTP routing, request validation, JSON serialization, embedded HTML serving. |
| **generator** | `src/generator.rs` | Core engine. Manages shared atomic state, spawns per-service worker threads, coordinates completion. |
| **messages** | `src/messages.rs` | Log message generation. 580 lines of message pools, templates, and randomization logic across 10 categories. |
| **rotation** | `src/rotation.rs` | Size-based file rotation with buffered I/O. Handles file creation, archival naming, and periodic flushing. |
| **UI** | `src/index.html` | Self-contained HTML/CSS/JS dashboard. Embedded into the binary via `include_str!`. |

---

## Configuration Limits

| Parameter | Min | Max | Notes |
|-----------|-----|-----|-------|
| Number of services | 1 | 1,000 | Each service gets its own OS thread |
| Total volume | 1 MB | 1,048,576 MB (1 TB) | Limited by available disk space |
| File rotation size | 1 MB | 10,240 MB (10 GB) | Typical production value: 50–200 MB |

---

## Performance

The application is designed for throughput:

- **Buffered I/O** — Each `RotatingWriter` uses a 64 KB `BufWriter` to minimize system calls.
- **Periodic flushing** — Buffers are flushed every ~256 KB to balance throughput and data safety.
- **Lock-free progress tracking** — All shared counters use `AtomicU64` / `AtomicBool` with relaxed ordering for progress and sequential consistency only where needed.
- **SmallRng** — Uses `rand::rngs::SmallRng` for fast, non-cryptographic random number generation.
- **Parallel generation** — All services generate logs concurrently on separate OS threads.

Typical throughput on modern hardware (NVMe SSD): **500 MB–2 GB per second** depending on the number of services and disk speed.

---

## Tech Stack

| Crate | Version | Purpose |
|-------|---------|---------|
| [tokio](https://crates.io/crates/tokio) | 1.x | Async runtime, task spawning |
| [axum](https://crates.io/crates/axum) | 0.7 | HTTP server and routing |
| [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) | 1.x | JSON serialization/deserialization |
| [rand](https://crates.io/crates/rand) | 0.8 | Random number generation |
| [uuid](https://crates.io/crates/uuid) | 1.x | UUID v4 generation for trace/request IDs |
| [chrono](https://crates.io/crates/chrono) | 0.4 | Timestamp formatting |
| [tokio-util](https://crates.io/crates/tokio-util) | 0.7 | Utility types for tokio |

---

## License

MIT

