load('ext://helm_resource', 'helm_resource', 'helm_repo')

k8s_yaml(helm('helm'))
k8s_resource('axum-demo')
docker_build('axum-demo', '.')

k8s_resource(
  workload='axum-demo',
  port_forwards=8080
)

helm_repo('open-telemetry', 'https://open-telemetry.github.io/opentelemetry-helm-charts')
helm_resource('otlp', 'open-telemetry/opentelemetry-collector', flags = [
    '--set', 'standaloneCollector.enabled=true',
    '--set', 'agentCollector.enabled=false',
    '--set', 'config.exporters.jaeger.endpoint=jaeger-collector:14250',
    '--set', 'config.exporters.jaeger.tls.insecure=true',
    '--set', 'config.service.pipelines.traces.receivers[0]=otlp',
    '--set', 'config.service.pipelines.traces.exporters[0]=jaeger',
])

helm_repo('jaegertracing', 'https://jaegertracing.github.io/helm-charts')
helm_resource('jaeger', 'jaegertracing/jaeger', flags = [
    '--set', 'provisionDataStore.cassandra=false',
    '--set', 'allInOne.enabled=true',
    '--set', 'storage.type=none',
    '--set', 'agent.enabled=false',
    '--set', 'collector.enabled=false',
    '--set', 'query.enabled=false',
])

k8s_resource(workload = 'jaeger', port_forwards = '16686:16686')
