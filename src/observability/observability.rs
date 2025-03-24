use {
  opentelemetry::{global, propagation::TextMapCompositePropagator},
  opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge,
  opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter, TonicExporterBuilder},
  opentelemetry_sdk::{
    logs::SdkLoggerProvider,
    metrics::SdkMeterProvider,
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::{Sampler, SdkTracerProvider},
    Resource,
  },
  tracing::{debug, level_filters::LevelFilter},
  tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter},
};

pub fn setupObservability(
  debugModeEnabled: bool,
  otelResource: &Resource,
) -> (SdkLoggerProvider, SdkMeterProvider, SdkTracerProvider) {
  let spanAndEventFilterLevel = match debugModeEnabled {
    true => LevelFilter::DEBUG,
    _ => LevelFilter::INFO,
  };

  let spanAndEventFilter = EnvFilter::builder()
    .with_default_directive(spanAndEventFilterLevel.into())
    .from_env()
    .expect("Failed building span and event filter");

  /*
    We will filter out spans and events generated from OpenTelemetry and its dependent crates
    (opentelemetry-otlp uses crates like reqwest/tonic etc.) from being sent back to OpenTelemetry
    Collector, thus preventing infinite telemetry generation.

    NOTE : This will also drop events from crates like tonic etc. even when they are used outside
           the OTLP Exporter.
           For more details, see: https://github.com/open-telemetry/opentelemetry-rust/issues/761.
  */
  spanAndEventFilter
    .add_directive("hyper=off".parse().unwrap())
    .add_directive("opentelemetry=off".parse().unwrap())
    .add_directive("tonic=off".parse().unwrap())
    .add_directive("h2=off".parse().unwrap())
    .add_directive("reqwest=off".parse().unwrap());

  let logExporter = setupLogExporter(otelResource);

  let otelLayer = OpenTelemetryTracingBridge::new(&logExporter);

  tracing_subscriber::registry().with(otelLayer).init();

  (
    logExporter,
    setupMetricExporter(otelResource),
    setupTraceExporter(otelResource),
  )
}

pub fn setupLogExporter(otelResource: &Resource) -> SdkLoggerProvider {
  let logExporter = LogExporter::builder()
    .with_tonic()
    .build()
    .expect("Failed building log exporter");

  let loggerProvider = SdkLoggerProvider::builder()
    .with_resource(otelResource)
    .with_batch_exporter(logExporter)
    .build();

  debug!("Set global logger provider");

  loggerProvider
}

pub fn setupMetricExporter(otelResource: &Resource) -> SdkMeterProvider {
  let metricExporter = MetricExporter::builder()
    .with_tonic()
    .build()
    .expect("Failed building metric exporter");

  let meterProvider = SdkMeterProvider::builder()
    .with_resource(otelResource)
    .with_periodic_exporter(metricExporter)
    .build();

  global::set_meter_provider(meterProvider);

  debug!("Set global meter provider");

  meterProvider
}

pub fn setupTraceExporter(otelResource: &Resource) -> SdkTracerProvider {
  let spanExporter = SpanExporter::builder()
    .with_tonic()
    .build()
    .expect("Failed building span exporter");

  let tracerProvider = SdkTracerProvider::builder()
    .with_resource(otelResource)
    .with_batch_exporter(spanExporter)
    .with_sampler(Sampler::AlwaysOn)
    .build();

  global::set_tracer_provider(tracerProvider.clone());

  // Text map propagators are responsible for extracting and injecting trace context information
  // into carrier objects, such as HTTP headers or other transport-specific metadata.
  global::set_text_map_propagator(TextMapCompositePropagator::new(&[
    // Trace context is a standardized format for representing trace and span information. It
    // includes trace-id, span-id, trace state etc.
    TraceContextPropagator::new(),
    // Baggage is a mechanism for carrying key-value pairs along with the trace context. These
    // key-value pairs are known as baggage items. Baggage allows you to attach custom data to a
    // request, which will be propagated along with the trace context.
    BaggagePropagator::new(),
  ]));

  debug!("Set global tracer provider");

  tracerProvider
}
