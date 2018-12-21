package com.accenture.examples.circuit.breaker;

import com.netflix.hystrix.HystrixCircuitBreaker;
import com.netflix.hystrix.HystrixCommandKey;
import com.netflix.hystrix.HystrixCommandMetrics;
import com.netflix.hystrix.HystrixEventType;

import java.util.HashMap;
import java.util.Map;

/**
 * For production, you should setup hystrix-servo-metrics-publisher v1.5.12 that polls and sends
 * metrics to Graphite for visualization. Please refer to https://github.com/jewzaam/hystrixexample
 *
 * In case you want to capture the metrics yourself, Hystrix maintains a rolling memory map for metrics.
 *
 *         for (HystrixCommandMetrics metrics : HystrixCommandMetrics.getInstances()) {
 *             try {
 *                 log.info("metrics ---- {}", SimpleMapper.getInstance().getMapper().writeValueAsString(getMetrics(metrics)));
 *             } catch (JsonProcessingException e) {
 *                 // ok to ignore
 *             }
 *         }
 *
 *         for (HystrixThreadPoolMetrics metrics : HystrixThreadPoolMetrics.getInstances()) {
 *             log.info("thread-pool ---- {}", metrics);
 *         }
 *
 *         for (HystrixCollapserMetrics metrics : HystrixCollapserMetrics.getInstances()) {
 *             log.info("collapser ---- {}", metrics);
 *         }
 *
 */
public class CircuitBreakerUtil {

    public static Map<String, Object> getMetrics(HystrixCommandMetrics metrics) {
        HystrixCommandKey key = metrics.getCommandKey();
        HystrixCircuitBreaker circuitBreaker = HystrixCircuitBreaker.Factory.getInstance(key);
        Map<String, Object> result = new HashMap<>();
        result.put("type", "HystrixCommand");
        result.put("name", key.name());
        result.put("group", metrics.getCommandGroup().name());
        result.put("currentTime", System.currentTimeMillis());
        // circuit breaker
        if (circuitBreaker == null) {
            // circuit breaker is disabled and thus never open
            result.put("isCircuitBreakerOpen", false);
        } else {
            result.put("isCircuitBreakerOpen", circuitBreaker.isOpen());
        }
        HystrixCommandMetrics.HealthCounts healthCounts = metrics.getHealthCounts();
        result.put("errorPercentage", healthCounts.getErrorPercentage());
        result.put("errorCount", healthCounts.getErrorCount());
        result.put("requestCount", healthCounts.getTotalRequests());
        saveNumberField(result, metrics, "rollingCountBadRequests", HystrixEventType.BAD_REQUEST);
        saveNumberField(result, metrics, "rollingCountCollapsedRequests", HystrixEventType.COLLAPSED);
        saveNumberField(result, metrics, "rollingCountEmit", HystrixEventType.EMIT);
        saveNumberField(result, metrics, "rollingCountExceptionsThrown", HystrixEventType.EXCEPTION_THROWN);
        saveNumberField(result, metrics, "rollingCountFailure", HystrixEventType.FAILURE);
        saveNumberField(result, metrics, "rollingCountFallbackEmit", HystrixEventType.FALLBACK_EMIT);
        saveNumberField(result, metrics, "rollingCountFallbackFailure", HystrixEventType.FALLBACK_FAILURE);
        saveNumberField(result, metrics, "rollingCountFallbackMissing", HystrixEventType.FALLBACK_MISSING);
        saveNumberField(result, metrics, "rollingCountFallbackRejection", HystrixEventType.FALLBACK_REJECTION);
        saveNumberField(result, metrics, "rollingCountFallbackSuccess", HystrixEventType.FALLBACK_SUCCESS);
        saveNumberField(result, metrics, "rollingCountResponsesFromCache", HystrixEventType.RESPONSE_FROM_CACHE);
        saveNumberField(result, metrics, "rollingCountSemaphoreRejected", HystrixEventType.SEMAPHORE_REJECTED);
        saveNumberField(result, metrics, "rollingCountShortCircuited", HystrixEventType.SHORT_CIRCUITED);
        saveNumberField(result, metrics, "rollingCountSuccess", HystrixEventType.SUCCESS);
        saveNumberField(result, metrics, "rollingCountThreadPoolRejected", HystrixEventType.THREAD_POOL_REJECTED);
        saveNumberField(result, metrics, "rollingCountTimeout", HystrixEventType.TIMEOUT);
        result.put("currentConcurrentExecutionCount", metrics.getCurrentConcurrentExecutionCount());
        result.put("rollingMaxConcurrentExecutionCount", metrics.getRollingMaxConcurrentExecutions());
        result.put("latencyExecuteMean", metrics.getExecutionTimeMean());
        Map<String, Object> latencyExecute = new HashMap<>();
        latencyExecute.put("0", metrics.getExecutionTimePercentile(0));
        latencyExecute.put("25", metrics.getExecutionTimePercentile(25));
        latencyExecute.put("50", metrics.getExecutionTimePercentile(50));
        latencyExecute.put("75", metrics.getExecutionTimePercentile(75));
        latencyExecute.put("90", metrics.getExecutionTimePercentile(90));
        latencyExecute.put("95", metrics.getExecutionTimePercentile(95));
        latencyExecute.put("99", metrics.getExecutionTimePercentile(99));
        latencyExecute.put("99.5", metrics.getExecutionTimePercentile(99.5));
        latencyExecute.put("100", metrics.getExecutionTimePercentile(100));
        result.put("latencyExecute", latencyExecute);
        return result;
    }

    private static void saveNumberField(Map<String, Object> map, HystrixCommandMetrics metrics, String name, HystrixEventType type) {
        try {
            map.put(name, metrics.getRollingCount(type));
        } catch (NoSuchFieldError error) {
            map.put(name, 0L);
        }
    }

}
