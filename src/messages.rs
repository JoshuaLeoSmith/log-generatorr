use rand::Rng;

/// Log level with weighted distribution: INFO=500, WARN=10, ERROR=3
#[derive(Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn random(rng: &mut impl Rng) -> Self {
        let n: u32 = rng.gen_range(0..513);
        match n {
            0..=2 => LogLevel::Error,
            3..=12 => LogLevel::Warn,
            _ => LogLevel::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

// ---------------------------------------------------------------------------
// Message pools – wide variety of realistic enterprise log messages
// ---------------------------------------------------------------------------

const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];

const API_PATHS: &[&str] = &[
    "/api/v1/users", "/api/v1/orders", "/api/v1/products", "/api/v1/inventory",
    "/api/v1/payments", "/api/v1/shipments", "/api/v1/notifications",
    "/api/v2/users", "/api/v2/orders", "/api/v2/products",
    "/api/v2/analytics", "/api/v2/reports", "/api/v2/search",
    "/api/v3/accounts", "/api/v3/transactions", "/api/v3/webhooks",
    "/api/internal/health", "/api/internal/metrics", "/api/internal/config",
    "/api/v1/auth/login", "/api/v1/auth/logout", "/api/v1/auth/refresh",
    "/api/v1/auth/verify", "/api/v1/roles", "/api/v1/permissions",
    "/api/v1/tenants", "/api/v1/subscriptions", "/api/v1/billing",
    "/api/v1/invoices", "/api/v1/coupons", "/api/v1/discounts",
    "/api/v1/catalog", "/api/v1/categories", "/api/v1/tags",
    "/api/v1/comments", "/api/v1/reviews", "/api/v1/ratings",
    "/api/v1/feeds", "/api/v1/timelines", "/api/v1/messages",
    "/api/v1/threads", "/api/v1/attachments", "/api/v1/uploads",
    "/api/v1/downloads", "/api/v1/exports", "/api/v1/imports",
];

const STATUS_CODES_OK: &[u16] = &[200, 201, 202, 204];
const STATUS_CODES_WARN: &[u16] = &[301, 302, 304, 400, 401, 403, 404, 405, 408, 409, 429];
const STATUS_CODES_ERR: &[u16] = &[500, 502, 503, 504];

const DB_TABLES: &[&str] = &[
    "users", "orders", "products", "sessions", "payments", "audit_log",
    "inventory", "shipments", "notifications", "events", "metrics",
    "configurations", "tenants", "subscriptions", "invoices", "accounts",
    "transactions", "roles", "permissions", "tags", "categories",
    "comments", "reviews", "feeds", "messages", "attachments",
    "cache_entries", "job_queue", "dead_letter_queue", "rate_limits",
];

const DB_OPERATIONS: &[&str] = &[
    "SELECT", "INSERT", "UPDATE", "DELETE", "UPSERT", "COUNT", "AGGREGATE",
    "JOIN", "INDEX SCAN", "SEQ SCAN", "VACUUM", "ANALYZE",
];

const CACHE_KEYS: &[&str] = &[
    "user_profile", "session_token", "product_catalog", "price_matrix",
    "feature_flags", "rate_limit_counter", "geo_lookup", "config_snapshot",
    "auth_permissions", "tenant_settings", "search_results", "api_response",
    "inventory_count", "order_summary", "notification_prefs", "dashboard_data",
];

const QUEUE_NAMES: &[&str] = &[
    "order-processing", "email-notifications", "payment-webhooks",
    "inventory-sync", "analytics-events", "audit-trail", "user-onboarding",
    "report-generation", "data-export", "search-indexing",
    "image-processing", "pdf-generation", "sms-notifications",
    "push-notifications", "batch-processing", "etl-pipeline",
];

const EXTERNAL_SERVICES: &[&str] = &[
    "Stripe API", "SendGrid", "Twilio", "AWS S3", "AWS SQS", "Redis Cluster",
    "Elasticsearch", "PostgreSQL Primary", "PostgreSQL Replica", "MongoDB",
    "RabbitMQ", "Kafka Broker", "Consul", "Vault", "Datadog", "PagerDuty",
    "Slack Webhook", "GitHub API", "Google Maps API", "Auth0",
    "Cloudflare", "Fastly CDN", "New Relic", "Sentry", "LaunchDarkly",
];

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    "PostmanRuntime/7.32.3",
    "python-requests/2.31.0",
    "Go-http-client/2.0",
    "curl/8.1.2",
    "okhttp/4.12.0",
    "Apache-HttpClient/4.5.14",
    "grpc-java/1.58.0",
];

const ERROR_TYPES: &[&str] = &[
    "NullPointerException", "ConnectionTimeoutException", "OutOfMemoryError",
    "SocketException", "IOException", "SerializationException",
    "DeserializationError", "AuthenticationException", "AuthorizationException",
    "RateLimitExceededException", "CircuitBreakerOpenException",
    "RetryExhaustedException", "ValidationException", "ConflictException",
    "ResourceNotFoundException", "ServiceUnavailableException",
    "GatewayTimeoutException", "BadRequestException", "InternalServerError",
    "DatabaseConnectionException", "LockAcquisitionException",
    "OptimisticLockException", "DeadlockDetectedException",
    "MessageParsingException", "SchemaValidationException",
    "CertificateExpiredException", "SSLHandshakeException",
    "DNSResolutionException", "DiskFullException", "QuotaExceededException",
];

const STACK_FRAMES: &[&str] = &[
    "at com.enterprise.service.UserService.findById(UserService.java:142)",
    "at com.enterprise.service.OrderService.processOrder(OrderService.java:87)",
    "at com.enterprise.repository.BaseRepository.execute(BaseRepository.java:56)",
    "at com.enterprise.controller.ApiController.handleRequest(ApiController.java:203)",
    "at com.enterprise.middleware.AuthFilter.doFilter(AuthFilter.java:34)",
    "at com.enterprise.cache.CacheManager.get(CacheManager.java:91)",
    "at com.enterprise.queue.MessageConsumer.onMessage(MessageConsumer.java:67)",
    "at com.enterprise.db.ConnectionPool.getConnection(ConnectionPool.java:145)",
    "at com.enterprise.http.RetryHandler.execute(RetryHandler.java:78)",
    "at com.enterprise.serialization.JsonMapper.deserialize(JsonMapper.java:112)",
    "at com.enterprise.validation.RequestValidator.validate(RequestValidator.java:53)",
    "at com.enterprise.circuit.CircuitBreaker.call(CircuitBreaker.java:89)",
    "at org.springframework.web.servlet.DispatcherServlet.doDispatch(DispatcherServlet.java:1067)",
    "at org.apache.tomcat.util.threads.TaskThread$WrappingRunnable.run(TaskThread.java:61)",
    "at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1136)",
    "at java.base/java.lang.Thread.run(Thread.java:833)",
];

const INFO_TEMPLATES: &[&str] = &[
    "Request completed successfully",
    "Database query executed",
    "Cache operation completed",
    "Message published to queue",
    "Message consumed from queue",
    "Health check passed",
    "Configuration reloaded",
    "External service call succeeded",
    "Session validated for user",
    "Batch job started",
    "Batch job completed",
    "Scheduled task executed",
    "Connection pool stats reported",
    "Feature flag evaluated",
    "Metrics flushed to collector",
    "Graceful shutdown initiated",
    "Service instance registered with discovery",
    "SSL certificate verified",
    "Rate limit check passed",
    "Distributed lock acquired",
    "Distributed lock released",
    "Data export completed",
    "Search index updated",
    "Webhook delivered successfully",
    "Background worker processing item",
    "Tenant context initialized",
    "Circuit breaker status: CLOSED",
    "Retry attempt succeeded",
    "File upload completed",
    "PDF report generated",
    "Email notification queued",
    "Push notification sent",
    "User authentication successful",
    "Token refresh completed",
    "API key validated",
    "CORS preflight request handled",
    "Request rate within threshold",
    "GC pause recorded",
    "Thread pool utilization reported",
    "Memory usage within bounds",
];

const WARN_TEMPLATES: &[&str] = &[
    "Slow query detected",
    "High memory utilization detected",
    "Connection pool near capacity",
    "Rate limit threshold approaching",
    "Deprecated API version used",
    "Cache miss ratio elevated",
    "Retry attempt required",
    "Response time exceeded SLA threshold",
    "Certificate expiring soon",
    "Disk space below threshold",
    "Queue depth increasing",
    "External service degraded performance",
    "Request payload size unusually large",
    "Stale cache entry detected",
    "Partial failure in batch operation",
    "Circuit breaker status: HALF-OPEN",
    "Thread pool saturation warning",
    "DNS resolution slow",
    "Upstream service returned non-standard response",
    "Schema version mismatch detected",
    "Failover to secondary database triggered",
    "Log buffer near capacity",
    "Excessive connection churn detected",
    "Token expiration imminent",
    "Orphaned resource detected during cleanup",
];

const ERROR_TEMPLATES: &[&str] = &[
    "Request processing failed",
    "Database connection lost",
    "External service call failed",
    "Message processing failed",
    "Authentication failed",
    "Authorization denied",
    "Circuit breaker tripped: OPEN",
    "All retry attempts exhausted",
    "Data validation failed",
    "Unhandled exception in request handler",
    "Out of memory: heap space exhausted",
    "Deadlock detected in transaction",
    "Connection refused by upstream",
    "SSL handshake failed",
    "Corrupt message received from queue",
    "Database constraint violation",
    "Timeout waiting for distributed lock",
    "Service discovery lookup failed",
    "Configuration parsing error",
    "Critical: health check failed",
    "Disk write failed: no space left on device",
    "Fatal: unable to bind to port",
    "Cascade failure detected across services",
    "Data integrity check failed",
    "Backup process failed",
];

const ADJECTIVES: &[&str] = &[
    "primary", "secondary", "cached", "stale", "partial", "complete",
    "encrypted", "compressed", "validated", "sanitized", "normalized",
    "aggregated", "batched", "streamed", "replicated", "sharded",
];

const IP_OCTETS: std::ops::Range<u8> = 1..255;

fn random_ip(rng: &mut impl Rng) -> String {
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(IP_OCTETS),
        rng.gen_range(IP_OCTETS),
        rng.gen_range(IP_OCTETS),
        rng.gen_range(IP_OCTETS),
    )
}

fn random_latency(rng: &mut impl Rng, level: LogLevel) -> u32 {
    match level {
        LogLevel::Info => rng.gen_range(1..500),
        LogLevel::Warn => rng.gen_range(500..5000),
        LogLevel::Error => rng.gen_range(3000..30000),
    }
}

fn random_trace_id(rng: &mut impl Rng) -> String {
    // Simplified trace ID (hex)
    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
        rng.gen::<u32>(),
    )
}

fn random_span_id(rng: &mut impl Rng) -> String {
    format!("{:016x}", rng.gen::<u64>())
}

fn pick<'a>(rng: &mut impl Rng, items: &'a [&str]) -> &'a str {
    items[rng.gen_range(0..items.len())]
}

fn pick_u16(rng: &mut impl Rng, items: &[u16]) -> u16 {
    items[rng.gen_range(0..items.len())]
}

/// Generate a realistic log message for the given level
pub fn generate_message(rng: &mut impl Rng, level: LogLevel, service_name: &str) -> String {
    let trace_id = random_trace_id(rng);
    let span_id = random_span_id(rng);
    let thread_id = rng.gen_range(1..128);

    let (template, detail) = match level {
        LogLevel::Info => {
            let t = pick(rng, INFO_TEMPLATES);
            let d = generate_info_detail(rng);
            (t, d)
        }
        LogLevel::Warn => {
            let t = pick(rng, WARN_TEMPLATES);
            let d = generate_warn_detail(rng);
            (t, d)
        }
        LogLevel::Error => {
            let t = pick(rng, ERROR_TEMPLATES);
            let d = generate_error_detail(rng);
            (t, d)
        }
    };

    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");

    format!(
        "{timestamp} [{level}] [{service}] [trace_id={trace_id}] [span_id={span_id}] [thread={thread}] {template} | {detail}",
        timestamp = timestamp,
        level = level.as_str(),
        service = service_name,
        trace_id = trace_id,
        span_id = span_id,
        thread = format!("worker-{}", thread_id),
        template = template,
        detail = detail,
    )
}

fn generate_info_detail(rng: &mut impl Rng) -> String {
    let category: u32 = rng.gen_range(0..10);
    match category {
        0 => {
            // HTTP request
            let method = pick(rng, HTTP_METHODS);
            let path = pick(rng, API_PATHS);
            let status = pick_u16(rng, STATUS_CODES_OK);
            let latency = random_latency(rng, LogLevel::Info);
            let ip = random_ip(rng);
            let ua = pick(rng, USER_AGENTS);
            let bytes = rng.gen_range(50..50000);
            format!(
                "method={} path={} status={} latency_ms={} client_ip={} user_agent=\"{}\" response_bytes={}",
                method, path, status, latency, ip, ua, bytes
            )
        }
        1 => {
            // Database query
            let op = pick(rng, DB_OPERATIONS);
            let table = pick(rng, DB_TABLES);
            let rows = rng.gen_range(0..10000);
            let latency = random_latency(rng, LogLevel::Info);
            let adj = pick(rng, ADJECTIVES);
            format!(
                "operation={} table={} rows_affected={} query_time_ms={} data_type={} connection_pool_active={}",
                op, table, rows, latency, adj, rng.gen_range(1..50)
            )
        }
        2 => {
            // Cache operation
            let key = pick(rng, CACHE_KEYS);
            let hit = rng.gen_bool(0.8);
            let ttl = rng.gen_range(60..86400);
            format!(
                "cache_key={} hit={} ttl_seconds={} size_bytes={} region={}",
                key, hit, ttl, rng.gen_range(100..100000), pick(rng, &["us-east-1", "us-west-2", "eu-west-1", "ap-southeast-1"])
            )
        }
        3 => {
            // Queue operation
            let queue = pick(rng, QUEUE_NAMES);
            let depth = rng.gen_range(0..5000);
            let consumer_lag = rng.gen_range(0..100);
            format!(
                "queue={} action=publish depth={} consumer_lag={} partition={} message_size_bytes={}",
                queue, depth, consumer_lag, rng.gen_range(0..12), rng.gen_range(100..10000)
            )
        }
        4 => {
            // External service call
            let svc = pick(rng, EXTERNAL_SERVICES);
            let latency = random_latency(rng, LogLevel::Info);
            format!(
                "external_service=\"{}\" method=GET latency_ms={} status=200 retries=0 circuit_state=CLOSED",
                svc, latency
            )
        }
        5 => {
            // Health check
            let uptime = rng.gen_range(1..365 * 24 * 3600);
            let cpu = rng.gen_range(1..80);
            let mem = rng.gen_range(20..80);
            format!(
                "uptime_seconds={} cpu_usage={}% memory_usage={}% gc_pause_ms={} active_threads={} open_connections={}",
                uptime, cpu, mem, rng.gen_range(1..50), rng.gen_range(5..200), rng.gen_range(1..100)
            )
        }
        6 => {
            // User/Auth event
            let user_id = uuid::Uuid::new_v4();
            let ip = random_ip(rng);
            let actions = &["login", "logout", "token_refresh", "password_change", "mfa_verify", "api_key_rotate"];
            let action = pick(rng, actions);
            format!(
                "user_id={} action={} client_ip={} session_duration_ms={} auth_provider={}",
                user_id, action, ip, rng.gen_range(0..86400000u64), pick(rng, &["oauth2", "saml", "ldap", "local", "oidc"])
            )
        }
        7 => {
            // Batch/scheduled
            let job_id = uuid::Uuid::new_v4();
            let items = rng.gen_range(1..100000);
            format!(
                "job_id={} items_processed={} duration_ms={} success_rate={:.2}% next_run_in_seconds={}",
                job_id, items, rng.gen_range(100..300000), rng.gen_range(9500..10000) as f64 / 100.0, rng.gen_range(60..3600)
            )
        }
        8 => {
            // Feature flag
            let flags = &["dark_mode", "new_checkout_flow", "beta_search", "ai_recommendations", "v2_pricing", "graphql_gateway"];
            let flag = pick(rng, flags);
            let enabled = rng.gen_bool(0.7);
            format!(
                "feature_flag={} enabled={} variant={} user_segment={} evaluation_ms={}",
                flag, enabled, pick(rng, &["control", "treatment_a", "treatment_b"]),
                pick(rng, &["enterprise", "pro", "free", "trial", "internal"]),
                rng.gen_range(0..5)
            )
        }
        _ => {
            // Metrics flush
            let metrics_count = rng.gen_range(50..5000);
            format!(
                "metrics_flushed={} flush_duration_ms={} dropped={} destination={} batch_size={}",
                metrics_count, rng.gen_range(10..500), rng.gen_range(0..5),
                pick(rng, &["datadog", "prometheus", "graphite", "influxdb", "cloudwatch"]),
                rng.gen_range(100..1000)
            )
        }
    }
}

fn generate_warn_detail(rng: &mut impl Rng) -> String {
    let category: u32 = rng.gen_range(0..8);
    match category {
        0 => {
            let op = pick(rng, DB_OPERATIONS);
            let table = pick(rng, DB_TABLES);
            let latency = random_latency(rng, LogLevel::Warn);
            format!(
                "operation={} table={} query_time_ms={} threshold_ms=500 rows_scanned={} missing_index=true",
                op, table, latency, rng.gen_range(10000..1000000)
            )
        }
        1 => {
            let method = pick(rng, HTTP_METHODS);
            let path = pick(rng, API_PATHS);
            let status = pick_u16(rng, STATUS_CODES_WARN);
            let latency = random_latency(rng, LogLevel::Warn);
            format!(
                "method={} path={} status={} latency_ms={} client_ip={} retry_after_seconds={}",
                method, path, status, latency, random_ip(rng), rng.gen_range(1..60)
            )
        }
        2 => {
            let pool_size = rng.gen_range(50..200);
            let active = pool_size - rng.gen_range(1..5);
            format!(
                "pool_size={} active_connections={} idle={} wait_queue={} max_wait_ms={}",
                pool_size, active, pool_size - active, rng.gen_range(5..50), rng.gen_range(100..5000)
            )
        }
        3 => {
            let svc = pick(rng, EXTERNAL_SERVICES);
            let latency = random_latency(rng, LogLevel::Warn);
            format!(
                "external_service=\"{}\" latency_ms={} expected_max_ms=1000 status=200 degraded=true retry_count={}",
                svc, latency, rng.gen_range(1..3)
            )
        }
        4 => {
            let queue = pick(rng, QUEUE_NAMES);
            format!(
                "queue={} depth={} max_depth=10000 consumer_lag_seconds={} oldest_message_age_seconds={}",
                queue, rng.gen_range(5000..9500), rng.gen_range(30..300), rng.gen_range(60..600)
            )
        }
        5 => {
            format!(
                "memory_usage={}% threshold=85% heap_used_mb={} heap_max_mb={} gc_collections={} gc_time_ms={}",
                rng.gen_range(80..95), rng.gen_range(3000..7500), 8192, rng.gen_range(100..1000), rng.gen_range(500..5000)
            )
        }
        6 => {
            format!(
                "disk_usage={}% partition=/data available_gb={} inode_usage={}% oldest_file_days={}",
                rng.gen_range(80..95), rng.gen_range(5..50), rng.gen_range(60..90), rng.gen_range(30..365)
            )
        }
        _ => {
            let ip = random_ip(rng);
            format!(
                "client_ip={} requests_per_minute={} limit=1000 remaining={} window_reset_seconds={}",
                ip, rng.gen_range(800..999), rng.gen_range(1..200), rng.gen_range(10..60)
            )
        }
    }
}

fn generate_error_detail(rng: &mut impl Rng) -> String {
    let category: u32 = rng.gen_range(0..6);
    let include_stack = rng.gen_bool(0.4);

    let detail = match category {
        0 => {
            let method = pick(rng, HTTP_METHODS);
            let path = pick(rng, API_PATHS);
            let status = pick_u16(rng, STATUS_CODES_ERR);
            let err = pick(rng, ERROR_TYPES);
            format!(
                "method={} path={} status={} error_type={} latency_ms={} request_id={}",
                method, path, status, err, random_latency(rng, LogLevel::Error), uuid::Uuid::new_v4()
            )
        }
        1 => {
            let svc = pick(rng, EXTERNAL_SERVICES);
            let err = pick(rng, ERROR_TYPES);
            format!(
                "external_service=\"{}\" error_type={} retries=3 last_attempt_ms={} circuit_state=OPEN fallback_used=true",
                svc, err, random_latency(rng, LogLevel::Error)
            )
        }
        2 => {
            let table = pick(rng, DB_TABLES);
            let err = pick(rng, ERROR_TYPES);
            format!(
                "operation=WRITE table={} error_type={} connection_id={} statement_timeout_ms={} rollback=true",
                table, err, rng.gen_range(1..1000), random_latency(rng, LogLevel::Error)
            )
        }
        3 => {
            let queue = pick(rng, QUEUE_NAMES);
            let err = pick(rng, ERROR_TYPES);
            format!(
                "queue={} error_type={} message_id={} retry_count=3 dead_lettered=true original_timestamp={}",
                queue, err, uuid::Uuid::new_v4(), chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
            )
        }
        4 => {
            let err = pick(rng, ERROR_TYPES);
            let user_id = uuid::Uuid::new_v4();
            format!(
                "error_type={} user_id={} client_ip={} failure_count={} account_locked={}",
                err, user_id, random_ip(rng), rng.gen_range(3..10), rng.gen_bool(0.3)
            )
        }
        _ => {
            let err = pick(rng, ERROR_TYPES);
            format!(
                "error_type={} component={} heap_used_mb={} available_mb={} oom_killer_invoked={}",
                err, pick(rng, &["worker", "scheduler", "gateway", "processor", "aggregator"]),
                rng.gen_range(7000..8192), rng.gen_range(0..100), rng.gen_bool(0.2)
            )
        }
    };

    if include_stack {
        let num_frames = rng.gen_range(3..8);
        let mut stack = String::from("\n  Stacktrace:\n");
        for _ in 0..num_frames {
            stack.push_str("    ");
            stack.push_str(pick(rng, STACK_FRAMES));
            stack.push('\n');
        }
        format!("{}{}", detail, stack)
    } else {
        detail
    }
}

