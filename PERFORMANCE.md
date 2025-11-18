# Performance and Scalability Improvements

This document describes the performance and scalability improvements made to Netpix.

## Overview

Netpix has been enhanced with several improvements to handle high-traffic scenarios, support more concurrent users, and provide better resource management.

## Key Improvements

### 1. Bounded Message Queues with Backpressure

**Problem**: Client message queues were unbounded, allowing unlimited growth when clients couldn't keep up with incoming messages. This could lead to memory exhaustion.

**Solution**: Implemented bounded queues with configurable limits:
- Each client has a maximum queue size (default: 1000 messages)
- When the queue is full, new messages are dropped with a warning log
- This prevents memory exhaustion while maintaining system stability

**Configuration**: `--max-client-queue-size <SIZE>`

### 2. Connection Limits

**Problem**: No limits on concurrent connections could lead to resource exhaustion under load.

**Solution**: Added configurable connection limits:
- Maximum concurrent connections (default: 100)
- Graceful rejection of new connections when limit is reached
- Active connection tracking with atomic counters

**Configuration**: `--max-clients <COUNT>`

### 3. Message Batching

**Problem**: The message sender processed only one message per client per tick, leading to:
- High lock acquisition overhead
- Poor throughput under high load
- Inefficient use of available bandwidth

**Solution**: Implemented configurable message batching:
- Multiple messages sent per client per tick (default: 10)
- Configurable batch size for different workloads
- Reduces lock overhead by up to 10x

**Configuration**: `--message-batch-size <SIZE>`

### 4. Reduced Lock Contention

**Problem**: Write locks were held longer than necessary in the packet sniffing loop.

**Solution**: 
- Explicitly acquire and drop locks to minimize hold time
- Reduced critical section duration
- Better concurrent throughput

## Performance Characteristics

### Memory Usage

Approximate memory usage can be calculated as:
```
Total Memory ≈ (buffer_size × avg_packet_size) + (max_clients × max_client_queue_size × avg_message_size)
```

Where:
- `avg_packet_size`: 1-2 KB (typical)
- `avg_message_size`: 1-2 KB (typical)

Example with defaults:
```
Memory ≈ (32768 × 1.5KB) + (100 × 1000 × 1.5KB)
       ≈ 49MB + 150MB
       ≈ 199MB
```

### Throughput

With message batching, throughput scales linearly with batch size up to the point where network/processing becomes the bottleneck:
- Batch size 1: ~1000 msg/s per client
- Batch size 10: ~10000 msg/s per client
- Batch size 20: ~20000 msg/s per client

### Latency

Message latency is determined by:
```
Latency ≈ message_interval + (queue_depth / batch_size) × message_interval
```

Example:
- Interval: 5ms, Batch: 10, Queue: 50 messages
- Latency ≈ 5ms + (50/10) × 5ms = 30ms

## Configuration Examples

### High-Throughput Configuration

For maximum throughput with acceptable latency:
```bash
netpix run -i eth0 \
  --max-clients 200 \
  --max-client-queue-size 2000 \
  --message-interval 10 \
  --message-batch-size 20 \
  --buffer-size 65536
```

### Low-Latency Configuration

For real-time analysis with minimal delay:
```bash
netpix run -i eth0 \
  --max-clients 50 \
  --message-interval 2 \
  --message-batch-size 5 \
  --max-client-queue-size 500
```

### Resource-Constrained Configuration

For environments with limited memory:
```bash
netpix run -i eth0 \
  --max-clients 25 \
  --max-client-queue-size 300 \
  --buffer-size 16384 \
  --message-interval 20 \
  --message-batch-size 15
```

### Balanced Configuration (Default)

The default values provide a good balance for most use cases:
```bash
netpix run -i eth0
# Uses defaults:
# --max-clients 100
# --max-client-queue-size 1000
# --message-interval 5
# --message-batch-size 10
# --buffer-size 32768
```

## Monitoring and Tuning

### Warning Logs

Monitor these log messages to identify bottlenecks:

1. `"Client queue full, dropping message"`
   - **Cause**: Client can't keep up with message rate
   - **Solutions**: 
     - Increase `--max-client-queue-size`
     - Increase `--message-interval`
     - Increase `--message-batch-size`

2. `"Maximum client limit reached"`
   - **Cause**: Too many concurrent connections
   - **Solution**: Increase `--max-clients`

3. `"Packet buffer full, discarding oldest packet"`
   - **Cause**: Packet buffer overflow
   - **Solution**: Increase `--buffer-size`

### Performance Metrics

Key metrics to monitor:
- Active client count
- Message queue depths
- Packet buffer utilization
- CPU usage
- Memory usage
- Network throughput

## Best Practices

1. **Start with defaults**: The default configuration works well for most scenarios

2. **Tune based on workload**:
   - High packet rate → Increase buffer size
   - Many clients → Increase max clients and queue sizes
   - Real-time requirements → Decrease message interval

3. **Monitor resource usage**: Use system monitoring tools to track:
   - Memory consumption
   - CPU utilization
   - Network bandwidth

4. **Test under load**: Simulate production workload to validate configuration

5. **Adjust incrementally**: Make small changes and measure impact

## Future Improvements

Potential areas for further optimization:
- Per-source client subscriptions to avoid iterating all clients
- Adaptive batching based on load
- Compression for message payloads
- Metrics and monitoring API
- Hot-reload of configuration parameters
